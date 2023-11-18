use anyhow::Result;
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

impl MongoConfig {
    pub fn build() -> Result<Self> {
        Ok(Self {
            host: env::var("MONGO_HOST")?,
            port: env::var("MONGO_PORT")?.parse()?,
            username: env::var("MONGO_USERNAME")?,
            password: env::var("MONGO_PASSWORD")?,
            database: env::var("MONGO_DATABASE")?,
            movies_collection: "movies".to_owned(),
        })
    }
}
