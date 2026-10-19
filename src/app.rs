use topcoat::{Result, router::{Router, RouterBuilderDiscoverExt, module_router, page}, view::{View, view}};
use topcoat::asset::{AssetBundle, RouterBuilderAssetExt};
use topcoat::router::{layout, Slot};

pub fn router() -> Router {
    module_router!().discover()
        .assets(AssetBundle::load().unwrap())
        .build()
}
#[layout]
async fn root(slot: Slot<'_>) -> Result<impl View> {
Ok(view! { <!DOCTYPE html><html lang="vi"><head><meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>"Kim Khuê"</title>
<link rel="stylesheet" href=(topcoat::tailwind::stylesheet!())>
</head><body>(slot)</body></html> })
}
#[page]
async fn home() -> Result<impl View> {
    Ok(view! { <main id="main"></main> })
}
