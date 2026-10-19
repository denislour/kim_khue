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
    i18n::TEXT,
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
                    <span class="reader-brand" translate="no">(TEXT.brand)</span>
                    <div class="reader-settings">
                        label(
                            attrs: attributes! { for="reader-size" },
                            (TEXT.font_size)
                        )
                        input(
                            attrs: attributes! {
                                id="reader-size"
                                class="reader-size"
                                aria-label=(TEXT.font_size)
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
                            (TEXT.night_mode)
                        )
                        button(
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Icon,
                            attrs: attributes! {
                                id="reader-close"
                                class="close-reader"
                                type="button"
                                aria-label=(TEXT.close_reader)
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
                            (TEXT.sample_badge)
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
                    <p class="reader-end">(TEXT.reader_end)</p>
                    dialog_description(
                        attrs: attributes! { class="reader-notice" },
                        (TEXT.reader_notice)
                    )
                </div>
            )
        )
    })
}
