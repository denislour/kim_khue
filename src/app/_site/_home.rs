use super::components::home::{author_intro, home_hero, story_library};
use crate::{db::db, services};
use topcoat::{
    Result,
    context::Cx,
    router::{page, query_params},
    runtime::signal,
    view::{View, view},
};

#[query_params(error = bad_request)]
struct HomeQuery {
    q: Option<String>,
    genre: Option<String>,
    page: Option<usize>,
}

#[page]
pub(crate) async fn home_page(cx: &Cx) -> Result<impl View> {
    let params = query_params::<HomeQuery>(cx)?;
    let initial_query = params
        .q
        .as_deref()
        .unwrap_or("")
        .chars()
        .take(services::story::MAX_QUERY_CHARS)
        .collect::<String>();
    let initial_genre =
        services::story::genre_key(params.genre.as_deref().unwrap_or("all")).to_owned();
    let initial_page = params.page.unwrap_or(1).max(1);
    let page = signal(cx, move || initial_page);
    let query = signal(cx, move || initial_query);
    let genre = signal(cx, move || initial_genre);
    let catalog =
        services::story::search_page(&mut db(cx), &query.get(), &genre.get(), page.get()).await?;

    Ok(view! {
        <main id="main">
            home_hero()
            story_library(
                stories: &catalog.stories,
                total: catalog.total,
                matched: catalog.matched,
                current_page: catalog.page,
                page_count: catalog.page_count,
                page: &page,
                previews: &catalog.previews,
                query: &query,
                genre: &genre
            )
            author_intro()
        </main>
    })
}
