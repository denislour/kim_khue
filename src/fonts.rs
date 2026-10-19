//! Self-hosted OFL fonts. AssetConfig resolves Topcoat's bundled font URLs.
use topcoat::{
    asset::{AssetConfig, asset},
    context::{Cx, try_app_context},
    view::Unescaped,
};

pub fn styles(cx: &Cx) -> Unescaped<String> {
    let config = try_app_context::<AssetConfig>(cx).expect("asset bundle is registered");
    let mut css = String::new();
    css.push_str("@font-face{font-family:KKDisplay;font-style:normal;font-weight:400 600;font-display:swap;src:url(");
    css.push_str(&config.resolve(asset!(
        "assets/fonts/cormorantgaramond-vietnamese-400-600.woff2"
    )));
    css.push_str(");unicode-range:U+0102-0103, U+0110-0111, U+0128-0129, U+0168-0169, U+01A0-01A1, U+01AF-01B0, U+0300-0301, U+0303-0304, U+0308-0309, U+0323, U+0329, U+1EA0-1EF9, U+20AB;}");
    css.push_str("@font-face{font-family:KKDisplay;font-style:normal;font-weight:400 600;font-display:swap;src:url(");
    css.push_str(&config.resolve(asset!("assets/fonts/cormorantgaramond-latin-400-600.woff2")));
    css.push_str(");unicode-range:U+0000-00FF, U+0131, U+0152-0153, U+02BB-02BC, U+02C6, U+02DA, U+02DC, U+0304, U+0308, U+0329, U+2000-206F, U+20AC, U+2122, U+2191, U+2193, U+2212, U+2215, U+FEFF, U+FFFD;}");
    // Only fixed CSS and trusted, local asset URLs are used here.
    Unescaped::new_unchecked(css)
}
