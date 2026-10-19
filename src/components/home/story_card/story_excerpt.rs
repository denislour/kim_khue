use crate::{
    components::ui::accordion::{accordion, accordion_content, accordion_item, accordion_trigger},
    features::Story,
    i18n::I18N,
};
use topcoat::{
    Result,
    view::{View, attributes, component, view},
};

#[component]
pub async fn story_excerpt(story: &Story) -> Result<impl View> {
    Ok(view! {
        accordion(
            accordion_item(
                attrs: attributes! { class="story-preview" },
                accordion_trigger((I18N.read_sample))
                accordion_content(
                    attrs: attributes! { class="excerpt" },
                    <h4>(story.chapter)</h4>
                    for paragraph in story.excerpt {
                        <p>(paragraph)</p>
                    }
                )
            )
        )
    })
}
