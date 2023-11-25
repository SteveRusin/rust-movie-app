use actix_web::{middleware::Logger, web, App, HttpServer};
use anyhow::Result;
use api::clients::mongo_client::mongo_connect;
use api::controllers::movie_controller::get_movies;

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    log::info!("Starting server at http://localhost:3000");

    let client = mongo_connect().await?;

    HttpServer::new(move || {
        App::new()
            // todo find a way to remove clone
            .app_data(web::Data::new(client.clone()))
            .service(get_movies)
            .wrap(Logger::default())
    })
    .bind("localhost:3000")?
    .run()
    .await
    .map_err(anyhow::Error::from)
}
