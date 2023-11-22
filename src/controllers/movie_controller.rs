use actix_web::{get, HttpResponse, Responder};

#[get("/movies")]
async fn get_movies() -> impl Responder {
    HttpResponse::Ok().body("Hello, world !!!")
}

