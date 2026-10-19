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
        accordion::{accordion, accordion_content, accordion_item, accordion_trigger},
        badge::{BadgeVariant, badge},
        button::{ButtonSize, ButtonVariant, button, button_variants},
        card::{card, card_content, card_description, card_footer, card_header, card_title},
        dialog::{dialog, dialog_content, dialog_description, dialog_header, dialog_title},
        input::input,
        label::label,
        separator::separator,
        toggle::{ToggleKind, ToggleSize, toggle, toggle_group},
    },
    features::SAMPLE_STORIES,
    i18n::{Locale, Messages, messages},
};

static TEXT: &Messages = messages(Locale::Vi);

pub fn router(db: toasty::Db) -> Router {
    module_router!()
        .discover()
        .app_context(db)
        .assets(AssetBundle::load().unwrap())
        .runtime()
        .build()
}

fn cover(name: &str) -> Asset {
    match name {
        "mai" => asset!("assets/hoa-mai.svg"),
        "moon" => asset!("assets/moon.svg"),
        _ => asset!("assets/landscape.svg"),
    }
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
                dialog(
                    open: false,
                    attrs: attributes! {
                        class="reader"
                        id="reader"
                        aria-labelledby="reader-title"
                    },
                    dialog_content(
                        attrs: attributes! { class="reader-panel" },
                        <div class="reader-toolbar">
                            <span class="reader-brand" translate="no">
                                (TEXT.brand)
                            </span>
                            <div class="reader-settings">
                                label(
                                    attrs: attributes! { for="reader-size" },
                                    (TEXT.font_size)
                                )
                                input(
                                    attrs: attributes! {
                                        id="reader-size"
                                        class="reader-size"
                                        aria-label=(TEXT.font_size)
                                        type="range"
                                        min="16"
                                        max="26"
                                        value="20"
                                    }
                                )
                                toggle(
                                    size: ToggleSize::Sm,
                                    attrs: attributes! {
                                        id="reader-theme"
                                        name="reader-night"
                                        class="reader-theme"
                                    },
                                    (TEXT.night_mode)
                                )
                                button(
                                    variant: ButtonVariant::Ghost,
                                    size: ButtonSize::Icon,
                                    attrs: attributes! {
                                        id="reader-close"
                                        class="close-reader"
                                        type="button"
                                        aria-label=(TEXT.close_reader)
                                    },
                                    "×"
                                )
                            </div>
                        </div>
                        <div class="reader-page">
                            dialog_header(
                                attrs: attributes! { class="reader-heading" },
                                badge(
                                    variant: BadgeVariant::Outline,
                                    attrs: attributes! { class="sample-label" },
                                    (TEXT.sample_badge)
                                )
                                dialog_title(attrs: attributes! { id="reader-title" }, "")
                                dialog_description(
                                    attrs: attributes! { id="reader-chapter" class="reader-chapter" },
                                    ""
                                )
                            )
                            <div id="reader-content" class="reader-prose"></div>
                            separator(
                                attrs: attributes! { class="reader-divider" aria-hidden="true" }
                            )
                            <p class="reader-end">(TEXT.reader_end)</p>
                            dialog_description(
                                attrs: attributes! { class="reader-notice" },
                                (TEXT.reader_notice)
                            )
                        </div>
                    )
                )
            </body>
        </html>
    })
}

