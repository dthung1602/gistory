use std::sync::Arc;

use axum::Extension;
use axum::extract::Multipart;
use axum::extract::{Json, Path};
use axum::http::StatusCode;
use axum_valid::Valid;
use diesel::{
    ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection,
};
use log::info;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;

use crate::dto::{CreateRepoDto, UploadResult};
use crate::models::*;
use crate::schema::repo;

// TODO format error to json

pub async fn create_repo(
    Extension(conn): Extension<Arc<Mutex<SqliteConnection>>>,
    Valid(Json(create_repo_dto)): Valid<Json<CreateRepoDto>>,
) -> Json<Repo> {
    info!("Create repo dto: {create_repo_dto:?}");
    let mut conn = conn.lock().await;

    let new_repo = Repo {
        uuid: uuid::Uuid::new_v4().to_string(),
        name: create_repo_dto.name,
        username: create_repo_dto.username,
        email: create_repo_dto.email,
        branch: create_repo_dto.branch,
        method: (create_repo_dto.method as usize) as i32,
    };
    let res = diesel::insert_into(repo::table)
        .values(&new_repo)
        .returning(Repo::as_returning())
        .get_result(&mut *conn)
        .unwrap();

    Json(res)
}

pub async fn get_repo(
    Extension(conn): Extension<Arc<Mutex<SqliteConnection>>>,
    Path(repo_id): Path<String>,
) -> (StatusCode, Json<Option<Repo>>) {
    info!("Get repo info {repo_id}");

    let mut conn = conn.lock().await;
    let result: Option<Repo> = repo::table
        .filter(repo::uuid.eq(repo_id))
        .select(Repo::as_select())
        .first(&mut *conn)
        .optional()
        .expect("Cant get data");

    let status_code = if result.is_none() {
        StatusCode::NOT_FOUND
    } else {
        StatusCode::OK
    };

    (status_code, Json(result))
}

const UPLOAD_DIR: &str = "upload";

pub async fn upload_file(multipart: Multipart) -> (StatusCode, Json<Option<UploadResult>>) {
    info!("Processing file upload");

    let mut multipart = multipart;

    // Create the upload directory if it doesn't exist
    if fs::create_dir_all(UPLOAD_DIR).await.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(None));
    }

    // Process the multipart form
    let field = multipart.next_field().await.unwrap().unwrap();
    let content_type = field
        .content_type()
        .unwrap_or("application/octet-stream")
        .to_string();

    // Generate a unique ID for the file
    let file_id = uuid::Uuid::new_v4().to_string();
    let file_path = format!("{}/{}", UPLOAD_DIR, file_id);

    // Save the file
    let Ok(mut file) = fs::File::create(&file_path).await else {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(None));
    };

    // Write the file data
    let Ok(data) = field.bytes().await else {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(None));
    };

    if file.write_all(&data).await.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(None));
    }

    let upload_result = UploadResult {
        uuid: file_id,
        content_type,
        size: data.len(),
    };

    (StatusCode::OK, Json(Some(upload_result)))
}
