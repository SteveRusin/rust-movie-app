use std::{
    fs::{self},
    path::Path,
};

use anyhow::{Context, Result};

use api::clients::mongo_client::{mongo_connect, MongoConfig};
use api::movie::Movie;
use mongodb::Database;

#[tokio::main]
async fn main() -> Result<()> {
    let client = mongo_connect().await?;
    let config = MongoConfig::build()?;
    let db = client.database(&config.database);
    db.drop(None).await?;

    populate_movies(&db).await?;
    create_users_collection(&db).await?;

    println!("Database seeded successfully!");

    Ok(())
}

async fn create_users_collection(db: &Database) -> Result<()> {
    let config = MongoConfig::build()?;

    db.create_collection(&config.users_collection, None)
        .await?;

    Ok(())
}

async fn populate_movies(db: &Database) -> Result<()> {
    let path_to_mocks = Path::new("src/bin/movies-mock");
    let config = MongoConfig::build()?;

    db.create_collection(&config.movies_collection, None)
        .await?;

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

            db.collection::<Movie>(&config.movies_collection)
                .insert_many(&movies_json, None)
                .await?;
        }
    }

    Ok(())
}
