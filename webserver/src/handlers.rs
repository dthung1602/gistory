use axum::extract::{Json, Path};
use axum_valid::Valid;

use crate::dto::CreateRepoDto;

// TODO format error to json

pub async fn create_repo(Valid(Json(create_repo_dto)): Valid<Json<CreateRepoDto>>) -> String {
    println!("GET: {create_repo_dto:?}");
    "Create repo".to_string()
}

pub async fn repo_info(Path(repo_id): Path<String>) -> String {
    format!("Repo info {repo_id}")
}

pub async fn download_repo(Path(repo_id): Path<String>) -> String {
    format!("Download repo {repo_id}")
}

pub async fn upload_file() -> String {
    format!("Upload file")
}
