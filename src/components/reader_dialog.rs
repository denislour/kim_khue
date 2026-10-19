use crate::{
    components::{
        badge::{BadgeVariant, badge},
        button::{ButtonSize, ButtonVariant, button},
        dialog::{dialog, dialog_content, dialog_description, dialog_header, dialog_title},
        input::input,
        label::label,
        separator::separator,
        toggle::{ToggleSize, toggle},
    },
    i18n::I18N,
};
use topcoat::{
    Result,
    view::{View, attributes, component, view},
};

#[component]
pub async fn reader_dialog() -> Result<impl View> {
    Ok(view! {
        dialog(
            open: false,
            attrs: attributes! {
                class="reader"
                id="reader"
                aria-labelledby="reader-title"
            },
            dialog_content(
                attrs: attributes! { class="reader-panel" },
                <div class="reader-toolbar">
                    <span class="reader-brand" translate="no">(I18N.brand)</span>
                    <div class="reader-settings">
                        label(
                            attrs: attributes! { for="reader-size" },
                            (I18N.font_size)
                        )
                        input(
                            attrs: attributes! {
                                id="reader-size"
                                class="reader-size"
                                aria-label=(I18N.font_size)
                                type="range"
                                min="16"
                                max="26"
                                value="20"
                            }
                        )
                        toggle(
                            size: ToggleSize::Sm,
                            attrs: attributes! {
                                id="reader-theme"
                                name="reader-night"
                                class="reader-theme"
                            },
                            (I18N.night_mode)
                        )
                        button(
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Icon,
                            attrs: attributes! {
                                id="reader-close"
                                class="close-reader"
                                type="button"
                                aria-label=(I18N.close_reader)
                            },
                            "×"
                        )
                    </div>
                </div>
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
                    separator(
                        attrs: attributes! { class="reader-divider" aria-hidden="true" }
                    )
                    <p class="reader-end">(I18N.reader_end)</p>
                    dialog_description(
                        attrs: attributes! { class="reader-notice" },
                        (I18N.reader_notice)
                    )
                </div>
            )
        )
    })
}
