use crate::{
    components::ui::{
        toggle::toggle_group,
        toggle::{ToggleKind, ToggleSize, toggle},
    },
    i18n::I18N,
};
use topcoat::{
    Result,
    runtime::{Event, Signal},
    view::{View, attributes, component, view},
};

#[component]
pub async fn genre_filters(genre: &Signal<String>, page: &Signal<usize>) -> Result<impl View> {
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
                        :checked=$(genre.get() == key)
                        @change=$(|_e: Event| {
                            page.set(1);
                            genre.set(key.to_owned());
                        })
                        class="genre-toggle"
                    },
                    (text)
                )
            }
        )
    })
}
