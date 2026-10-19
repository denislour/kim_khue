mod app;
mod components;
mod features;
mod fonts;
mod i18n;

#[tokio::main]
async fn main() {
    topcoat::start(app::router()).await.unwrap();
}
