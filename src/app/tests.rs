use super::*;
use crate::model::{Chapter, Story};
use std::sync::OnceLock;
use topcoat::{
    asset::{AssetConfig, MANIFEST_VERSION, Manifest, ManifestEntry, RawAsset},
    router::{Body, HeaderMap, Method, StatusCode, request::Request, to_bytes},
};

const STORY: &str = "ben-cu-dong-ngan";
const PUBLIC_PROSE: &str = "HTTP regression published prose";
const DRAFT_PROSE: &str = "HTTP_SECRET_DRAFT_DO_NOT_DISCLOSE";

fn test_assets() -> AssetConfig {
    static ASSETS: OnceLock<AssetConfig> = OnceLock::new();
    ASSETS
        .get_or_init(|| {
            // Discover the actual declarations, including runtime and Tailwind assets.
            // A catalog is sufficient for rendering: no bundling, network, or files in
            // target/debug/assets are needed. These URLs are never fetched by tests.
            let binary = std::fs::read(std::env::current_exe().unwrap()).unwrap();
            let assets = RawAsset::find_in_binary(&binary)
                .into_iter()
                .map(|asset| ManifestEntry {
                    id: asset.id(),
                    file: format!("test-{:016x}.bin", asset.id().as_u64()),
                    hash: "test".into(),
                    content_type: "application/octet-stream".into(),
                })
                .collect::<Vec<_>>();
            assert!(
                !assets.is_empty(),
                "test executable must retain asset declarations"
            );
            AssetConfig::hosted_at(
                "/test-assets",
                Manifest {
                    version: MANIFEST_VERSION,
                    assets,
                },
            )
        })
        .clone()
}

fn test_router(db: toasty::Db) -> Router {
    router_builder(db).assets(test_assets()).runtime().build()
}

async fn fixture() -> toasty::Db {
    let mut db = crate::db::connect("sqlite::memory:").await.unwrap();
    Story::create()
        .slug(STORY)
        .title("HTTP regression story")
        .genre_key("wuxia")
        .cover("river")
        .summary("HTTP regression summary")
        .exec(&mut db)
        .await
        .unwrap();
    for position in 1..=2u64 {
        Chapter::create()
            .slug(format!("{STORY}-{position}"))
            .story_slug(STORY)
            .position(position)
            .title(format!("HTTP chapter {position}"))
            .content(PUBLIC_PROSE)
            .published(true)
            .exec(&mut db)
            .await
            .unwrap();
    }
    db
}

async fn dispatch(router: &Router, method: Method, uri: &str) -> (StatusCode, HeaderMap, String) {
    let response = router
        .handle(
            Request::builder()
                .method(method)
                .uri(uri)
                .header("host", "localhost")
                .body(Body::empty())
                .unwrap(),
        )
        .await;
    let (parts, body) = response.into_parts();
    // Collect the streaming body so render-time panics fail the test as well.
    let bytes = to_bytes(body, 4 * 1024 * 1024).await.unwrap();
    (
        parts.status,
        parts.headers,
        String::from_utf8(bytes.to_vec()).unwrap(),
    )
}

fn assert_layout(html: &str) {
    let lowercase = html.to_ascii_lowercase();
    for marker in [
        "<!doctype html>",
        "<html ",
        "</html>",
        "<head>",
        "</head>",
        "<body ",
        "</body>",
    ] {
        assert_eq!(
            lowercase.matches(marker).count(),
            1,
            "root layout marker {marker}"
        );
    }
    for marker in [
        "class=\"site-header shell\"",
        "class=\"site-footer shell\"",
        "id=\"main\"",
    ] {
        assert_eq!(
            html.matches(marker).count(),
            1,
            "site layout marker {marker}"
        );
    }
    assert!(
        html.contains("/test-assets/test-"),
        "assets must resolve from the test catalog"
    );
}

#[tokio::test]
async fn home_and_story_render_the_real_nested_layouts_once() {
    let router = test_router(fixture().await);
    for uri in [
        "/",
        "/stories/ben-cu-dong-ngan",
        "/stories/ben-cu-dong-ngan?chapter=2",
    ] {
        let (status, headers, html) = dispatch(&router, Method::GET, uri).await;
        assert_eq!(status, StatusCode::OK, "{uri}: {html}");
        assert!(
            headers["content-type"]
                .to_str()
                .unwrap()
                .starts_with("text/html")
        );
        assert_layout(&html);
        assert!(html.contains("HTTP regression story"));
        assert!(
            html.contains(PUBLIC_PROSE),
            "published prose/preview missing from {uri}"
        );
    }
}

#[tokio::test]
async fn legacy_slug_redirects_to_the_selected_canonical_chapter() {
    let router = test_router(fixture().await);
    let (status, headers, _) = dispatch(&router, Method::GET, "/stories/river?chapter=2").await;
    assert_eq!(status, StatusCode::PERMANENT_REDIRECT);
    assert_eq!(
        headers["location"],
        "/stories/ben-cu-dong-ngan?chapter=2#chapter"
    );
}

