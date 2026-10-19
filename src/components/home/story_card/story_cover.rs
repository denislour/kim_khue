use crate::{features::Story, i18n::I18N};
use topcoat::{
    Result,
    asset::{Asset, asset},
    view::{View, component, view},
};

fn cover(name: &str) -> Asset {
    match name {
        "mai" => asset!("assets/hoa-mai.svg"),
        "moon" => asset!("assets/moon.svg"),
        _ => asset!("assets/landscape.svg"),
    }
}

#[component]
pub async fn story_cover(story: &Story) -> Result<impl View> {
    Ok(view! {
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
                <span class="cover-author">(I18N.brand)</span>
            </div>
            <span class="cover-seal">"KK"</span>
            <span class="cover-rule"></span>
        </div>
    })
}
