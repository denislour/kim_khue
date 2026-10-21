#[path = "chapter_content.rs"]
mod chapter_content_component;
#[path = "chapter_index.rs"]
mod chapter_index_component;

pub use chapter_content_component::chapter_content;
pub use chapter_index_component::chapter_index;
