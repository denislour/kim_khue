use crate::{
    components::ui::{
        badge::{BadgeVariant, badge},
        dialog::{dialog_description, dialog_header, dialog_title},
        separator::separator,
    },
    i18n::I18N,
};
use topcoat::{
    Result,
    view::{View, attributes, component, view},
};

#[component]
pub async fn reader_content() -> Result<impl View> {
    Ok(view! {
        <div class="reader-page">
            dialog_header(
                attrs: attributes! { class="reader-heading" },
                badge(
                    variant: BadgeVariant::Outline,
                    attrs: attributes! { class="sample-label" },
                    (I18N.sample_badge)
                )
                dialog_title(attrs: attributes! { id="reader-title" }, "")
                dialog_description(
                    attrs: attributes! { id="reader-chapter" class="reader-chapter" },
                    ""
                )
            )
            <div id="reader-content" class="reader-prose"></div>
            separator(attrs: attributes! { class="reader-divider" aria-hidden="true" })
            <p class="reader-end">(I18N.reader_end)</p>
            dialog_description(
                attrs: attributes! { class="reader-notice" },
                (I18N.reader_notice)
            )
        </div>
    })
}
