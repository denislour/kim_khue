//! Web navigation belongs to the app layer, not database services.
use std::fmt::Write;

fn encode_component(value: &str) -> String {
    let mut encoded = String::new();
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || b"-._~".contains(&byte) {
            encoded.push(char::from(byte));
        } else {
            write!(encoded, "%{byte:02X}").expect("writing to a String cannot fail");
        }
    }
    encoded
}

pub(crate) fn story_url(slug: &str) -> String {
    format!("/stories/{}", encode_component(slug))
}

pub(crate) fn chapter_url(slug: &str, position: u64) -> String {
    format!("{}?chapter={position}#chapter", story_url(slug))
}

pub(crate) fn library_url(query: &str, genre: &str, page: usize) -> String {
    let mut params = Vec::new();
    if !query.trim().is_empty() {
        params.push(format!("q={}", encode_component(query.trim())));
    }
    if genre != "all" {
        params.push(format!("genre={}", encode_component(genre)));
    }
    if page > 1 {
        params.push(format!("page={page}"));
    }
    if params.is_empty() {
        "/#library".to_owned()
    } else {
        format!("/?{}#library", params.join("&"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn navigation_encodes_values_and_preserves_chapter_positions() {
        assert_eq!(chapter_url("river", 2), "/stories/river?chapter=2#chapter");
        assert_eq!(story_url("a/b?#"), "/stories/a%2Fb%3F%23");
        assert_eq!(library_url("", "all", 1), "/#library");
        assert_eq!(
            library_url(" Đèn & ", "wuxia", 2),
            "/?q=%C4%90%C3%A8n%20%26&genre=wuxia&page=2#library"
        );
    }
}
