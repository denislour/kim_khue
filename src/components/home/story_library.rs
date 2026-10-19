mod genre_filters;
mod story_grid;
mod story_search;

use crate::{components::ui::button::button, features::SAMPLE_STORIES, i18n::I18N};
use topcoat::{
    Result,
    view::{View, attributes, component, view},
};

#[component]
pub async fn story_library() -> Result<impl View> {
    Ok(view! {
        <section class="library shell" id="library" aria-labelledby="stories-heading">
            <div class="library-heading">
                <div>
                    <h2 id="stories-heading">(I18N.stories_heading)</h2>
                    <p>(I18N.library_intro)</p>
                </div>
                story_search::story_search()
            </div>
            <div class="library-tools" id="library-tools" hidden=(true)>
                genre_filters::genre_filters()
                <output
                    id="results-count"
                    aria-live="polite"
                    data-label=(I18N.results_label)
                    data-one=(I18N.result_one)
                ></output>
            </div>
            <p class="sample-notice">(I18N.sample_notice)</p>
            story_grid::story_grid(stories: SAMPLE_STORIES)
            <div class="no-results" id="no-results" hidden=(true)>
                <p>(I18N.no_results)</p>
                button(
                    attrs: attributes! {
                        id="clear-search"
                        type="button"
                        class="clear-search-button"
                    },
                    (I18N.clear_search)
                )
            </div>
        </section>
    })
}
