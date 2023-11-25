use anyhow::{anyhow, Result};
use std::env;

#[derive(Debug)]
pub struct MongoConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    pub movies_collection: String,
}

static mut MONGO_CONFIG: Option<MongoConfig> = None;

impl MongoConfig {
    pub fn build() -> Result<&'static Self> {
        unsafe {
            if MONGO_CONFIG.is_none() {
                MONGO_CONFIG = Some(Self {
                    host: env::var("MONGO_HOST")?,
                    port: env::var("MONGO_PORT")?.parse()?,
                    username: env::var("MONGO_USERNAME")?,
                    password: env::var("MONGO_PASSWORD")?,
                    database: env::var("MONGO_DATABASE")?,
                    movies_collection: "movies".to_owned(),
                });
            }

            return MONGO_CONFIG
                .as_ref()
                .ok_or(anyhow!("Error getting mongo config"));
        }
    }
}
