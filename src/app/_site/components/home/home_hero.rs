use crate::{
    components::ui::{
        button::{ButtonSize, ButtonVariant, button_variants},
        separator::separator,
    },
    i18n::I18N,
};
use topcoat::{
    Result,
    asset::asset,
    icon::{icon, iconify::iconify_icon},
    view::{View, attributes, class, component, view},
};

#[component]
pub async fn home_hero() -> Result<impl View> {
    Ok(view! {
        <section class="hero shell" aria-labelledby="hero-title">
            <img
                class="hero-landscape"
                src=(asset!("assets/landscape-hero.svg"))
                width="1000"
                height="690"
                alt=""
                fetchpriority="high"
            >
            <div class="hero-copy">
                <p class="hero-signature">
                    <span aria-hidden="true"></span>
                    (I18N.brand_note)
                </p>
                <h1 id="hero-title">(I18N.hero_heading)</h1>
                <p class="hero-intro">(I18N.intro)</p>
                <a
                    class=(class!(
                        button_variants(ButtonVariant::Primary, ButtonSize::Lg),
                        "primary-link",
                    ))
                    href="#library"
                >
                    (I18N.explore)
                    icon(
                        data: iconify_icon!("lucide:arrow-up-right"),
                        attrs: attributes! { class="link-icon" aria-hidden="true" }
                    )
                </a>
                <p class="hero-footnote">(I18N.hero_footnote)</p>
            </div>
            <div class="landscape-inscription">
                for quote in [I18N.hero_quote, I18N.hero_quote_second] {
                    <p>
                        <span class="sr-only">(quote)</span>
                        <span class="inscription-words" aria-hidden="true">
                            for word in quote.split_whitespace() {
                                <span>(word)</span>
                            }
                        </span>
                    </p>
                }
            </div>
            <div class="hero-bottom" aria-hidden="true">
                separator(attrs: attributes! { class="hero-rule" })
                <img
                    src=(asset!("assets/logo-kim-khue.svg"))
                    width="30"
                    height="30"
                    alt=""
                >
                separator(attrs: attributes! { class="hero-rule" })
            </div>
        </section>
    })
}
