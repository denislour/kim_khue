#[path = "shared/site_footer.rs"]
mod site_footer_component;
#[path = "shared/site_header.rs"]
mod site_header_component;

pub(crate) use site_footer_component::site_footer;
pub(crate) use site_header_component::site_header;
