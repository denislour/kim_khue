use crate::{
    components::ui::{input::input, label::label},
    i18n::I18N,
};
use topcoat::icon::{icon, iconify::iconify_icon};
use topcoat::{
    Result,
    runtime::{Event, Signal},
    view::{View, attributes, component, view},
};

#[component]
pub async fn story_search(query: &Signal<String>, page: &Signal<usize>) -> Result<impl View> {
    Ok(view! {
        <form
            class="search"
            id="search-form"
            role="search"
            action="/"
            method="get"
            @submit=$(|e: Event| e.prevent_default())
        >
            label(
                attrs: attributes! { class="sr-only" for="story-search" },
                (I18N.search_label)
            )
            icon(
                data: iconify_icon!("lucide:search"),
                attrs: attributes! { class="search-glyph" aria-hidden="true" }
            )
            input(
                attrs: attributes! {
                    id="story-search"
                    type="search"
                    name="q"
                    maxlength="120"
                    placeholder=(I18N.search_placeholder)
                    autocomplete="off"
                    :value=$(query.get())
                    @input=$(|e: Event| {
                        page.set(1);
                        query.set(e.target.value);
                    })
                }
            )
        </form>
    })
}
