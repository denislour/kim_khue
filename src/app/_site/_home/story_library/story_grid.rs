use super::story_card_component::story_card;
use crate::{
    i18n::I18N,
    model::{Chapter, Story},
};
use std::collections::HashMap;
use topcoat::{
    Result,
    view::{View, component, view},
};

#[component]
pub async fn story_grid(
    stories: &[Story],
    total: usize,
    previews: &HashMap<String, Chapter>,
) -> Result<impl View> {
    Ok(view! {
        <p class="no-results" hidden=(total != 0)>(I18N.empty_stories)</p>
        <div class="books" id="books">
            #[key(&story.slug)]
            for story in stories {
                story_card(story: story, preview: previews.get(&story.slug))
            }
        </div>
    })
}
