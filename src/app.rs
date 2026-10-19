use topcoat::{Result, router::{Router, RouterBuilderDiscoverExt, module_router, page}, view::{View, view}};
use topcoat::asset::{AssetBundle, RouterBuilderAssetExt};

pub fn router() -> Router {
    module_router!().discover()
        .assets(AssetBundle::load().unwrap())
        .build()
}
#[page]
async fn home() -> Result<impl View> {
    Ok(view! { <main id="main"></main> })
}
