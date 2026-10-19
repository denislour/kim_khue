use crate::{
    components::{
        button::{ButtonSize, ButtonVariant, button_variants},
        separator::separator,
    },
    i18n::TEXT,
};
use topcoat::{
    Result,
    asset::asset,
    view::{View, attributes, class, component, view},
};

#[component]
pub async fn home_hero() -> Result<impl View> {
    Ok(view! {
        <section class="hero shell" aria-labelledby="hero-title">
            <img
                class="hero-landscape"
                src=(asset!("assets/landscape.svg"))
                width="1000"
                height="690"
                alt=""
                fetchpriority="high"
            >
            <div class="hero-copy">
                <p class="hero-signature">
                    <span aria-hidden="true"></span>
                    (TEXT.brand_note)
                </p>
                <h1 id="hero-title">(TEXT.hero_heading)</h1>
                <p class="hero-intro">(TEXT.intro)</p>
                <a
                    class=(class!(
                        button_variants(ButtonVariant::Primary, ButtonSize::Lg),
                        "primary-link",
                    ))
                    href="#library"
                >
                    (TEXT.explore)
                    <span aria-hidden="true">" ↗"</span>
                </a>
                <p class="hero-footnote">(TEXT.hero_footnote)</p>
            </div>
            <p class="landscape-inscription">(TEXT.hero_quote)</p>
            <div class="hero-bottom" aria-hidden="true">
                separator(attrs: attributes! { class="hero-rule" })
                <img src=(asset!("assets/logo.svg")) width="30" height="30" alt="">
                separator(attrs: attributes! { class="hero-rule" })
            </div>
        </section>
    })
}
