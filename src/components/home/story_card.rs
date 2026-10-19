mod story_cover;
mod story_excerpt;

use crate::{
    components::ui::{
        badge::{BadgeVariant, badge},
        card::{card, card_description, card_footer, card_header, card_title},
    },
    features::Story,
    i18n::I18N,
};
use topcoat::{
    Result,
    view::{View, attributes, component, view},
};

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
            story_cover::story_cover(story: story)
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
                        (I18N.sample_badge)
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
                story_excerpt::story_excerpt(story: story)
            )
        )
    })
}
