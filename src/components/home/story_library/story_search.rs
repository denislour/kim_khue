use crate::{
    components::ui::{input::input, label::label},
    i18n::I18N,
};
use topcoat::{
    Result,
    view::{View, attributes, component, view},
};

#[component]
pub async fn story_search() -> Result<impl View> {
    Ok(view! {
        <form class="search" id="search-form" role="search" hidden=(true)>
            label(
                attrs: attributes! { class="sr-only" for="story-search" },
                (I18N.search_label)
            )
            <span aria-hidden="true" class="search-icon"></span>
            input(
                attrs: attributes! {
                    id="story-search"
                    type="search"
                    name="q"
                    placeholder=(I18N.search_placeholder)
                    autocomplete="off"
                }
            )
        </form>
    })
}
