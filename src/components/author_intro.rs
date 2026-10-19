use crate::{
    components::card::{card, card_content, card_description},
    i18n::I18N,
};
use topcoat::{
    Result,
    asset::asset,
    view::{View, attributes, component, view},
};

#[component]
pub async fn author_intro() -> Result<impl View> {
    Ok(view! {
        <section class="shell" id="about" aria-labelledby="about-heading">
            card(
                attrs: attributes! { class="about" },
                card_content(
                    attrs: attributes! { class="about-art" aria-hidden="true" },
                    <img
                        src=(asset!("assets/logo.svg"))
                        width="160"
                        height="160"
                        alt=""
                    >
                    <span>(I18N.brand)</span>
                )
                card_content(
                    attrs: attributes! { class="about-copy" },
                    <h2 id="about-heading">(I18N.about_heading)</h2>
                    card_description((I18N.about_text))
                    <p class="author-note">(I18N.about_note)</p>
                )
            )
        </section>
    })
}
