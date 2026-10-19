use crate::components::badge::{BadgeVariant, badge};
use crate::components::button::{ButtonSize, ButtonVariant, button, button_variants};
use crate::components::dialog::{
    dialog, dialog_content, dialog_description, dialog_header, dialog_title,
};
use crate::components::input::input;
use crate::components::label::label;
use crate::components::separator::separator;
use crate::components::site_header::site_header;
use crate::components::toggle::{ToggleSize, toggle};
use crate::i18n::TEXT;
use topcoat::{
    Result,
    asset::asset,
    context::Cx,
    router::{Slot, layout},
    view::{View, attributes, class, view},
};

#[layout("/")]
async fn root(cx: &Cx, slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html lang=(TEXT.lang)>
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <meta name="theme-color" content="#ffffff">
                <meta name="description" content=(TEXT.intro)>
                <title>(TEXT.title)</title>
                <link rel="icon" type="image/svg+xml" href=(asset!("assets/logo.svg"))>
                <style>(crate::fonts::styles(cx))</style>
                <link rel="stylesheet" href=(topcoat::tailwind::stylesheet!())>
                <link rel="stylesheet" href=(asset!("styles.custom.css"))>

                topcoat::runtime::script()
                topcoat::dev::script()
            </head>
            <body id="top">
                <a class="skip-link" href="#main">(TEXT.skip_content)</a>
                site_header()
                (slot)
                <footer class="site-footer shell">
                    <a class="footer-brand" href="#top" translate="no">(TEXT.brand)</a>
                    <p>(TEXT.footer)</p>
                    <a
                        class=(class!(
                            button_variants(ButtonVariant::Ghost, ButtonSize::Sm),
                            "back-to-top",
                        ))
                        href="#top"
                    >
                        (TEXT.back_top)
                        <span aria-hidden="true">" ↑"</span>
                    </a>
                </footer>
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
                            <span class="reader-brand" translate="no">
                                (TEXT.brand)
                            </span>
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
            </body>
        </html>
    })
}
