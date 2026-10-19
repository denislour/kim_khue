mod app;
mod components;
mod features;
mod fonts;
mod i18n;
mod model;

#[tokio::main]
async fn main() {
    topcoat::start(app::router()).await.unwrap();
}
