mod data;
mod dong_ngan;

use self::data::DEMO_STORIES;
use crate::model::{Chapter, Story};
use toasty::Db;

pub async fn run(db: &mut Db) -> toasty::Result<usize> {
    upgrade_dong_ngan(db).await?;
    let mut inserted = 0;
    for sample in DEMO_STORIES {
        let existing = Story::all()
            .filter(Story::fields().slug().eq(sample.slug))
            .first()
            .exec(db)
            .await?;

        if existing.is_none() {
            Story::create()
                .slug(sample.slug)
                .title(sample.title)
                .genre_key(sample.genre_key)
                .cover(sample.cover)
                .summary(sample.summary)
                .exec(db)
                .await?;

            inserted += 1;
        }

        // Upgrade only the original demo copy; preserve summaries edited by the author.
        toasty::sql::statement("UPDATE stories SET summary = ?1 WHERE slug = ?2 AND summary = ?3")
            .bind(sample.summary)
            .bind(sample.slug)
            .bind(sample.legacy_summary)
            .exec(db)
            .await?;

        if sample.slug == dong_ngan::SLUG {
            continue;
        }

        let chapter_slug = format!("{}-1", sample.slug);
        let chapter = Chapter::all()
            .filter(Chapter::fields().slug().eq(&chapter_slug))
            .first()
            .exec(db)
            .await?;
        if chapter.is_none() {
            Chapter::create()
                .slug(chapter_slug)
                .story_slug(sample.slug)
                .position(1u64)
                .title(sample.chapter)
                .content(sample.excerpt.join("\n\n"))
                .published(true)
                .exec(db)
                .await?;
        }
    }

    seed_dong_ngan(db).await?;
    Ok(inserted)
}

// Rename the existing demo in place so its id and chapter references survive.
async fn upgrade_dong_ngan(db: &mut Db) -> toasty::Result<()> {
    let legacy = Story::all()
        .filter(Story::fields().slug().eq("river"))
        .first()
        .exec(db)
        .await?;
    let current = Story::all()
        .filter(Story::fields().slug().eq(dong_ngan::SLUG))
        .first()
        .exec(db)
        .await?;
    if legacy.is_some() && current.is_none() {
        let mut tx = db.transaction().await?;
        toasty::sql::statement("UPDATE stories SET slug = ?1, title = ?2 WHERE slug = 'river'")
            .bind(dong_ngan::SLUG)
            .bind(dong_ngan::TITLE)
            .exec(&mut tx)
            .await?;
        toasty::sql::statement("UPDATE chapters SET story_slug = ?1, slug = ?1 || '-' || position WHERE story_slug = 'river'")
            .bind(dong_ngan::SLUG).exec(&mut tx).await?;
        tx.commit().await?;
    }
    Ok(())
}

