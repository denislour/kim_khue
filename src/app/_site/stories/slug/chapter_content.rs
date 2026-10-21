use crate::{
    app::urls::chapter_url,
    components::ui::{
        button::{ButtonSize, ButtonVariant, button_variants},
        separator::separator,
    },
    i18n::I18N,
    services::story::Reading,
};
use topcoat::{
    Result,
    view::{View, attributes, class, component, view},
};

#[component]
pub async fn chapter_content(reading: &Reading) -> Result<impl View> {
    let chapter = &reading.chapter;
    let current = reading
        .chapters
        .iter()
        .position(|item| item.position == chapter.position)
        .unwrap_or(0);
    let previous = current
        .checked_sub(1)
        .and_then(|index| reading.chapters.get(index));
    let next = reading.chapters.get(current + 1);
    let title = chapter
        .title
        .strip_prefix("Khởi đầu · ")
        .unwrap_or(&chapter.title);
    Ok(view! {
        <article class="chapter-article" id="chapter" aria-labelledby="chapter-title">
            <header class="chapter-article-heading">
                <p class="chapter-position">
                    (I18N.chapter_label)
                    " "
                    (chapter.position)
                    " / "
                    (reading.total)
                </p>
                <h2 id="chapter-title">(title)</h2>
                <a class="chapter-toc-link" href="#chapters">(I18N.chapter_index)</a>
            </header>
            separator(attrs: attributes! { class="chapter-rule" })
            if !chapter.published || chapter.content.trim().is_empty() {
                <div class="chapter-draft">
                    <p class="chapter-draft-title">(I18N.chapter_draft)</p>
                    <p>(I18N.chapter_draft_note)</p>
                    <a href=(chapter_url(&reading.story.slug, 1))>
                        (I18N.first_chapter)
                    </a>
                </div>
            } else {
                <div class="chapter-prose">
                    for paragraph in chapter
                        .content
                        .split("\n\n")
                        .filter(|text| !text.trim().is_empty()) {
                        <p>(paragraph)</p>
                    }
                </div>
                <p class="chapter-colophon">
                    (I18N.chapter_end)
                    " "
                    (chapter.position)
                </p>
            }
            <nav class="reading-navigation" aria-label=(I18N.chapter_navigation)>
                if let Some(previous) = previous {
                    <a
                        class=(class!(
                            button_variants(ButtonVariant::Outline, ButtonSize::Md),
                            "reading-previous",
                        ))
                        href=(chapter_url(&reading.story.slug, previous.position))
                        rel="prev"
                    >
                        "← "
                        (I18N.previous_chapter)
                    </a>
                }
                if let Some(next) = next {
                    <a
                        class=(class!(
                            button_variants(ButtonVariant::Outline, ButtonSize::Md),
                            "reading-next",
                        ))
                        href=(chapter_url(&reading.story.slug, next.position))
                        rel="next"
                    >
                        (I18N.next_chapter)
                        " →"
                    </a>
                }
            </nav>
        </article>
    })
}
