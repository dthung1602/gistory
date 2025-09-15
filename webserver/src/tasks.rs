use std::path::{Path, PathBuf};
use std::sync::Arc;

use async_compression::tokio::write::ZstdEncoder;
use axum::body::Bytes;
use axum::extract::multipart::Field;
use diesel::serialize::ToSql;
use diesel::{ExpressionMethods, OptionalExtension, QueryResult, RunQueryDsl, SqliteConnection};
use gistory::git;
use gistory::visualizer::CommitGrid;
use log::{debug, error, info};
use tokio::fs;
use tokio::io::{AsyncWriteExt, BufReader};
use tokio::sync::Mutex;

use crate::constants::{REPO_DOWNLOAD_DIR, UPLOAD_DIR};
use crate::dbconnection;
use crate::dto::{CreateRepoDto, RepoVisualizeMethod, VisualizerMethodDto};
use crate::error::{Error, FieldErr, Result};
use crate::models::{Repo, RepoStatus};

pub async fn create_upload_file(field: Field<'_>) -> Result<(String, Bytes)> {
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
    Ok((file_id, data))
}

pub async fn create_grid_from_dto(dto: VisualizerMethodDto) -> Result<CommitGrid> {
    let mut grid = CommitGrid::new(dto.start_date);
    match dto.method {
        RepoVisualizeMethod::Full => {
            grid.full(dto.commit_count.unwrap(), dto.end_date.unwrap())?;
        }
        RepoVisualizeMethod::Random => grid.random(dto.end_date.unwrap())?,
        RepoVisualizeMethod::PatternFile => {
            let mut path_buf = PathBuf::from(UPLOAD_DIR);
            path_buf.push(dto.input_file.unwrap());
            grid.read_pattern_file(&path_buf).await?;
        }
        RepoVisualizeMethod::Image => {
            let mut path_buf = PathBuf::from(UPLOAD_DIR);
            path_buf.push(dto.input_file.unwrap());
            grid.read_image_file(&path_buf).await?;
        }
        RepoVisualizeMethod::Text => {
            let text = dto.text.unwrap();
            let font = dto.font.unwrap();
            let commit_count = dto.commit_count.unwrap();
            grid.show_text(text.clone(), font, commit_count)?;
        }
        RepoVisualizeMethod::RawPattern => {
            let raw_pattern = dto.raw_pattern.unwrap();
            grid.set_data(raw_pattern);
        }
    }
    Ok(grid)
}

pub async fn compress_directory(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();

    let mut archive_file_path = path.to_path_buf();
    archive_file_path.set_extension("tar");
    let archive_file = fs::File::create(&archive_file_path).await?;

    let mut compressed_file_path = path.to_path_buf();
    compressed_file_path.set_extension("tar.zst");
    let compressed_file = fs::File::create(compressed_file_path).await?;

    let mut tar_builder = tokio_tar::Builder::new(archive_file);
    tar_builder.append_dir_all("", path).await?;
    let mut archive_file = tar_builder.into_inner().await?;
    archive_file.flush().await?;
    drop(archive_file);

    let archive_file = fs::File::open(&archive_file_path).await?;
    let mut archive_file_reader = BufReader::new(archive_file);

    let mut compressor = ZstdEncoder::new(compressed_file);
    tokio::io::copy(&mut archive_file_reader, &mut compressor).await?;
    compressor.shutdown().await?;
    let mut compressed_file = compressor.into_inner();
    compressed_file.flush().await?;
    drop(compressed_file);

    fs::remove_file(archive_file_path).await?;

    Ok(())
}

pub async fn generate_repo(repo_dto: CreateRepoDto, db_repo: Repo) {
    use std::env;

    use crate::schema::repo::dsl::{repo, status, uuid};

    info!("Start generating repo: {} {:?}", db_repo.uuid, repo_dto);
    let status_value = if do_generate_repo(repo_dto, &db_repo).await.is_ok() {
        RepoStatus::Done
    } else {
        RepoStatus::Error
    };
    let query = diesel::update(repo)
        .filter(uuid.eq(&db_repo.uuid))
        .set(status.eq(status_value));

    let mut conn = dbconnection::establish_connection();
    let result = query.execute(&mut conn);
    if let Err(err) = result {
        error!("Error updating repo status: {err:?}");
    } else {
        info!("Created repo for {}", db_repo.uuid);
    }
}

async fn do_generate_repo(repo_dto: CreateRepoDto, db_repo: &Repo) -> Result<()> {
    let grid = create_grid_from_dto(repo_dto.visualizer_method).await?;

    let mut working_dir = std::env::current_dir()?;
    working_dir.push(REPO_DOWNLOAD_DIR);
    working_dir.push(&db_repo.uuid);
    let repo_path = working_dir.join(repo_dto.name);

    let mut git_repo = git::repo::Repo::new(
        repo_path.clone(),
        repo_dto.branch,
        repo_dto.timezone,
        repo_dto.username,
        repo_dto.email,
    );
    git_repo.init().await.unwrap();
    debug!("Git repo: {git_repo:?}");

    grid.populate_repo(&mut git_repo).await?;
    debug!("Repo populated {}", db_repo.uuid);

    compress_directory(&repo_path).await?;
    debug!("Compress repo {}", db_repo.uuid);

    Ok(())
}
