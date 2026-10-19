use kimkhue::{config::Config, db, router};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    run(Config::load()?)
}

#[tokio::main]
async fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let db = db::connect(&config.database_url).await?;
    topcoat::start(router(db)).await?;
    Ok(())
}
