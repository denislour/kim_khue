#[path = "slug/chapter_content.rs"]
mod chapter_content_component;
#[path = "slug/chapter_index.rs"]
mod chapter_index_component;

use crate::{app::urls::chapter_url, db::db, i18n::I18N, services::story::read};
use chapter_content_component::chapter_content;
use chapter_index_component::chapter_index;
use topcoat::{
    Result,
    context::Cx,
    router::{
        error::{RouterErrorExt, redirect_permanent},
        module_param, page, path_param, query_params,
    },
    view::{View, view},
};

module_param!(slug);

#[query_params(error = bad_request)]
struct StoryQuery {
    chapter: Option<u64>,
}

#[page]
pub(crate) async fn story_page(cx: &Cx) -> Result<impl View> {
    let slug = path_param::<Slug>(cx);
    let params = query_params::<StoryQuery>(cx)?;
    let position = params.chapter.unwrap_or(1);
    let reading = read(&mut db(cx), slug, position).await?.ok_or_not_found()?;
    if slug != reading.story.slug {
        return Err(redirect_permanent(chapter_url(&reading.story.slug, position)).into());
    }
    let published = reading
        .chapters
        .iter()
        .filter(|chapter| chapter.available)
        .count();
    Ok(view! {
        <main id="main" class="story-detail shell">
            <header class="story-detail-heading">
                <a class="detail-back" href="/#library">
                    "← "
                    (I18N.back_library)
                </a>
                <div class="detail-book-meta">
                    <span>(I18N.genre_label(&reading.story.genre_key))</span>
                    <span>
                        (reading.total)
                        " "
                        (I18N.chapter_count)
                        " · "
                        (published)
                        " "
                        (I18N.chapters_published)
                    </span>
                </div>
                <h1>(&reading.story.title)</h1>
                <p class="detail-author" translate="no">(I18N.brand)</p>
            </header>
            <div class="reading-layout">
                chapter_index(
                    slug: &reading.story.slug,
                    chapters: &reading.chapters,
                    current: position
                )
                chapter_content(reading: &reading)
            </div>
        </main>
    })
}
