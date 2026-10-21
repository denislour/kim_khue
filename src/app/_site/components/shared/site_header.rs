use crate::{
    browser,
    components::ui::button::{ButtonSize, ButtonVariant, button, button_variants},
    i18n::I18N,
};
use topcoat::{
    Result,
    asset::asset,
    context::Cx,
    icon::{icon, iconify::iconify_icon},
    runtime::{Event, signal},
    view::{View, attributes, component, view},
};

#[component]
pub async fn site_header(cx: &Cx) -> Result<impl View> {
    let theme = signal(cx, || "system".to_owned());
    Ok(view! {
        <header class="site-header shell" :data-theme=(browser::site_theme(&theme))>
            <a class="brand" href="/" translate="no">
                <img
                    src=(asset!("assets/logo-kim-khue.svg"))
                    width="58"
                    height="58"
                    alt=""
                >
                <span>
                    <span class="brand-name">(I18N.brand)</span>
                    <span class="brand-note">(I18N.brand_note)</span>
                </span>
            </a>
            <nav class="main-nav" aria-label=(I18N.main_navigation)>
                <a
                    class=(button_variants(ButtonVariant::Ghost, ButtonSize::Sm))
                    href="/#library"
                >
                    (I18N.catalog)
                </a>
                <a
                    class=(button_variants(ButtonVariant::Ghost, ButtonSize::Sm))
                    href="/#about"
                >
                    (I18N.about_nav)
                </a>
            </nav>
            button(
                variant: ButtonVariant::Ghost,
                size: ButtonSize::Icon,
                attrs: attributes! {
                    id="site-theme"
                    class="site-theme"
                    type="button"
                    aria-label=(I18N.theme_toggle)
                    title=(I18N.theme_toggle)
                    @click=$(|_e: Event| {
                        let dark = raw!(
                            "cx.hydrate(document.documentElement.classList.contains('dark'))",
                            false,
                        );
                        if dark {
                            theme.set("light".to_owned());
                        } else {
                            theme.set("dark".to_owned());
                        }
                    })
                },
                icon(
                    data: iconify_icon!("lucide:moon"),
                    attrs: attributes! { class="theme-moon" aria-hidden="true" }
                )
                icon(
                    data: iconify_icon!("lucide:sun"),
                    attrs: attributes! { class="theme-sun" aria-hidden="true" }
                )
            )
        </header>
    })
}
