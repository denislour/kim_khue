use crate::{
    app::urls::chapter_url,
    browser,
    components::ui::accordion::{accordion_content, accordion_item, accordion_trigger},
    i18n::I18N,
    services::story::ChapterLink,
};
use topcoat::{
    Result,
    context::Cx,
    runtime::signal,
    view::{View, attributes, component, view},
};

#[component]
pub async fn chapter_index(
    cx: &Cx,
    slug: &str,
    chapters: &[ChapterLink],
    current: u64,
) -> Result<impl View> {
    let initial_open = signal(cx, || true);
    Ok(view! {
        <aside class="chapter-sidebar" id="chapters">
            accordion_item(
                attrs: attributes! {
                    class="chapter-picker"
                    :open=(browser::chapter_index_open(&initial_open))
                },
                accordion_trigger(
                    attrs: attributes! { class="chapter-picker-heading" },
                    (I18N.chapter_index)
                    <span class="chapter-picker-count">(chapters.len())</span>
                )
                accordion_content(
                    attrs: attributes! { class="chapter-picker-content" },
                    <nav aria-label=(I18N.chapter_index)>
                        <ol class="chapter-list">
                            for chapter in chapters {
                                <li>
                                    <a
                                        href=(chapter_url(slug, chapter.position))
                                        aria-current=(if chapter.position == current {
                                            Some("page")
                                        } else {
                                            None
                                        })
                                    >
                                        <span class="chapter-number">
                                            (format!("{:02}", chapter.position))
                                        </span>
                                        <span class="chapter-list-title">
                                            (&chapter.title)
                                            if !chapter.available {
                                                <small>(I18N.chapter_draft)</small>
                                            }
                                        </span>
                                    </a>
                                </li>
                            }
                        </ol>
                    </nav>
                )
            )
        </aside>
    })
}
