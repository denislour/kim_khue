use kim_khue::{config::Config, db, seed};
use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.as_slice() {
        [] => (),
        [arg] if arg == "--init" => (),
        [arg] if arg == "--help" || arg == "-h" => {
            println!("Usage: cargo run --bin seed -- [--init]");
            println!("  --init  Ensure tables exist before seeding (also done by connect).");
            return Ok(());
        }
        _ => return Err("Usage: cargo run --bin seed -- [--init]".into()),
    };

    run(Config::load()?)
}

#[tokio::main]
async fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut db = db::connect(&config.database_url).await?;

    let inserted = seed::run(&mut db).await?;
    println!("Đã cập nhật dữ liệu seed; thêm mới {inserted} truyện.");
    Ok(())
}
