use serde::{Deserialize, Serialize};
use std::{
    fs::{self},
    path::Path,
};

use anyhow::Result;
use dotenv::dotenv;
use mongodb::{bson::doc, options::ClientOptions, Client};

use api::config::MongoConfig;

#[derive(Debug, Serialize, Deserialize)]
struct Movie {
    title: String,
    year: String,
    runtime: String,
    poster: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let config = MongoConfig::build()?;

    let client_options = ClientOptions::parse(format!(
        "mongodb://{username}:{password}@{host}:{port}",
        username = config.username,
        password = config.password,
        host = config.host,
        port = config.port
    ))
    .await?;

    let client = Client::with_options(client_options)?;

    client.database(&config.database).drop(None).await?;

    client
        .database(&config.database)
        .create_collection(&config.movies_collection, None)
        .await?;

    let path = Path::new("src/bin/movies-seed.json");

    let movies = fs::read_to_string(path)?;
    let movies_json = serde_json::from_str::<Vec<Movie>>(&movies)?;

    client
        .database(&config.database)
        .collection(&config.movies_collection)
        .insert_many(movies_json, None)
        .await?;

    println!("Database seeded successfully!");

    Ok(())
}
