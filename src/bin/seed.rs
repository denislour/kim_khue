use kim_khue::{config::Config, db, seed};
use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    let initialize = match args.as_slice() {
        [] => false,
        [arg] if arg == "--init" => true,
        [arg] if arg == "--help" || arg == "-h" => {
            println!("Usage: cargo run --bin seed -- [--init]");
            println!("  --init  Create tables in a new database before seeding.");
            return Ok(());
        }
        _ => return Err("Usage: cargo run --bin seed -- [--init]".into()),
    };

    run(Config::load()?, initialize)
}

#[tokio::main]
async fn run(config: Config, initialize: bool) -> Result<(), Box<dyn Error>> {
    let mut db = db::connect(&config.database_url).await?;

    if initialize {
        db.push_schema().await?;
    }

    let inserted = seed::run(&mut db).await?;
    println!("Đã thêm {inserted} truyện mẫu.");
    Ok(())
}
