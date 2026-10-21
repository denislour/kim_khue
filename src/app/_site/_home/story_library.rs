#[path = "story_library/genre_filters.rs"]
mod genre_filters_component;
#[path = "story_library/story_card.rs"]
mod story_card_component;
#[path = "story_library/story_grid.rs"]
mod story_grid_component;
#[path = "story_library/story_search.rs"]
mod story_search_component;

use genre_filters_component::genre_filters;
use story_grid_component::story_grid;
use story_search_component::story_search;

use crate::{
    app::urls::library_url,
    browser,
    components::ui::button::button,
    i18n::I18N,
    model::{Chapter, Story},
};
use std::collections::HashMap;
use topcoat::{
    Result,
    runtime::{Event, Signal},
    view::{View, attributes, component, view},
};

#[component]
pub async fn story_library(
    stories: &[Story],
    total: usize,
    matched: usize,
    current_page: usize,
    page_count: usize,
    page: &Signal<usize>,
    previews: &HashMap<String, Chapter>,
    query: &Signal<String>,
    genre: &Signal<String>,
) -> Result<impl View> {
    Ok(view! {
        <section
            class="library shell"
            id="library"
            aria-labelledby="stories-heading"
            :data-query=(browser::sync_library_url(query, genre, page))
        >
            <div class="library-heading">
                <div>
                    <h2 id="stories-heading">(I18N.stories_heading)</h2>
                    <p>(I18N.library_intro)</p>
                </div>
                story_search(query: query, page: page)
            </div>
            <div class="library-tools" id="library-tools">
                genre_filters(genre: genre, page: page)
                <output
                    id="results-count"
                    aria-live="polite"
                    data-label=(I18N.results_label)
                    data-one=(I18N.result_one)
                >
                    (matched)
                    " "
                    (I18N.results_label)
                </output>
            </div>
            story_grid(stories: stories, total: total, previews: previews)
            if page_count > 1 || current_page > 1 {
                <nav class="library-pagination" aria-label=(I18N.library_pagination)>
                    if current_page > 1 {
                        <a
                            href=(library_url(
                                &query.get(),
                                &genre.get(),
                                current_page - 1,
                            ))
                            rel="prev"
                        >
                            (I18N.previous_page)
                        </a>
                    }
                    <span>
                        (I18N.page_label)
                        " "
                        (current_page)
                        " / "
                        (page_count)
                    </span>
                    if current_page < page_count {
                        <a
                            href=(library_url(
                                &query.get(),
                                &genre.get(),
                                current_page + 1,
                            ))
                            rel="next"
                        >
                            (I18N.next_page)
                        </a>
                    }
                </nav>
            }
            if stories.is_empty() && matched > 0 {
                <div class="no-results">
                    <p>(I18N.page_empty)</p>
                    <a href=(library_url(&query.get(), &genre.get(), 1))>
                        (I18N.first_page)
                    </a>
                </div>
            }
            <div class="no-results" id="no-results" hidden=(matched != 0 || total == 0)>
                <p>(I18N.no_results)</p>
                button(
                    attrs: attributes! {
                        id="clear-search"
                        type="button"
                        class="clear-search-button"
                        @click=$(|_e: Event| {
                            page.set(1);
                            query.set("".to_owned());
                            genre.set("all".to_owned());
                            raw!(
                                "queueMicrotask(() => document.getElementById('story-search')?.focus());",
                                (),
                            );
                        })
                    },
                    (I18N.clear_search)
                )
            </div>
        </section>
    })
}
