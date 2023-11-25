use actix_web::{error, get, web, HttpResponse, Responder};
use mongodb::{options::FindOptions, Client, Collection};

use crate::{clients::mongo_client::MongoConfig, movie::Movie};
use futures::stream::StreamExt;

#[get("/movies")]
async fn get_movies(client: web::Data<Client>) -> actix_web::Result<impl Responder> {
    let config = MongoConfig::build().map_err(error::ErrorInternalServerError)?;
    let movies_collection: Collection<Movie> = client
        .database(&config.database)
        .collection(&config.movies_collection);

    let find_options = FindOptions::builder()
        .sort(mongodb::bson::doc! { "id": 1 })
        .build();

    let result = movies_collection
        .find(None, find_options)
        .await
        .map_err(error::ErrorInternalServerError)?
        .collect::<Vec<_>>()
        .await;

    // todo add pagination

    let result: Result<Vec<Movie>, _> = result.into_iter().collect();
    let result = result.map_err(error::ErrorInternalServerError)?;

    println!("Found {} movies", result.len());

    Ok(HttpResponse::Ok().json(result))
}
