use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Movie {
    id: String,
    movie_id: u32,
    original_title: String,
    original_language: String,
    overview: String,
    popularity: f64,
    poster_path: String,
    backdrop_path: String,
    release_date: Option<String>,
    vote_average: f64,
    vote_count: u32,
    created_at: Option<String>,
    updated_at: Option<String>,
    casts: Option<Vec<Cast>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cast {
    id: String,
    movie_id: u32,
    character: String,
    name: String,
    profile_path: String,
    popularity: String,
    created_at: Option<String>,
    updated_at: Option<String>,
}
