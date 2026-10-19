use std::{env, error::Error};

pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        if let Err(error) = dotenvy::dotenv() {
            if !error.not_found() {
                return Err(error.into());
            }
        }

        Ok(Self {
            database_url: env::var("DATABASE_URL")?,
        })
    }
}
