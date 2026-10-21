use crate::model::{Chapter, Story};
use std::collections::HashMap;
use toasty::Db;
use unicode_normalization::{UnicodeNormalization, char::is_combining_mark};

pub const MAX_QUERY_CHARS: usize = 120;

pub fn normalize(text: &str) -> String {
    text.nfd()
        .filter(|c| !is_combining_mark(*c))
        .flat_map(char::to_lowercase)
        .map(|c| if c == 'đ' { 'd' } else { c })
        .collect()
}

pub fn genre_key(key: &str) -> &str {
    match key {
        "wuxia" | "romance" | "fantasy" => key,
        _ => "all",
    }
}

pub async fn list(db: &mut Db) -> toasty::Result<Vec<Story>> {
    Story::all()
        .order_by(Story::fields().id().asc())
        .exec(db)
        .await
}

pub struct Catalog {
    pub stories: Vec<Story>,
    pub total: usize,
    pub previews: HashMap<String, Chapter>,
    pub matched: usize,
    pub page: usize,
    pub page_count: usize,
}

pub const PAGE_SIZE: usize = 24;
const SCAN_BATCH: usize = 128;

pub async fn search(db: &mut Db, query: &str, genre: &str) -> toasty::Result<Catalog> {
    search_page(db, query, genre, 1).await
}

/// Pages are one-based; zero is treated as one. Out-of-range pages are empty.
/// `total` counts all stories; `matched` counts matches before pagination.
/// Empty normalized queries use database counts and pagination; nonempty queries
/// scan bounded batches of title/summary metadata, never chapter bodies.
pub async fn search_page(
    db: &mut Db,
    query: &str,
    genre: &str,
    page: usize,
) -> toasty::Result<Catalog> {
    let total = Story::all().count().exec(db).await? as usize;
    let query = normalize(
        &query
            .trim()
            .chars()
            .take(MAX_QUERY_CHARS)
            .collect::<String>(),
    );
    let genre = genre_key(genre);
    let page = page.max(1);
    let start = page.saturating_sub(1).saturating_mul(PAGE_SIZE);
    let mut matched = 0;
    let mut ids = Vec::new();
    if query.is_empty() {
        let mut filtered = Story::all();
        if genre != "all" {
            filtered = filtered.filter(Story::fields().genre_key().eq(genre));
            matched = Story::all()
                .filter(Story::fields().genre_key().eq(genre))
                .count()
                .exec(db)
                .await? as usize;
        } else {
            matched = total;
        }
        // Toasty converts offsets to i64 and panics if they do not fit. Skip
        // out-of-range pages before constructing an offset, including usize::MAX.
        if start < matched && i64::try_from(start).is_ok() {
            ids = filtered
                .order_by(Story::fields().id().asc())
                .select(Story::fields().id())
                .limit(PAGE_SIZE)
                .offset(start)
                .exec(db)
                .await?;
        }
    } else {
        let mut after = 0u64;
        loop {
            let mut scan = Story::all().filter(Story::fields().id().gt(after));
            if genre != "all" {
                scan = scan.filter(Story::fields().genre_key().eq(genre));
            }
            let batch = scan
                .order_by(Story::fields().id().asc())
                .select((
                    Story::fields().id(),
                    Story::fields().title(),
                    Story::fields().summary(),
                ))
                .limit(SCAN_BATCH)
                .exec(db)
                .await?;
            if batch.is_empty() {
                break;
            }
            for (id, title, summary) in batch {
                after = id;
                if normalize(&format!("{title} {summary}")).contains(&query) {
                    if matched >= start && ids.len() < PAGE_SIZE {
                        ids.push(id);
                    }
                    matched += 1;
                }
            }
        }
    }
    let mut stories = Vec::new();
    let mut previews = HashMap::new();
    if !ids.is_empty() {
        stories = Story::all()
            .filter(Story::fields().id().in_list(ids))
            .order_by(Story::fields().id().asc())
            .exec(db)
            .await?;
        if !stories.is_empty() {
            let slugs: Vec<String> = stories.iter().map(|story| story.slug.clone()).collect();
            previews = Chapter::all()
                .filter(Chapter::fields().story_slug().in_list(slugs))
                .filter(Chapter::fields().position().eq(1u64))
                .filter(Chapter::fields().published().eq(true))
                .exec(db)
                .await?
                .into_iter()
                .map(|chapter| (chapter.story_slug.clone(), chapter))
                .collect();
        }
    }
    Ok(Catalog {
        stories,
        total,
        previews,
        matched,
        page,
        page_count: matched.div_ceil(PAGE_SIZE),
    })
}

