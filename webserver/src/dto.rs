use std::sync::LazyLock;

use chrono::{DateTime, FixedOffset};
use gistory::visualizer::{CommitCount, Font};
use regex::Regex;
use serde::Deserialize;
use validator::{Validate, ValidationError};

static NAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9-._]{1,64}$").unwrap());
static BRANCH_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9-._/]{1,64}$").unwrap());

#[derive(Clone, Debug, Deserialize)]
pub enum RepoVisualizeMethod {
    Full,
    Random,
    PatternFile,
    Image,
    Text,
}

#[derive(Clone, Debug, Validate, Deserialize)]
#[validate(schema(function = "validate_create_repo_dto", skip_on_field_errors = true))]
pub struct CreateRepoDto {
    #[validate(regex(path = *NAME_REGEX))]
    name: String,
    #[validate(regex(path = *NAME_REGEX))]
    username: String,
    #[validate(email)]
    email: String,
    #[validate(regex(path = *BRANCH_REGEX))]
    branch: String,

    method: RepoVisualizeMethod,

    start_date: DateTime<FixedOffset>,
    end_date: Option<DateTime<FixedOffset>>,
    commit_count: Option<CommitCount>,
    font: Option<Font>,
    #[validate(length(equal = 36))]
    input_file: Option<String>,
    #[validate(length(min = 1, max = 64))]
    text: Option<String>,
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
