use crate::{
    components::ui::button::{ButtonSize, ButtonVariant, button_variants},
    i18n::I18N,
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
                    <span class="brand-name">(I18N.brand)</span>
                    <span class="brand-note">(I18N.brand_note)</span>
                </span>
            </a>
            <nav class="main-nav" aria-label=(I18N.main_navigation)>
                <a
                    class=(button_variants(ButtonVariant::Ghost, ButtonSize::Sm))
                    href="#library"
                >
                    (I18N.catalog)
                </a>
                <a
                    class=(button_variants(ButtonVariant::Ghost, ButtonSize::Sm))
                    href="#about"
                >
                    (I18N.about_nav)
                </a>
            </nav>
            <a
                class=(class!(
                    button_variants(ButtonVariant::Outline, ButtonSize::Sm),
                    "header-reading",
                ))
                href="#story-river"
            >
                (I18N.sample_nav)
                <span aria-hidden="true">" ↗"</span>
            </a>
        </header>
    })
}
