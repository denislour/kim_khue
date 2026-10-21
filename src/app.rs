mod _site;
pub(crate) mod urls;

#[cfg(test)]
mod tests;

use crate::i18n::I18N;
use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt, asset},
    context::Cx,
    router::{Router, RouterBuilder, Slot, layout, module_router},
    runtime::RouterBuilderRuntimeExt,
    view::{View, view},
};

fn router_builder(db: toasty::Db) -> RouterBuilder {
    module_router!().app_context(db)
}

pub fn router(db: toasty::Db) -> Router {
    router_builder(db)
        .assets(AssetBundle::load().unwrap())
        .runtime()
        .build()
}

#[layout]
pub(crate) async fn root(cx: &Cx, slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html lang=(I18N.lang)>
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <meta name="color-scheme" content="light dark">
                <meta name="theme-color" content="#ffffff">
                <meta name="description" content=(I18N.intro)>
                <title>(I18N.title)</title>
                <link
                    rel="icon"
                    type="image/svg+xml"
                    href=(asset!("assets/favicon.svg"))
                >
                <style>(crate::fonts::styles(cx))</style>
                <link rel="stylesheet" href=(topcoat::tailwind::stylesheet!())>
                <link rel="stylesheet" href=(asset!("styles.custom.css"))>

                topcoat::runtime::script()
                topcoat::dev::script()
            </head>
            <body id="top">(slot)</body>
        </html>
    })
}
