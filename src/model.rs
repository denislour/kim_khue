#[derive(Debug, toasty::Model)]
pub struct Story {
    #[key]
    #[auto]
    pub id: u64,

    #[unique]
    pub slug: String,

    pub title: String,
    pub genre_key: String,
    pub cover: String,
    pub summary: String,
}
