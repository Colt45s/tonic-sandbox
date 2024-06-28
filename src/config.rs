use anyhow::Result;
use std::env;

pub struct Config {
    // pub database_url: String,
    pub port: u16,
}

impl Config {
    pub fn load() -> Result<Self> {
        // let database_url = env::var("DATABASE_URL")?;

        let port = env::var("PORT")
            .map(|t| t.parse::<u16>().expect("Invalid port"))
            .unwrap_or(50051);
        let config = Self {
            // database_url,
            port,
        };
        Ok(config)
    }
}
