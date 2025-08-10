mod dto;
mod handlers;

use axum::{
    Router,
    routing::{get, post},
};
use tower_http::compression::CompressionLayer;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route_service(
            "/",
            ServeFile::new_with_mime("static/index.html", &mime::TEXT_HTML),
        )
        .nest_service("/static", ServeDir::new("static"))
        .route("/api/repo", post(handlers::create_repo))
        .route("/api/repo/{id}", get(handlers::repo_info))
        .route("/api/upload", get(handlers::upload_file))
        .layer(CompressionLayer::new())
        .route("/api/download/{id}", get(handlers::download_repo));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/**
asdf
/                        GET    index page
/static/ *               GET    static files
/api/repo                POST   create repo
/api/repo/{id}           GET    get repo metadata: download link, expire, etc
/api/download/{id}       GET    download file
/api/upload              POST   upload temp file for ref as image/pattern file
*/
struct Foo {}
