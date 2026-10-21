use crate::{
    app::urls::story_url,
    browser,
    i18n::I18N,
    model::{Chapter, Story},
};
use topcoat::{
    Result,
    asset::{Asset, asset},
    context::Cx,
    runtime::{Event, signal},
    view::{View, component, view},
};

const PAGE_SLICES: usize = 32;

fn cover(name: &str) -> Asset {
    match name {
        "mai" => asset!("assets/hoa-mai.svg"),
        "moon" => asset!("assets/moon.svg"),
        _ => asset!("assets/landscape-hero.svg"),
    }
}

#[component]
async fn cover_artwork(story: &Story) -> Result<impl View> {
    Ok(view! {
        <span class="book-binding"></span>
        <img
            class="cover-art"
            src=(cover(&story.cover))
            width="400"
            height="500"
            alt=""
            loading="lazy"
        >
        <span class="cover-author">(I18N.brand)</span>
        <div class="cover-title">
            <span>
                for word in story.title.split_whitespace() {
                    <span>(word)</span>
                }
            </span>
        </div>
        <img
            class="cover-seal"
            src=(asset!("assets/seal-kim-khue.svg"))
            width="44"
            height="44"
            alt=""
            loading="lazy"
        >
        <span class="cover-rule"></span>
    })
}

#[component]
pub async fn story_cover(cx: &Cx, story: &Story, preview: Option<&Chapter>) -> Result<impl View> {
    let opened = signal(cx, || false);
    let preview_id = format!("foreword-{}", story.slug);
    let stage_id = format!("book-stage-{}", story.slug);
    let read_label = format!("{} — {}", I18N.read_story, story.title);
    let hover_start = browser::book_hover(&opened, &stage_id);
    let hover_cancel = browser::cancel_book_hover(&stage_id);
    let close_on_leave = browser::book_hover_device();
    let chapter = preview.filter(|chapter| chapter.published && !chapter.content.trim().is_empty());
    Ok(view! {
        <div
            class=(format!("book-cover cover-{}", story.cover))
            id=(&stage_id)
            style=(format!("--page-slices: {PAGE_SLICES}"))
            role="group"
            aria-label=(&story.title)
            :data-open=$(if opened.get() { "true" } else { "false" })
            @pointerenter=$(|_e: Event| hover_start)
            @pointerleave=$(|_e: Event| {
                hover_cancel;
                if close_on_leave {
                    opened.set(false);
                }
            })
        >
            <div class="book-scene">
                <div
                    class="cover-preview"
                    id=(&preview_id)
                    :aria-hidden=$(if opened.get() { "false" } else { "true" })
                >
                    <span class="cover-preview-label">(I18N.foreword)</span>
                    if let Some(chapter) = chapter {
                        <p class="cover-preview-chapter">
                            (chapter
                                .title
                                .strip_prefix("Khởi đầu · ")
                                .unwrap_or(&chapter.title))
                        </p>
                        <p class="cover-preview-text">
                            (chapter.content.split("\n\n").next().unwrap_or(""))
                        </p>
                    } else {
                        <p class="cover-preview-text">(I18N.no_chapter)</p>
                    }
                </div>
                <div class="turning-cover" aria-hidden="true">
                    for slice in 0..PAGE_SLICES {
                        <div class="page-strip" style=(format!("--slice: {slice}"))>
                            <div class="page-strip-front">
                                <div class="page-strip-artwork">
                                    cover_artwork(story: story)
                                </div>
                            </div>
                            <div class="page-strip-back">
                                <div class="page-strip-endpaper">
                                    <span class="book-endpaper-frame"></span>
                                    <img
                                        src=(asset!("assets/seal-kim-khue.svg"))
                                        width="68"
                                        height="68"
                                        alt=""
                                    >
                                </div>
                            </div>
                        </div>
                    }
                </div>
            </div>
            <a
                class="book-toggle"
                href=(format!("{}#chapter", story_url(&story.slug)))
                aria-label=(&read_label)
                aria-controls=(&preview_id)
                :aria-expanded=$(if opened.get() { "true" } else { "false" })
                @keydown=$(|e: Event| {
                    if e.key == "Escape" {
                        hover_cancel;
                        opened.set(false);
                    }
                })
            >
                <span class="sr-only">(&read_label)</span>
            </a>
        </div>
    })
}
