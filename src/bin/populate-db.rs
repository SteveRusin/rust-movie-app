use std::{
    fs::{self},
    path::Path,
};

use anyhow::{Context, Result};

use api::clients::mongo_client::{mongo_connect, MongoConfig};
use api::movie::Movie;

#[tokio::main]
async fn main() -> Result<()> {
    let client = mongo_connect().await?;
    let config = MongoConfig::build()?;
    client.database(&config.database).drop(None).await?;

    client
        .database(&config.database)
        .create_collection(&config.movies_collection, None)
        .await?;

    let path_to_mocks = Path::new("src/bin/movies-mock");

    for entry in fs::read_dir(path_to_mocks)? {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path
            .file_name()
            .with_context(|| "error reading path")?
            .to_str()
            .with_context(|| "cannot convert to string")?;

        if file_name.ends_with(".json") {
            let mocks = fs::read_to_string(path.to_str().unwrap())?;
            let movies_json: Vec<Movie> = serde_json::from_str(&mocks)?;

            println!("Inserting {:?} movies", file_name);

            client
                .database(&config.database)
                .collection::<Movie>(&config.movies_collection)
                .insert_many(&movies_json, None)
                .await?;
        }
    }

    println!("Database seeded successfully!");

    Ok(())
}
