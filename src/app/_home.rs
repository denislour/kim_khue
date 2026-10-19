use crate::components::author_intro::author_intro;
use crate::components::home_hero::home_hero;
use crate::components::story_library::story_library;
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
