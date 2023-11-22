use actix_web::{middleware::Logger, App, HttpServer};

use api::controllers::movie_controller::get_movies;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    log::info!("Starting server at http://localhost:3000");

    HttpServer::new(|| App::new().service(get_movies).wrap(Logger::default()))
        .bind("localhost:3000")?
        .run()
        .await
}
