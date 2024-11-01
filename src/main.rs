use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{
    routing::{get, post},
    Router,
};
use entities::movie::Movie;
use routes::movies::{create::create_movie, get::get_movie};

mod entities;
mod routes;

#[derive(Clone)]
struct AppState {
    db: Arc<Mutex<HashMap<String, Movie>>>,
}

#[tokio::main]
async fn main() {
    let db = Arc::new(Mutex::new(HashMap::<String, Movie>::new()));
    let app = Router::new()
        .route("/movie/:id", get(get_movie))
        .route("/movie", post(create_movie))
        .with_state(AppState { db });

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listening on port 3000");
    axum::serve(listener, app).await.unwrap();
}
