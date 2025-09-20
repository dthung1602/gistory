mod constants;
mod dbconnection;
mod dto;
mod error;
mod handlers;
mod models;
mod schema;
mod tasks;

use std::sync::Arc;

use crate::constants::REPO_DOWNLOAD_DIR;
use axum::http::Method;
use axum::{
    routing::{get, post}, Extension,
    Router,
};
use core::time::Duration;
use dotenvy::dotenv;
use http::header::{ACCEPT, CONTENT_TYPE};
use tokio::sync::Mutex;
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer, MaxAge};
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    let db = Arc::new(Mutex::new(dbconnection::establish_connection()));

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([ACCEPT, CONTENT_TYPE])
        .max_age(MaxAge::exact(Duration::from_secs(3600 * 24)));

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
        .layer(cors)
        .nest_service("/api/download", ServeDir::new(REPO_DOWNLOAD_DIR))
        .layer(Extension(db));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
