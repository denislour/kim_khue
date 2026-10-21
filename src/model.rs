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

#[derive(Debug, toasty::Model)]
pub struct Chapter {
    #[key]
    #[auto]
    pub id: u64,
    #[unique]
    pub slug: String,
    #[index]
    pub story_slug: String,
    pub position: u64,
    pub title: String,
    pub content: String,
    #[default(false)]
    pub published: bool,
}
