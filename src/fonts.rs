//! The site's single OFL typeface, including Latin, Vietnamese and combining marks.
use topcoat::{
    asset::{AssetConfig, asset},
    context::Cx,
    context::try_app_context,
    view::Unescaped,
};

pub fn styles(cx: &Cx) -> Unescaped<String> {
    let config = try_app_context::<AssetConfig>(cx).expect("asset bundle is registered");
    let url = config.resolve(asset!(
        "assets/fonts/cormorantgaramond-vietnamese-latin.woff2"
    ));
    // Fixed CSS and a trusted, local asset URL; no user content is interpolated.
    Unescaped::new_unchecked(format!(
        "@font-face{{font-family:'Cormorant Garamond';font-style:normal;font-weight:400 600;font-display:swap;src:url({url}) format('woff2');}}"
    ))
}
