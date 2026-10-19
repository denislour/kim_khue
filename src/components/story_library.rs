use super::story_card::story_card;
use crate::{
    components::{
        button::button,
        input::input,
        label::label,
        toggle::{ToggleKind, ToggleSize, toggle, toggle_group},
    },
    features::SAMPLE_STORIES,
    i18n::TEXT,
};
use topcoat::{
    Result,
    view::{View, attributes, component, view},
};

#[component]
pub async fn story_library() -> Result<impl View> {
    let genres = [
        ("all", TEXT.all_genres),
        ("wuxia", TEXT.wuxia),
        ("romance", TEXT.romance),
        ("fantasy", TEXT.fantasy),
    ];
    Ok(view! {
        <section class="library shell" id="library" aria-labelledby="stories-heading">
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
                    story_card(story: story)
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
    })
}
