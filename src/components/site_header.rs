use crate::{
    components::button::{ButtonSize, ButtonVariant, button_variants},
    i18n::TEXT,
};
use topcoat::{
    Result,
    asset::asset,
    view::{View, class, component, view},
};

#[component]
pub async fn site_header() -> Result<impl View> {
    Ok(view! {
        <header class="site-header shell">
            <a class="brand" href="/" translate="no">
                <img src=(asset!("assets/logo.svg")) width="58" height="58" alt="">
                <span>
                    <span class="brand-name">(TEXT.brand)</span>
                    <span class="brand-note">(TEXT.brand_note)</span>
                </span>
            </a>
            <nav class="main-nav" aria-label=(TEXT.main_navigation)>
                <a
                    class=(button_variants(ButtonVariant::Ghost, ButtonSize::Sm))
                    href="#library"
                >
                    (TEXT.catalog)
                </a>
                <a
                    class=(button_variants(ButtonVariant::Ghost, ButtonSize::Sm))
                    href="#about"
                >
                    (TEXT.about_nav)
                </a>
            </nav>
            <a
                class=(class!(
                    button_variants(ButtonVariant::Outline, ButtonSize::Sm),
                    "header-reading",
                ))
                href="#story-river"
            >
                (TEXT.sample_nav)
                <span aria-hidden="true">" ↗"</span>
            </a>
        </header>
    })
}
