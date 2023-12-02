use actix_web::{error, get, web, HttpResponse, Responder};
use mongodb::{options::FindOptions, Client, Collection};
use serde::{Deserialize, Serialize};

use crate::{clients::mongo_client::MongoConfig, movie::Movie};
use futures::stream::StreamExt;

#[derive(Serialize, Deserialize, Debug)]
struct MoviesReponse {
    movies: Vec<Movie>,
    total_count: u64,
}

#[derive(Deserialize, Debug, Serialize)]
struct MovieParams {
    limit: Option<i64>,
    skip: Option<u64>,
}

impl MovieParams {
    fn get_limit(&self) -> i64 {
        self.limit.unwrap_or(10)
    }

    fn get_skip(&self) -> u64 {
        self.skip.unwrap_or(0)
    }
}

#[get("/movies")]
async fn get_movies(client: web::Data<Client>, params: web::Query<MovieParams>) -> actix_web::Result<impl Responder> {
    let config = MongoConfig::build().map_err(error::ErrorInternalServerError)?;
    let movies_collection: Collection<Movie> = client
        .database(&config.database)
        .collection(&config.movies_collection);

    let find_options = FindOptions::builder()
        .sort(mongodb::bson::doc! { "id": 1 })
        .limit(params.get_limit())
        .skip(params.get_skip())
        .build();

    let result = movies_collection
        .find(None, find_options)
        .await
        .map_err(error::ErrorInternalServerError)?
        .collect::<Vec<_>>()
        .await;

    let total_count = movies_collection
        .count_documents(None, None)
        .await
        .map_err(error::ErrorInternalServerError)?;

    let result: Result<Vec<Movie>, _> = result.into_iter().collect();
    let result = result.map_err(error::ErrorInternalServerError)?;
    let result = MoviesReponse {
        movies: result,
        total_count,
    };

    Ok(HttpResponse::Ok().json(result))
}
