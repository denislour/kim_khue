#[path = "site_footer.rs"]
mod site_footer_component;
#[path = "site_header.rs"]
mod site_header_component;
#[path = "story_card/mod.rs"]
mod story_card_component;

pub use site_footer_component::site_footer;
pub use site_header_component::site_header;
pub use story_card_component::story_card;