#[tokio::test]
async fn missing_stories_and_chapters_and_invalid_queries_have_http_errors() {
    let router = test_router(fixture().await);
    for (uri, expected) in [
        ("/stories/missing", StatusCode::NOT_FOUND),
        (
            "/stories/ben-cu-dong-ngan?chapter=99",
            StatusCode::NOT_FOUND,
        ),
        ("/stories/ben-cu-dong-ngan?chapter=0", StatusCode::NOT_FOUND),
        (
            "/stories/ben-cu-dong-ngan?chapter=abc",
            StatusCode::BAD_REQUEST,
        ),
    ] {
        let (status, _, html) = dispatch(&router, Method::GET, uri).await;
        assert_eq!(status, expected, "{uri}: {html}");
    }
}

#[tokio::test]
async fn route_groups_and_component_modules_are_not_public_paths() {
    let router = test_router(fixture().await);
    for uri in [
        "/_site",
        "/_home",
        "/home",
        "/_site/stories/ben-cu-dong-ngan",
        "/components",
        "/shared",
        "/components/shared",
        "/stories",
        "/truyen/ben-cu-dong-ngan",
    ] {
        let (status, _, html) = dispatch(&router, Method::GET, uri).await;
        assert_eq!(status, StatusCode::NOT_FOUND, "{uri}: {html}");
    }
}

#[tokio::test]
async fn catalog_pagination_preserves_filters_and_total_match_count() {
    let mut db = fixture().await;
    for index in 0..crate::services::story::PAGE_SIZE {
        Story::create()
            .slug(format!("catalog-{index}"))
            .title(format!("HTTP catalog item {index}"))
            .genre_key("wuxia")
            .cover("river")
            .summary("Pagination fixture")
            .exec(&mut db)
            .await
            .unwrap();
    }
    let router = test_router(db);
    let (status, _, first) = dispatch(&router, Method::GET, "/?q=HTTP&genre=wuxia").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(first.matches("role=\"article\"").count(), 24);
    assert!(first.contains("/?q=HTTP&amp;genre=wuxia&amp;page=2#library"));
    assert!(first.contains(">25 tác phẩm</output>"));
    let (status, _, second) = dispatch(&router, Method::GET, "/?q=HTTP&genre=wuxia&page=2").await;
    assert_eq!(status, StatusCode::OK);
    assert_layout(&second);
    assert_eq!(second.matches("role=\"article\"").count(), 1);
    assert!(second.contains("HTTP catalog item 23"));
    assert!(!second.contains("HTTP regression story"));
    assert!(second.contains("/?q=HTTP&amp;genre=wuxia#library"));
    assert!(second.contains(">25 tác phẩm</output>"));
    let (status, _, empty) = dispatch(&router, Method::GET, "/?q=HTTP&genre=wuxia&page=99").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(empty.matches("role=\"article\"").count(), 0);
    assert!(empty.contains(crate::i18n::I18N.page_empty));
    for uri in ["/?page=abc", "/?page=-1"] {
        assert_eq!(
            dispatch(&router, Method::GET, uri).await.0,
            StatusCode::BAD_REQUEST
        );
    }
    assert_eq!(
        dispatch(&router, Method::GET, "/?page=0").await.0,
        StatusCode::OK
    );
}

#[tokio::test]
async fn pages_reject_post_requests() {
    let router = test_router(fixture().await);
    for uri in ["/", "/stories/ben-cu-dong-ngan"] {
        let (status, headers, html) = dispatch(&router, Method::POST, uri).await;
        assert_eq!(status, StatusCode::METHOD_NOT_ALLOWED, "{uri}: {html}");
        assert!(
            headers["allow"]
                .to_str()
                .unwrap()
                .split(',')
                .any(|method| method.trim() == "GET")
        );
    }
}

#[tokio::test]
async fn written_unpublished_body_is_absent_from_reader_and_home_preview() {
    let mut db = fixture().await;
    toasty::sql::statement("UPDATE chapters SET published = FALSE, content = ?1 WHERE story_slug = ?2 AND position = 1")
        .bind(DRAFT_PROSE)
        .bind(STORY)
        .exec(&mut db)
        .await
        .unwrap();
    let persisted = Chapter::all()
        .filter(Chapter::fields().story_slug().eq(STORY))
        .filter(Chapter::fields().position().eq(1u64))
        .first()
        .exec(&mut db)
        .await
        .unwrap()
        .unwrap();
    assert!(!persisted.published);
    assert_eq!(persisted.content, DRAFT_PROSE);

    let router = test_router(db);
    for uri in [
        "/",
        "/stories/ben-cu-dong-ngan",
        "/stories/ben-cu-dong-ngan?chapter=1",
        "/stories/ben-cu-dong-ngan?chapter=2",
    ] {
        let (status, _, html) = dispatch(&router, Method::GET, uri).await;
        assert_eq!(status, StatusCode::OK, "{uri}: {html}");
        assert_layout(&html);
        assert!(!html.contains(DRAFT_PROSE), "draft body leaked in {uri}");
        if uri.starts_with("/stories/") && !uri.ends_with("chapter=2") {
            assert!(html.contains("class=\"chapter-draft\""));
            assert!(!html.contains("class=\"chapter-prose\""));
        } else if uri.ends_with("chapter=2") {
            assert!(html.contains(PUBLIC_PROSE));
        }
    }
}
