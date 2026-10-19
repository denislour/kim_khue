use crate::{
    components::{
        accordion::{accordion, accordion_content, accordion_item, accordion_trigger},
        badge::{BadgeVariant, badge},
        card::{card, card_description, card_footer, card_header, card_title},
    },
    features::Story,
    i18n::TEXT,
};
use topcoat::{
    Result,
    asset::{Asset, asset},
    view::{View, attributes, component, view},
};

fn cover(name: &str) -> Asset {
    match name {
        "mai" => asset!("assets/hoa-mai.svg"),
        "moon" => asset!("assets/moon.svg"),
        _ => asset!("assets/landscape.svg"),
    }
}

#[component]
pub async fn story_card(story: &Story) -> Result<impl View> {
    Ok(view! {
        card(
            attrs: attributes! {
                class="story"
                id=(format!("story-{}", story.id))
                data-genre=(story.genre_key)
                role="article"
                aria-labelledby=(format!("title-{}", story.id))
            },
            <div class=(format!("book-cover cover-{}", story.cover)) aria-hidden="true">
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
    })
}
