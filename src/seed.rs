use crate::{features::SAMPLE_STORIES, model::Story};
use toasty::Db;

pub async fn run(db: &mut Db) -> toasty::Result<usize> {
    let mut inserted = 0;
    for sample in SAMPLE_STORIES {
        let existing = Story::all()
            .filter(Story::fields().slug().eq(sample.id))
            .first()
            .exec(db)
            .await?;

        if existing.is_some() {
            continue;
        }

        Story::create()
            .slug(sample.id)
            .title(sample.title)
            .genre_key(sample.genre_key)
            .cover(sample.cover)
            .summary(sample.summary)
            .exec(db)
            .await?;

        inserted += 1;
    }

    Ok(inserted)
}
