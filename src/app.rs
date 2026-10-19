use topcoat::{Result, router::{Router, RouterBuilderDiscoverExt, module_router, page}, view::{View, view}};

pub fn router() -> Router {
    module_router!().discover()
        .build()
}
#[page]
async fn home() -> Result<impl View> {
    Ok(view! { <main id="main"></main> })
}