#[page]
async fn home() -> Result<impl View> {
    let genres = [
        ("all", TEXT.all_genres),
        ("wuxia", TEXT.wuxia),
        ("romance", TEXT.romance),
        ("fantasy", TEXT.fantasy),
    ];
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

            <section
                class="library shell"
                id="library"
                aria-labelledby="stories-heading"
            >
                <div class="library-heading">
                    <div>
                        <h2 id="stories-heading">(TEXT.stories_heading)</h2>
                        <p>(TEXT.library_intro)</p>
                    </div>
                    <form class="search" id="search-form" role="search" hidden=(true)>
                        label(
                            attrs: attributes! { class="sr-only" for="story-search" },
                            (TEXT.search_label)
                        )
                        <span aria-hidden="true" class="search-icon"></span>
                        input(
                            attrs: attributes! {
                                id="story-search"
                                type="search"
                                name="q"
                                placeholder=(TEXT.search_placeholder)
                                autocomplete="off"
                            }
                        )
                    </form>
                </div>
                <div class="library-tools" id="library-tools" hidden=(true)>
                    toggle_group(
                        attrs: attributes! {
                            class="genre-filters"
                            role="radiogroup"
                            aria-label=(TEXT.filter_label)
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
                    <output
                        id="results-count"
                        aria-live="polite"
                        data-label=(TEXT.results_label)
                        data-one=(TEXT.result_one)
                    ></output>
                </div>
                <p class="sample-notice">(TEXT.sample_notice)</p>
                <p class="no-results" hidden=(SAMPLE_STORIES.is_empty())>
                    (TEXT.empty_stories)
                </p>
                <div class="books" id="books">
                    for story in SAMPLE_STORIES {
                        card(
                            attrs: attributes! {
                                class="story"
                                id=(format!("story-{}", story.id))
                                data-genre=(story.genre_key)
                                role="article"
                                aria-labelledby=(format!("title-{}", story.id))
                            },
                            <div
                                class=(format!("book-cover cover-{}", story.cover))
                                aria-hidden="true"
                            >
                                <span class="book-binding"></span>
                                <img
                                    class="cover-art"
                                    src=(cover(story.cover))
                                    width="400"
                                    height="500"
                                    alt=""
                                    loading="lazy"
                                >
                                <div class="cover-title">
                                    <span>(story.title)</span>
                                    <span class="cover-author">(TEXT.brand)</span>
                                </div>
                                <span class="cover-seal">"KK"</span>
                                <span class="cover-rule"></span>
                            </div>
                            card_header(
                                attrs: attributes! { class="book-copy" },
                                <div class="book-meta">
                                    badge(
                                        variant: BadgeVariant::Outline,
                                        attrs: attributes! { class="genre-badge" },
                                        (story.genre)
                                    )
                                    badge(
                                        variant: BadgeVariant::Outline,
                                        attrs: attributes! { class="sample-label" },
                                        (TEXT.sample_badge)
                                    )
                                </div>
                                card_title(
                                    attrs: attributes! { class="story-title" id=(format!("title-{}", story.id)) },
                                    (story.title)
                                )
                                card_description(
                                    attrs: attributes! { class="story-summary" },
                                    (story.summary)
                                )
                            )
                            card_footer(
                                attrs: attributes! { class="book-actions" },
                                accordion(
                                    accordion_item(
                                        attrs: attributes! { class="story-preview" },
                                        accordion_trigger((TEXT.read_sample))
                                        accordion_content(
                                            attrs: attributes! { class="excerpt" },
                                            <h4>(story.chapter)</h4>
                                            for paragraph in story.excerpt {
                                                <p>(paragraph)</p>
                                            }
                                        )
                                    )
                                )
                            )
                        )
                    }
                </div>
                <div class="no-results" id="no-results" hidden=(true)>
                    <p>(TEXT.no_results)</p>
                    button(
                        attrs: attributes! {
                            id="clear-search"
                            type="button"
                            class="clear-search-button"
                        },
                        (TEXT.clear_search)
                    )
                </div>
            </section>

            <section class="shell" id="about" aria-labelledby="about-heading">
                card(
                    attrs: attributes! { class="about" },
                    card_content(
                        attrs: attributes! { class="about-art" aria-hidden="true" },
                        <img
                            src=(asset!("assets/logo.svg"))
                            width="160"
                            height="160"
                            alt=""
                        >
                        <span>(TEXT.brand)</span>
                    )
                    card_content(
                        attrs: attributes! { class="about-copy" },
                        <h2 id="about-heading">(TEXT.about_heading)</h2>
                        card_description((TEXT.about_text))
                        <p class="author-note">(TEXT.about_note)</p>
                    )
                )
            </section>
        </main>
    })
}
