use crate::model::{Chapter, Story};
use toasty::Db;
use topcoat::context::{Cx, app_context};

pub async fn connect(database_url: &str) -> toasty::Result<Db> {
    let mut db = Db::builder()
        .models(toasty::models!(Story, Chapter))
        .connect(database_url)
        .await?;

    migrate(&mut db).await?;
    Ok(db)
}

const MIGRATIONS: &[(i64, &str)] = &[
    (1, include_str!("../migrations/001_stories.sql")),
    (2, include_str!("../migrations/002_chapters.sql")),
    (3, include_str!("../migrations/003_publication.sql")),
    (
        4,
        include_str!("../migrations/004_backfill_publication.sql"),
    ),
];

async fn migrate(db: &mut Db) -> toasty::Result<()> {
    let mut tx = db.transaction().await?;
    // Acquire the SQLite writer lock before reading history, including on first boot.
    toasty::sql::statement("CREATE TABLE IF NOT EXISTS schema_migrations (version INTEGER PRIMARY KEY, applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP)")
        .exec(&mut tx).await?;
    toasty::sql::statement("UPDATE schema_migrations SET version = version WHERE 0")
        .exec(&mut tx)
        .await?;
    for &(version, sql) in MIGRATIONS {
        let applied =
            toasty::sql::query("SELECT version FROM schema_migrations WHERE version = ?1")
                .bind(version)
                .exec(&mut tx)
                .await?;
        if applied.is_empty() {
            toasty::sql::statement(sql).exec(&mut tx).await?;
            toasty::sql::statement("INSERT INTO schema_migrations (version) VALUES (?1)")
                .bind(version)
                .exec(&mut tx)
                .await?;
        }
    }
    tx.commit().await
}

pub fn db(cx: &Cx) -> Db {
    app_context::<Db>(cx).clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn legacy_db() -> Db {
        let mut db = Db::builder()
            .models(toasty::models!(Story, Chapter))
            .connect("sqlite::memory:")
            .await
            .unwrap();
        for sql in [MIGRATIONS[0].1, MIGRATIONS[1].1] {
            toasty::sql::statement(sql).exec(&mut db).await.unwrap();
        }
        toasty::sql::statement("INSERT INTO stories (id, slug, title, genre_key, cover, summary) VALUES (42, 'old', 'Old title', 'wuxia', 'cover', 'Author summary')")
            .exec(&mut db).await.unwrap();
        for (id, content) in [(7i64, "Existing prose"), (8, " \n\t\u{2003}")] {
            toasty::sql::statement("INSERT INTO chapters (id, slug, story_slug, position, title, content) VALUES (?1, ?2, 'old', ?1, 'Author title', ?3)")
                .bind(id).bind(format!("old-{id}")).bind(content).exec(&mut db).await.unwrap();
        }
        db
    }

    #[tokio::test]
    async fn upgrade_preserves_legacy_rows_and_only_backfills_once() {
        let mut db = legacy_db().await;
        migrate(&mut db).await.unwrap();
        let stories = Story::all().exec(&mut db).await.unwrap();
        assert_eq!(stories[0].id, 42);
        assert_eq!(stories[0].summary, "Author summary");
        let chapters = Chapter::all()
            .order_by(Chapter::fields().id().asc())
            .exec(&mut db)
            .await
            .unwrap();
        assert_eq!(chapters[0].id, 7);
        assert_eq!(chapters[0].content, "Existing prose");
        assert!(chapters[0].published);
        assert!(!chapters[1].published);
        toasty::sql::statement("UPDATE chapters SET published = FALSE WHERE id = 7")
            .exec(&mut db)
            .await
            .unwrap();
        for _ in 0..2 {
            migrate(&mut db).await.unwrap();
        }
        assert!(
            !Chapter::all()
                .filter(Chapter::fields().id().eq(7u64))
                .first()
                .exec(&mut db)
                .await
                .unwrap()
                .unwrap()
                .published
        );
        assert_eq!(
            toasty::sql::query("SELECT version FROM schema_migrations")
                .exec(&mut db)
                .await
                .unwrap()
                .len(),
            MIGRATIONS.len()
        );
        let new = Chapter::create()
            .slug("new")
            .story_slug("old")
            .position(9u64)
            .title("Draft")
            .content("Written, not published")
            .exec(&mut db)
            .await
            .unwrap();
        assert!(!new.published);
        // Raw SQL inserts get the same safe default as model inserts.
        toasty::sql::statement("INSERT INTO chapters (slug, story_slug, position, title, content) VALUES ('raw', 'old', 10, 'Draft', 'Secret')")
            .exec(&mut db).await.unwrap();
        assert!(
            !Chapter::all()
                .filter(Chapter::fields().slug().eq("raw"))
                .first()
                .exec(&mut db)
                .await
                .unwrap()
                .unwrap()
                .published
        );
    }

    #[tokio::test]
    async fn failed_upgrade_rolls_back_schema_and_history() {
        let mut db = legacy_db().await;
        // Simulate schema drift: migration 003 must fail rather than hide it.
        toasty::sql::statement(MIGRATIONS[2].1)
            .exec(&mut db)
            .await
            .unwrap();
        assert!(migrate(&mut db).await.is_err());
        assert!(
            toasty::sql::query("SELECT version FROM schema_migrations")
                .exec(&mut db)
                .await
                .is_err()
        );
        assert_eq!(Story::all().exec(&mut db).await.unwrap()[0].id, 42);
        assert_eq!(Chapter::all().exec(&mut db).await.unwrap().len(), 2);
    }
}
