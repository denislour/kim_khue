use crate::components::home::{
    author_intro::author_intro, home_hero::home_hero, story_library::story_library,
};
use topcoat::{
    Result,
    router::page,
    view::{View, view},
};

#[page]
async fn home() -> Result<impl View> {
    Ok(view! {
        <main id="main">
            home_hero()
            story_library()
            author_intro()
        </main>
    })
}
