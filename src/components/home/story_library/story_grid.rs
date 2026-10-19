use crate::{components::home::story_card::story_card, features::Story, i18n::I18N};
use topcoat::{
    Result,
    view::{View, component, view},
};

#[component]
pub async fn story_grid(stories: &[Story]) -> Result<impl View> {
    Ok(view! {
        <p class="no-results" hidden=(stories.is_empty())>(I18N.empty_stories)</p>
        <div class="books" id="books">
            for story in stories {
                story_card(story: story)
            }
        </div>
    })
}
