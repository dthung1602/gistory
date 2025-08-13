use std::sync::LazyLock;

use chrono::{DateTime, FixedOffset};
use gistory::visualizer::{CommitCount, Font};
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

static NAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9-._]{1,64}$").unwrap());
static BRANCH_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9-._/]{1,64}$").unwrap());

#[derive(Clone, Debug, Deserialize)]
pub enum RepoVisualizeMethod {
    Full = 0,
    Random = 1,
    PatternFile = 2,
    Image = 3,
    Text = 4,
}

#[derive(Clone, Debug, Validate, Deserialize)]
#[validate(schema(function = "validate_create_repo_dto", skip_on_field_errors = true))]
pub struct CreateRepoDto {
    #[validate(regex(path = *NAME_REGEX))]
    pub name: String,
    #[validate(regex(path = *NAME_REGEX))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(regex(path = *BRANCH_REGEX))]
    pub branch: String,

    pub method: RepoVisualizeMethod,

    pub start_date: DateTime<FixedOffset>,
    pub end_date: Option<DateTime<FixedOffset>>,
    pub commit_count: Option<CommitCount>,
    pub font: Option<Font>,
    #[validate(length(equal = 36))]
    pub input_file: Option<String>,
    #[validate(length(min = 1, max = 64))]
    pub text: Option<String>,
}

pub fn validate_create_repo_dto(create_repo_dto: &CreateRepoDto) -> Result<(), ValidationError> {
    match create_repo_dto.method {
        RepoVisualizeMethod::Full => {
            if create_repo_dto.end_date.is_none() {
                return Err(ValidationError::new("Full method requires end_date"));
            }
            if create_repo_dto.commit_count.is_none() {
                return Err(ValidationError::new("Full method requires commit_count"));
            }
        }
        RepoVisualizeMethod::Random => {
            if create_repo_dto.end_date.is_none() {
                return Err(ValidationError::new("Random method requires end_date"));
            }
        }
        RepoVisualizeMethod::PatternFile => {
            if create_repo_dto.input_file.is_none() {
                return Err(ValidationError::new(
                    "PatternFile method requires input_file",
                ));
            }
        }
        RepoVisualizeMethod::Image => {
            if create_repo_dto.input_file.is_none() {
                return Err(ValidationError::new("Image method requires input_file"));
            }
        }
        RepoVisualizeMethod::Text => {
            if create_repo_dto.text.is_none() {
                return Err(ValidationError::new("Full method requires text"));
            }
            if create_repo_dto.font.is_none() {
                return Err(ValidationError::new("Full method requires font"));
            }
            if create_repo_dto.commit_count.is_none() {
                return Err(ValidationError::new("Full method requires commit_count"));
            }
        }
    }

    Ok(())
}

#[derive(Serialize, Debug)]
pub struct UploadResult {
    pub uuid: String,
    pub content_type: String,
    pub size: usize,
}
