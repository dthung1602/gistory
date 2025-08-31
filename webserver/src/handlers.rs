use std::sync::Arc;

use axum::Extension;
use axum::extract::Multipart;
use axum::extract::{Json, Path};
use axum_valid::Valid;
use diesel::{
    ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection,
};
use log::info;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;

use crate::dto::{CreateRepoDto, UploadResult};
use crate::error::{Error, FieldErr, Result};
use crate::models::*;
use crate::schema::repo;

pub async fn create_repo(
    Extension(conn): Extension<Arc<Mutex<SqliteConnection>>>,
    Valid(Json(create_repo_dto)): Valid<Json<CreateRepoDto>>,
) -> Result<Json<Repo>> {
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
        .get_result(&mut *conn)?;

    Ok(Json(res))
}

pub async fn get_repo(
    Extension(conn): Extension<Arc<Mutex<SqliteConnection>>>,
    Path(repo_id): Path<String>,
) -> Result<Json<Repo>> {
    info!("Get repo info {repo_id}");

    let mut conn = conn.lock().await;
    let result: Option<Repo> = repo::table
        .filter(repo::uuid.eq(repo_id))
        .select(Repo::as_select())
        .first(&mut *conn)
        .optional()?;

    match result {
        None => Err(Error::NotFound),
        Some(repo) => Ok(Json(repo)),
    }
}

const UPLOAD_DIR: &str = "upload";

pub async fn upload_file(mut multipart: Multipart) -> Result<Json<UploadResult>> {
    info!("Processing file upload");

    // Create the upload directory if it doesn't exist
    fs::create_dir_all(UPLOAD_DIR).await?;

    // Process the multipart form
    let field = multipart.next_field().await.map_err(|multipart_error| {
        Error::InvalidInput(vec![FieldErr {
            field: "file".to_string(),
            message: format!("{multipart_error}"),
        }])
    })?;

    let Some(field) = field else {
        return Err(Error::InvalidInput(vec![FieldErr {
            field: "file".to_string(),
            message: "file not found in form".to_string(),
        }]));
    };

    let content_type = field
        .content_type()
        .unwrap_or("application/octet-stream")
        .to_string();

    // Generate a unique ID for the file
    let file_id = uuid::Uuid::new_v4().to_string();
    let file_path = format!("{}/{}", UPLOAD_DIR, file_id);

    // Save the file
    let mut file = fs::File::create(&file_path).await?;

    // Write the file data
    let data = field.bytes().await.map_err(|multipart_error| {
        Error::InvalidInput(vec![FieldErr {
            field: "file".to_string(),
            message: format!("{multipart_error}"),
        }])
    })?;

    file.write_all(&data).await?;

    let upload_result = UploadResult {
        uuid: file_id,
        content_type,
        size: data.len(),
    };

    Ok(Json(upload_result))
}
