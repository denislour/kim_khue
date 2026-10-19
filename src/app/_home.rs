use crate::components::accordion::{
    accordion, accordion_content, accordion_item, accordion_trigger,
};
use crate::components::badge::{BadgeVariant, badge};
use crate::components::button::button;
use crate::components::card::{
    card, card_content, card_description, card_footer, card_header, card_title,
};
use crate::components::home_hero::home_hero;
use crate::components::input::input;
use crate::components::label::label;
use crate::components::toggle::{ToggleKind, ToggleSize, toggle, toggle_group};
use crate::features::SAMPLE_STORIES;
use crate::i18n::TEXT;
use topcoat::{
    Result,
    asset::{Asset, asset},
    router::page,
    view::{View, attributes, view},
};

fn cover(name: &str) -> Asset {
    match name {
        "mai" => asset!("assets/hoa-mai.svg"),
        "moon" => asset!("assets/moon.svg"),
        _ => asset!("assets/landscape.svg"),
    }
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
            home_hero()

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
