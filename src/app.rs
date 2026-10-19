mod _home;
mod _layout;

use topcoat::{
    asset::{AssetBundle, RouterBuilderAssetExt},
    router::{Router, RouterBuilderDiscoverExt, module_router},
    runtime::RouterBuilderRuntimeExt,
};

pub fn router(db: toasty::Db) -> Router {
    module_router!()
        .discover()
        .app_context(db)
        .assets(AssetBundle::load().unwrap())
        .runtime()
        .build()
}