async fn seed_dong_ngan(db: &mut Db) -> toasty::Result<()> {
    let mut tx = db.transaction().await?;
    let story = &DEMO_STORIES[0];
    // Replace only the known demo text; subsequent author edits are kept.
    toasty::sql::statement("UPDATE stories SET summary = ?1 WHERE slug = ?2 AND summary = ?3")
        .bind(story.summary).bind(dong_ngan::SLUG)
        .bind("Một người trở lại bến sông sau mười năm. Chuyến đò năm ấy đã đi, chỉ lời hẹn xưa còn ở lại. Giữa chén trà còn ấm và ngọn đèn bên kia bờ, chàng phải chọn tiếp tục cuộc giang hồ hay bước qua điều mình đã bỏ quên.")
        .exec(&mut tx).await?;
    // Fix the old demo place name without touching an author-written title.
    toasty::sql::statement("UPDATE stories SET title = ?1 WHERE slug = ?2 AND title IN (?3, ?4)")
        .bind(dong_ngan::TITLE)
        .bind(dong_ngan::SLUG)
        .bind("Bến cũ trăng ngàn")
        .bind("Bến Cũ Trăng Ngàn")
        .exec(&mut tx)
        .await?;
    toasty::sql::statement(
        "UPDATE chapters SET title = ?1 WHERE story_slug = ?2 AND position = 1 AND title = ?3",
    )
    .bind(dong_ngan::TITLES[0])
    .bind(dong_ngan::SLUG)
    .bind("Khởi đầu · Người về bên sông")
    .exec(&mut tx)
    .await?;
    toasty::sql::statement(
        "UPDATE chapters SET title = ?1 WHERE story_slug = ?2 AND position = 13 AND title = ?3",
    )
    .bind(dong_ngan::TITLES[12])
    .bind(dong_ngan::SLUG)
    .bind("Bến cũ, trời đông ngàn")
    .exec(&mut tx)
    .await?;
    toasty::sql::statement("UPDATE chapters SET title = ?1, content = ?2 WHERE story_slug = ?3 AND position = 1 AND content = ?4")
        .bind(dong_ngan::TITLES[0]).bind(dong_ngan::CHAPTER_ONE.trim())
        .bind(dong_ngan::SLUG).bind(story.excerpt.join("\n\n"))
        .exec(&mut tx).await?;
    for (index, (title, content)) in dong_ngan::TITLES
        .iter()
        .zip(dong_ngan::CONTENTS.iter())
        .enumerate()
    {
        let position = index as u64 + 1;
        let existing = Chapter::all()
            .filter(Chapter::fields().story_slug().eq(dong_ngan::SLUG))
            .filter(Chapter::fields().position().eq(position))
            .first()
            .exec(&mut tx)
            .await?;
        if existing.is_none() {
            Chapter::create()
                .slug(format!("{}-{position}", dong_ngan::SLUG))
                .story_slug(dong_ngan::SLUG)
                .position(position)
                .title(*title)
                .content(content.trim())
                .published(true)
                .exec(&mut tx)
                .await?;
        } else if let Some(chapter) = existing.filter(|chapter| chapter.content.trim().is_empty()) {
            // Match the previous value as well, so an author edit is never overwritten.
            toasty::sql::statement(
                "UPDATE chapters SET content = ?1 WHERE id = ?2 AND content = ?3",
            )
            .bind(content.trim())
            .bind(chapter.id)
            .bind(&chapter.content)
            .exec(&mut tx)
            .await?;
        }
    }
    tx.commit().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn repeats_preserve_publication_and_author_content() {
        let mut db = crate::db::connect("sqlite::memory:").await.unwrap();
        assert_eq!(run(&mut db).await.unwrap(), 3);
        let initial = Chapter::all().exec(&mut db).await.unwrap();
        assert_eq!(initial.len(), 15);
        assert!(initial.iter().all(|chapter| chapter.published));
        // Includes an empty fixture that will be completed, and a non-fixture story.
        for (slug, content) in [
            ("ben-cu-dong-ngan-1", "Author draft"),
            ("ben-cu-dong-ngan-2", ""),
        ] {
            toasty::sql::statement(
                "UPDATE chapters SET content = ?1, published = FALSE WHERE slug = ?2",
            )
            .bind(content)
            .bind(slug)
            .exec(&mut db)
            .await
            .unwrap();
        }
        let other = initial
            .iter()
            .find(|chapter| chapter.story_slug != dong_ngan::SLUG)
            .unwrap();
        toasty::sql::statement("UPDATE chapters SET published = FALSE WHERE id = ?1")
            .bind(other.id)
            .exec(&mut db)
            .await
            .unwrap();
        for _ in 0..2 {
            assert_eq!(run(&mut db).await.unwrap(), 0);
        }
        let chapters = Chapter::all().exec(&mut db).await.unwrap();
        assert_eq!(chapters.len(), 15);
        for slug in ["ben-cu-dong-ngan-1", "ben-cu-dong-ngan-2", &other.slug] {
            assert!(
                !chapters
                    .iter()
                    .find(|chapter| chapter.slug == slug)
                    .unwrap()
                    .published
            );
        }
        assert_eq!(
            chapters
                .iter()
                .find(|chapter| chapter.slug == "ben-cu-dong-ngan-1")
                .unwrap()
                .content,
            "Author draft"
        );
        assert!(
            !chapters
                .iter()
                .find(|chapter| chapter.slug == "ben-cu-dong-ngan-2")
                .unwrap()
                .content
                .is_empty()
        );
    }
}
