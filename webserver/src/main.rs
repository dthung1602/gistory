mod constants;
mod dto;
mod error;
mod handlers;
mod models;
mod schema;
mod tasks;

use std::env;
use std::sync::Arc;

use axum::{
    Extension, Router,
    routing::{get, post},
};
use diesel::prelude::*;
use dotenvy::dotenv;
use tokio::sync::Mutex;
use tower_http::compression::CompressionLayer;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    let db = Arc::new(Mutex::new(establish_connection()));

    let app = Router::new()
        .route_service(
            "/",
            ServeFile::new_with_mime("static/index.html", &mime::TEXT_HTML),
        )
        .nest_service("/static", ServeDir::new("static"))
        .route("/api/preview", get(handlers::preview))
        .route("/api/repo", post(handlers::create_repo))
        .route("/api/repo/{id}", get(handlers::get_repo))
        .route("/api/upload", post(handlers::upload_file))
        .layer(CompressionLayer::new())
        .nest_service("/api/download", ServeDir::new("download"))
        .layer(Extension(db));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
