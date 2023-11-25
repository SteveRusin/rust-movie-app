use anyhow::Result;
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client};

// reexport for convenience
pub use crate::config::MongoConfig;

pub async fn mongo_connect() -> Result<Client> {
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

    return Ok(client);
}
