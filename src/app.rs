use topcoat::{
    Result,
    asset::{Asset, AssetBundle, RouterBuilderAssetExt, asset},
    context::Cx,
    router::{Router, RouterBuilderDiscoverExt, Slot, layout, module_router, page},
    runtime::RouterBuilderRuntimeExt,
    view::{View, attributes, class, view},
};

use crate::{
    components::{
        button::{ButtonSize, ButtonVariant, button, button_variants},
        separator::separator,
    },
    i18n::{Locale, Messages, messages},
};

static TEXT: &Messages = messages(Locale::Vi);

pub fn router() -> Router {
    module_router!()
        .discover()
        .assets(AssetBundle::load().unwrap())
        .runtime()
        .build()
}

#[layout]
async fn root(cx: &Cx, slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html lang=(TEXT.lang)>
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <meta name="theme-color" content="#ffffff">
                <meta name="description" content=(TEXT.intro)>
                <title>(TEXT.title)</title>
                <link rel="icon" type="image/svg+xml" href=(asset!("assets/logo.svg"))>
                <style>(crate::fonts::styles(cx))</style>
                <link rel="stylesheet" href=(topcoat::tailwind::stylesheet!())>
                <link rel="stylesheet" href=(asset!("styles.custom.css"))>

                topcoat::runtime::script()
                topcoat::dev::script()
            </head>
            <body id="top">
                <a class="skip-link" href="#main">(TEXT.skip_content)</a>
                <header class="site-header shell">
                    <a class="brand" href="/" translate="no">
                        <img
                            src=(asset!("assets/logo.svg"))
                            width="58"
                            height="58"
                            alt=""
                        >
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
                (slot)
                <footer class="site-footer shell">
                    <a class="footer-brand" href="#top" translate="no">(TEXT.brand)</a>
                    <p>(TEXT.footer)</p>
                    <a
                        class=(class!(
                            button_variants(ButtonVariant::Ghost, ButtonSize::Sm),
                            "back-to-top",
                        ))
                        href="#top"
                    >
                        (TEXT.back_top)
                        <span aria-hidden="true">" ↑"</span>
                    </a>
                </footer>
            </body>
        </html>
    })
}

#[page]
async fn home() -> Result<impl View> {
    Ok(view! {
        <main id="main">
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

        </main>
    })
}
