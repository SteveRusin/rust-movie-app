use actix_web::{error, get, web, HttpResponse, Responder};
use anyhow::Result;
use mongodb::{options::FindOptions, Client, Collection};
use serde::{Deserialize, Serialize};

use crate::{clients::mongo_client::MongoConfig, movie::Movie};
use futures::join;
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
async fn get_movies(
    client: web::Data<Client>,
    params: web::Query<MovieParams>,
) -> actix_web::Result<impl Responder> {
    let config = MongoConfig::build().map_err(error::ErrorInternalServerError)?;
    let movies_collection: Collection<Movie> = client
        .database(&config.database)
        .collection(&config.movies_collection);

    let find_options = FindOptions::builder()
        .sort(mongodb::bson::doc! { "id": 1 })
        .limit(params.get_limit())
        .skip(params.get_skip())
        .build();

    let (result, total_count) = join!(
        get_movies_collection(&movies_collection, find_options),
        get_total_count(&movies_collection)
    );

    let result = result.map_err(error::ErrorInternalServerError)?;
    let total_count = total_count.map_err(error::ErrorInternalServerError)?;

    let result = MoviesReponse {
        movies: result,
        total_count,
    };

    Ok(HttpResponse::Ok().json(result))
}

async fn get_total_count(movies_collection: &Collection<Movie>) -> Result<u64> {
    return Ok(movies_collection
        .count_documents(None, None)
        .await
        .map_err(|e| anyhow::anyhow!("Error getting total count: {}", e))?);
}

async fn get_movies_collection(
    movies_collection: &Collection<Movie>,
    find_options: FindOptions,
) -> Result<Vec<Movie>> {
    return Ok(movies_collection
        .find(None, find_options)
        .await?
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?);
}