#[derive(Debug)]
pub struct ChapterLink {
    pub position: u64,
    pub title: String,
    pub available: bool,
}

pub struct Reading {
    pub story: Story,
    pub chapter: Chapter,
    pub total: usize,
    pub chapters: Vec<ChapterLink>,
}

pub async fn read(db: &mut Db, slug: &str, position: u64) -> toasty::Result<Option<Reading>> {
    let slug = if slug == "river" {
        "ben-cu-dong-ngan"
    } else {
        slug
    };
    if slug.len() > 160 || position == 0 {
        return Ok(None);
    }
    let Some(story) = Story::all()
        .filter(Story::fields().slug().eq(slug))
        .first()
        .exec(db)
        .await?
    else {
        return Ok(None);
    };
    use toasty::stmt::{Type, Value};
    // SQL redaction ensures unpublished bodies never cross the database boundary.
    let rows = toasty::sql::query("SELECT id, slug, story_slug, position, title, CASE WHEN published THEN content ELSE '' END, published FROM chapters WHERE story_slug = ?1 AND position = ?2")
        .bind(slug).bind(position)
        .column_types([Type::U64, Type::String, Type::String, Type::U64, Type::String, Type::String, Type::Bool])
        .exec(db).await?;
    let Some(row) = rows.first() else {
        return Ok(None);
    };
    let [
        Value::U64(id),
        Value::String(chapter_slug),
        Value::String(story_slug),
        Value::U64(position),
        Value::String(title),
        Value::String(content),
        Value::Bool(published),
    ] = row.as_record_unwrap().as_slice()
    else {
        return Err(toasty::Error::invalid_result("invalid chapter projection"));
    };
    let chapter = Chapter {
        id: *id,
        slug: chapter_slug.clone(),
        story_slug: story_slug.clone(),
        position: *position,
        title: title.clone(),
        content: content.clone(),
        published: *published,
    };
    let rows = toasty::sql::query("SELECT position, title, published AND length(trim(content, char(9,10,11,12,13,32,133,160,5760,8192,8193,8194,8195,8196,8197,8198,8199,8200,8201,8202,8232,8233,8239,8287,12288))) > 0 FROM chapters WHERE story_slug = ?1 ORDER BY position")
        .bind(slug).column_types([Type::U64, Type::String, Type::Bool]).exec(db).await?;
    let mut chapters = Vec::with_capacity(rows.len());
    for row in rows {
        let [
            Value::U64(position),
            Value::String(title),
            Value::Bool(available),
        ] = row.as_record_unwrap().as_slice()
        else {
            return Err(toasty::Error::invalid_result(
                "invalid chapter index projection",
            ));
        };
        chapters.push(ChapterLink {
            position: *position,
            title: title
                .strip_prefix("Khởi đầu · ")
                .unwrap_or(title)
                .to_owned(),
            available: *available,
        });
    }
    Ok(Some(Reading {
        story,
        chapter,
        total: chapters.len(),
        chapters,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn unpublished_bodies_never_reach_reader_or_previews() {
        let mut db = crate::db::connect("sqlite::memory:").await.unwrap();
        crate::seed::run(&mut db).await.unwrap();
        toasty::sql::statement("UPDATE chapters SET published = FALSE, content = 'Secret draft' WHERE slug = 'ben-cu-dong-ngan-1'")
            .exec(&mut db).await.unwrap();
        let reading = read(&mut db, "river", 1).await.unwrap().unwrap();
        assert!(!reading.chapter.published);
        assert!(reading.chapter.content.is_empty());
        assert!(!reading.chapters[0].available);
        let catalog = search(&mut db, "ben cu", "all").await.unwrap();
        assert_eq!(catalog.stories.len(), 1);
        assert!(catalog.previews.is_empty());
        let default = search(&mut db, "", "all").await.unwrap();
        assert_eq!(default.previews.len(), 2);
        assert!(!default.previews.contains_key("ben-cu-dong-ngan"));
        let persisted = Chapter::all()
            .filter(Chapter::fields().slug().eq("ben-cu-dong-ngan-1"))
            .first()
            .exec(&mut db)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(persisted.content, "Secret draft");
        toasty::sql::statement(
            "UPDATE chapters SET content = ' \n\t\u{2003}' WHERE slug = 'ben-cu-dong-ngan-2'",
        )
        .exec(&mut db)
        .await
        .unwrap();
        let reading = read(&mut db, "river", 2).await.unwrap().unwrap();
        assert!(reading.chapter.published);
        assert!(!reading.chapters[1].available);
    }

    #[tokio::test]
    async fn empty_search_pages_and_filters_in_database() {
        let mut db = crate::db::connect("sqlite::memory:").await.unwrap();
        let count = PAGE_SIZE * 2 + 3;
        for index in 0..count {
            Story::create()
                .slug(format!("story-{index}"))
                .title(format!("Đặng {index}"))
                .summary("Sương chiều")
                .genre_key(if index % 2 == 0 { "wuxia" } else { "fantasy" })
                .cover("cover")
                .exec(&mut db)
                .await
                .unwrap();
        }
        let default = search(&mut db, "", "all").await.unwrap();
        assert_eq!(default.total, count);
        assert_eq!(default.matched, count);
        assert_eq!(default.page, 1);
        assert_eq!(default.page_count, count.div_ceil(PAGE_SIZE));
        assert_eq!(default.stories.len(), PAGE_SIZE);
        assert_eq!(default.stories[0].slug, "story-0");
        assert_eq!(
            default.stories[PAGE_SIZE - 1].slug,
            format!("story-{}", PAGE_SIZE - 1)
        );

        // A combining mark normalizes to empty, even though trim alone is nonempty.
        let genre = search_page(&mut db, "\u{0301}", "wuxia", 0).await.unwrap();
        assert_eq!(genre.total, count);
        assert_eq!(genre.matched, count.div_ceil(2));
        assert_eq!(genre.page, 1);
        assert_eq!(genre.page_count, 2);
        assert_eq!(genre.stories.len(), PAGE_SIZE);
        assert!(genre.stories.iter().all(|story| story.genre_key == "wuxia"));
        assert_eq!(
            genre.stories[PAGE_SIZE - 1].slug,
            format!("story-{}", (PAGE_SIZE - 1) * 2)
        );

        let last = search_page(&mut db, " \n\t ", "wuxia", 2).await.unwrap();
        assert_eq!(last.matched, genre.matched);
        assert_eq!(last.stories.len(), genre.matched - PAGE_SIZE);
        assert_eq!(last.stories[0].slug, format!("story-{}", PAGE_SIZE * 2));
        assert_eq!(
            last.stories.last().unwrap().slug,
            format!("story-{}", count - 1)
        );
        let fallback = search(&mut db, "", "invalid").await.unwrap();
        assert_eq!(fallback.matched, count);
        let no_match = search(&mut db, "", "romance").await.unwrap();
        assert_eq!(no_match.total, count);
        assert_eq!(no_match.matched, 0);
        assert_eq!(no_match.page_count, 0);
        assert!(no_match.stories.is_empty());

        for page in [3, usize::MAX / PAGE_SIZE + 2, usize::MAX] {
            let excessive = search_page(&mut db, "", "wuxia", page).await.unwrap();
            assert_eq!(excessive.total, count);
            assert_eq!(excessive.matched, genre.matched);
            assert_eq!(excessive.page, page);
            assert_eq!(excessive.page_count, genre.page_count);
            assert!(excessive.stories.is_empty());
            assert!(excessive.previews.is_empty());
        }
        let mut empty = crate::db::connect("sqlite::memory:").await.unwrap();
        let no_stories = search_page(&mut empty, "", "all", usize::MAX)
            .await
            .unwrap();
        assert_eq!(no_stories.total, 0);
        assert_eq!(no_stories.matched, 0);
        assert_eq!(no_stories.page_count, 0);
        assert!(no_stories.stories.is_empty());
    }

    #[tokio::test]
    async fn search_pages_metadata_across_scan_batches() {
        let mut db = crate::db::connect("sqlite::memory:").await.unwrap();
        for index in 0..(SCAN_BATCH + 3) {
            Story::create()
                .slug(format!("story-{index}"))
                .title(format!("Đặng {index}"))
                .summary("Sương chiều")
                .genre_key("wuxia")
                .cover("cover")
                .exec(&mut db)
                .await
                .unwrap();
            Chapter::create()
                .slug(format!("chapter-{index}"))
                .story_slug(format!("story-{index}"))
                .position(1u64)
                .title("First")
                .content("Published prose")
                .published(true)
                .exec(&mut db)
                .await
                .unwrap();
        }
        let first = search(&mut db, "DANG", "wuxia").await.unwrap();
        assert_eq!(first.total, SCAN_BATCH + 3);
        assert_eq!(first.matched, first.total);
        assert_eq!(first.page, 1);
        assert_eq!(first.page_count, (SCAN_BATCH + 3).div_ceil(PAGE_SIZE));
        assert_eq!(first.stories.len(), PAGE_SIZE);
        assert_eq!(first.previews.len(), PAGE_SIZE);
        assert_eq!(first.stories[0].slug, "story-0");
        let next = search_page(&mut db, "suong chieu", "wuxia", 2)
            .await
            .unwrap();
        assert_eq!(next.stories[0].slug, format!("story-{PAGE_SIZE}"));
        assert!(
            next.previews
                .keys()
                .all(|slug| next.stories.iter().any(|story| &story.slug == slug))
        );
        let last = search_page(&mut db, "dang", "all", first.page_count)
            .await
            .unwrap();
        assert_eq!(
            last.stories.last().unwrap().slug,
            format!("story-{}", SCAN_BATCH + 2)
        );
        let missing = search_page(&mut db, "dang", "all", usize::MAX)
            .await
            .unwrap();
        assert_eq!(missing.matched, SCAN_BATCH + 3);
        assert!(missing.stories.is_empty());
        assert!(missing.previews.is_empty());
        let zero = search_page(&mut db, "dang", "invalid", 0).await.unwrap();
        assert_eq!(zero.page, 1);
        assert_eq!(zero.stories.len(), PAGE_SIZE);
        let no_match = search(&mut db, "dang", "fantasy").await.unwrap();
        assert_eq!(no_match.matched, 0);
        assert_eq!(no_match.page_count, 0);
        assert!(no_match.previews.is_empty());
    }

    #[test]
    fn vietnamese_search_normalizes_diacritics_and_case() {
        assert_eq!(normalize("ĐÈN sương"), "den suong");
        assert_eq!(normalize("Đặng"), normalize("Đa\u{0323}̆ng"));
    }

    #[tokio::test]
    async fn legacy_river_story_is_renamed_and_completed() {
        let mut db = crate::db::connect("sqlite::memory:").await.unwrap();
        Story::create()
            .slug("river")
            .title("Bến cũ trăng ngàn")
            .genre_key("wuxia")
            .cover("mountain")
            .summary("Old summary")
            .exec(&mut db)
            .await
            .unwrap();
        Chapter::create()
            .slug("river-1")
            .story_slug("river")
            .position(1u64)
            .title("Khởi đầu · Người về bên sông")
            .content("Old preview")
            .exec(&mut db)
            .await
            .unwrap();

        crate::seed::run(&mut db).await.unwrap();
        let reading = read(&mut db, "river", 1).await.unwrap().unwrap();
        assert_eq!(reading.story.slug, "ben-cu-dong-ngan");
        assert_eq!(reading.story.title, "Bến Cũ Đông Ngàn");
        assert_eq!(reading.chapter.title, "Chén trà còn ấm");
        assert_eq!(reading.total, 13);
        assert_eq!(reading.chapters[12].title, "Bến cũ, trời Đông Ngàn");
    }

    #[tokio::test]
    async fn catalog_and_reader_use_persisted_data_without_fixtures() {
        let mut db = crate::db::connect("sqlite::memory:").await.unwrap();
        assert!(search(&mut db, "", "all").await.unwrap().stories.is_empty());
        crate::seed::run(&mut db).await.unwrap();
        assert_eq!(crate::seed::run(&mut db).await.unwrap(), 0);
        assert_eq!(
            search(&mut db, "BEN CU", "wuxia")
                .await
                .unwrap()
                .stories
                .len(),
            1
        );
        assert!(
            search(&mut db, "ben cu", "fantasy")
                .await
                .unwrap()
                .stories
                .is_empty()
        );
        assert!(
            search(&mut db, "' OR 1=1 --", "all")
                .await
                .unwrap()
                .stories
                .is_empty()
        );
        let reading = read(&mut db, "river", 1).await.unwrap().unwrap();
        assert!(reading.chapter.content.contains("Mười năm"));
        assert!(read(&mut db, "missing", 1).await.unwrap().is_none());
        assert!(read(&mut db, "river", 0).await.unwrap().is_none());
        assert_eq!(Chapter::all().exec(&mut db).await.unwrap().len(), 15);
        assert_eq!(reading.story.title, "Bến Cũ Đông Ngàn");
        assert_eq!(reading.total, 13);
        assert_eq!(reading.chapters[12].title, "Bến cũ, trời Đông Ngàn");
        assert_eq!(
            reading
                .chapters
                .iter()
                .filter(|chapter| chapter.available)
                .count(),
            13
        );
        assert!(
            !read(&mut db, "river", 13)
                .await
                .unwrap()
                .unwrap()
                .chapter
                .content
                .is_empty()
        );
        assert!(read(&mut db, "river", 14).await.unwrap().is_none());
        for position in 1..=13 {
            let chapter = read(&mut db, "river", position)
                .await
                .unwrap()
                .unwrap()
                .chapter;
            assert!(
                chapter.content.chars().count() > 2500,
                "Chapter {position} must contain full prose"
            );
            assert!(
                chapter.content.split("\n\n").count() >= 20,
                "Chapter {position} must have complete scenes"
            );
        }

        toasty::sql::statement("UPDATE stories SET summary = ?1 WHERE slug = 'ben-cu-dong-ngan'")
            .bind("An author-edited description")
            .exec(&mut db)
            .await
            .unwrap();
        toasty::sql::statement(
            "UPDATE chapters SET content = ?1 WHERE slug = 'ben-cu-dong-ngan-1'",
        )
        .bind("An author-edited chapter")
        .exec(&mut db)
        .await
        .unwrap();
        crate::seed::run(&mut db).await.unwrap();
        let preserved = read(&mut db, "river", 1).await.unwrap().unwrap();
        assert_eq!(preserved.story.summary, "An author-edited description");
        assert_eq!(preserved.chapter.content, "An author-edited chapter");
    }

    #[tokio::test]
    async fn seed_completes_drafts_and_missing_chapters_but_preserves_author_edits() {
        let mut db = crate::db::connect("sqlite::memory:").await.unwrap();
        crate::seed::run(&mut db).await.unwrap();
        let original = read(&mut db, "river", 1)
            .await
            .unwrap()
            .unwrap()
            .chapter
            .content;
        toasty::sql::statement(
            "UPDATE chapters SET content = ?1 WHERE story_slug = ?2 AND position = 2",
        )
        .bind(" \n\t ")
        .bind("ben-cu-dong-ngan")
        .exec(&mut db)
        .await
        .unwrap();
        toasty::sql::statement(
            "UPDATE chapters SET content = ?1 WHERE story_slug = ?2 AND position = 3",
        )
        .bind("An author-edited third chapter")
        .bind("ben-cu-dong-ngan")
        .exec(&mut db)
        .await
        .unwrap();
        toasty::sql::statement("DELETE FROM chapters WHERE story_slug = ?1 AND position = 13")
            .bind("ben-cu-dong-ngan")
            .exec(&mut db)
            .await
            .unwrap();
        for _ in 0..2 {
            assert_eq!(crate::seed::run(&mut db).await.unwrap(), 0);
        }
        assert_eq!(
            read(&mut db, "river", 1)
                .await
                .unwrap()
                .unwrap()
                .chapter
                .content,
            original
        );
        assert!(
            read(&mut db, "river", 2)
                .await
                .unwrap()
                .unwrap()
                .chapter
                .content
                .chars()
                .count()
                > 2500
        );
        assert_eq!(
            read(&mut db, "river", 3)
                .await
                .unwrap()
                .unwrap()
                .chapter
                .content,
            "An author-edited third chapter"
        );
        let last = read(&mut db, "river", 13).await.unwrap().unwrap();
        assert!(last.chapter.content.ends_with("Chàng đã về nhà."));
        assert_eq!(last.chapters.len(), 13);
        assert!(last.chapters.iter().all(|chapter| chapter.available));
        assert_eq!(Chapter::all().exec(&mut db).await.unwrap().len(), 15);
    }
}
