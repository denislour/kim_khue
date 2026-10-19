use crate::{
    components::ui::toggle::{ToggleKind, ToggleSize, toggle, toggle_group},
    i18n::I18N,
};
use topcoat::{
    Result,
    view::{View, attributes, component, view},
};

#[component]
pub async fn genre_filters() -> Result<impl View> {
    let genres = [
        ("all", I18N.all_genres),
        ("wuxia", I18N.wuxia),
        ("romance", I18N.romance),
        ("fantasy", I18N.fantasy),
    ];
    Ok(view! {
        toggle_group(
            attrs: attributes! {
                class="genre-filters"
                role="radiogroup"
                aria-label=(I18N.filter_label)
            },
            for (key, text) in genres {
                toggle(
                    kind: ToggleKind::Exclusive,
                    size: ToggleSize::Sm,
                    attrs: attributes! {
                        name="genre"
                        value=(key)
                        data-genre=(key)
                        checked=(key == "all")
                        class="genre-toggle"
                    },
                    (text)
                )
            }
        )
    })
}
