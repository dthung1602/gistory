use std::sync::Arc;

use axum::Extension;
use axum::body::Bytes;
use axum::extract::Multipart;
use axum::extract::multipart::Field;
use axum::extract::{Json, Path, Query};
use axum_valid::Valid;
use diesel::{
    ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection,
};
use log::info;
use tokio::fs;
use tokio::sync::Mutex;

use crate::constants::UPLOAD_DIR;
use crate::dto::{CreateRepoDto, Preview, UploadResult, VisualizerMethodDto};
use crate::error::{Error, FieldErr, Result};
use crate::models::*;
use crate::schema::repo;
use crate::tasks::{create_grid_from_dto, create_upload_file, generate_repo};

pub async fn preview(
    Valid(Query(dto)): Valid<Query<VisualizerMethodDto>>,
) -> Result<Json<Preview>> {
    info!("Preview dto: {dto:?}");
    let grid = create_grid_from_dto(dto).await?;
    Ok(Json(Preview {
        data: grid.get_data().to_vec(),
    }))
}

pub async fn create_repo(
    Extension(conn): Extension<Arc<Mutex<SqliteConnection>>>,
    Valid(Json(create_repo_dto)): Valid<Json<CreateRepoDto>>,
) -> Result<Json<Repo>> {
    info!("Create repo dto: {create_repo_dto:?}");
    let mut conn = conn.lock().await;

    let new_repo = Repo {
        uuid: uuid::Uuid::new_v4().to_string(),
        name: create_repo_dto.name.clone(),
        username: create_repo_dto.username.clone(),
        email: create_repo_dto.email.clone(),
        branch: create_repo_dto.branch.clone(),
        method: create_repo_dto.visualizer_method.method as i32,
        status: RepoStatus::New,
    };
    let repo = diesel::insert_into(repo::table)
        .values(&new_repo)
        .returning(Repo::as_returning())
        .get_result(&mut *conn)?;

    tokio::spawn(generate_repo(create_repo_dto, repo.clone()));

    Ok(Json(repo))
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

    let (file_id, data) = create_upload_file(field).await?;

    let upload_result = UploadResult {
        uuid: file_id,
        content_type,
        size: data.len(),
    };

    Ok(Json(upload_result))
}
