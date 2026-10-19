mod app;
mod components;
mod config;
mod features;
mod fonts;
mod i18n;
mod model;

use config::Config;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    run(Config::load()?)
}

#[tokio::main]
async fn run(_config: Config) -> Result<(), Box<dyn Error>> {
    topcoat::start(app::router()).await?;
    Ok(())
}
