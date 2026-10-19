mod app;
mod components;
mod config;
mod db;
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
async fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let db = db::connect(&config.database_url).await?;
    topcoat::start(app::router(db)).await?;
    Ok(())
}
