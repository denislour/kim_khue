use topcoat::{Result, router::{Router, RouterBuilderDiscoverExt, module_router, page}, view::{View, view}};
use topcoat::asset::{AssetBundle, RouterBuilderAssetExt};
use topcoat::router::{layout, Slot};
use topcoat::runtime::RouterBuilderRuntimeExt;
use crate::i18n::{Locale, Messages, messages};
static TEXT: &Messages = messages(Locale::Vi);
use topcoat::context::Cx;
use topcoat::asset::asset;

pub fn router() -> Router {
    module_router!().discover()
        .assets(AssetBundle::load().unwrap())
        .runtime()
        .build()
}
#[layout]
async fn root(cx: &Cx, slot: Slot<'_>) -> Result<impl View> {
Ok(view! { <!DOCTYPE html><html lang=(TEXT.lang)><head><meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>(TEXT.title)</title>
<link rel="icon" type="image/svg+xml" href=(asset!("assets/logo.svg"))>
<style>(crate::fonts::styles(cx))</style>
<link rel="stylesheet" href=(topcoat::tailwind::stylesheet!())>
<link rel="stylesheet" href=(asset!("styles.custom.css"))>
topcoat::runtime::script()
topcoat::dev::script()
</head><body>(slot)</body></html> })
}
#[page]
async fn home() -> Result<impl View> {
    Ok(view! { <main id="main"></main> })
}
