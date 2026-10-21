#[path = "story_card/story_cover.rs"]
mod story_cover_component;

use story_cover_component::story_cover;

use crate::{
    app::urls::story_url,
    components::ui::{
        badge::{BadgeVariant, badge},
        card::{card, card_description, card_header, card_title},
    },
    i18n::I18N,
    model::{Chapter, Story},
};
use topcoat::{
    Result,
    view::{View, attributes, component, view},
};

#[component]
pub async fn story_card(story: &Story, preview: Option<&Chapter>) -> Result<impl View> {
    Ok(view! {
        card(
            attrs: attributes! {
                class="story"
                id=(format!("story-{}", story.slug))
                data-genre=(&story.genre_key)
                role="article"
                aria-labelledby=(format!("title-{}", story.id))
            },
            story_cover(story: story, preview: preview)
            card_header(
                attrs: attributes! { class="book-copy" },
                <div class="book-meta">
                    badge(
                        variant: BadgeVariant::Outline,
                        attrs: attributes! { class="genre-badge" },
                        (I18N.genre_label(&story.genre_key))
                    )
                </div>
                card_title(
                    attrs: attributes! { class="story-title" id=(format!("title-{}", story.id)) },
                    <a href=(format!("{}#chapter", story_url(&story.slug)))>
                        (&story.title)
                    </a>
                )
                card_description(
                    attrs: attributes! { class="story-summary" },
                    (&story.summary)
                )
            )
        )
    })
}
