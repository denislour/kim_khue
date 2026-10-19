use crate::components::reader_dialog::reader_dialog;
use crate::components::site_footer::site_footer;
use crate::components::site_header::site_header;
use crate::i18n::TEXT;
use topcoat::{
    Result,
    asset::asset,
    context::Cx,
    router::{Slot, layout},
    view::{View, view},
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
                site_footer()
                reader_dialog()
            </body>
        </html>
    })
}
