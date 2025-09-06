use std::sync::LazyLock;

use chrono::{FixedOffset, NaiveDate};
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
pub struct CreateRepoDto {
    #[validate(regex(path = *NAME_REGEX))]
    pub name: String,
    #[validate(regex(path = *NAME_REGEX))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(regex(path = *BRANCH_REGEX))]
    pub branch: String,
    #[serde(deserialize_with = "deserialize_fixed_offset")]
    pub timezone: FixedOffset,
    #[validate(nested)]
    pub visualizer_method: VisualizerMethodDto,
}

fn deserialize_fixed_offset<'de, D>(deserializer: D) -> Result<FixedOffset, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    // Try parsing as offset string like "+0300", "-0500", etc.
    s.parse::<FixedOffset>().map_err(serde::de::Error::custom)
}

#[derive(Clone, Debug, Validate, Deserialize)]
#[validate(schema(
    function = "validate_visualizer_method_dto",
    skip_on_field_errors = true
))]
pub struct VisualizerMethodDto {
    pub method: RepoVisualizeMethod,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub commit_count: Option<CommitCount>,
    pub font: Option<Font>,
    #[validate(length(equal = 36))]
    pub input_file: Option<String>,
    #[validate(length(min = 1, max = 64))]
    pub text: Option<String>,
}

pub fn validate_visualizer_method_dto(dto: &VisualizerMethodDto) -> Result<(), ValidationError> {
    match dto.method {
        RepoVisualizeMethod::Full => {
            if dto.end_date.is_none() {
                return Err(ValidationError::new("Full method requires end_date"));
            }
            if dto.commit_count.is_none() {
                return Err(ValidationError::new("Full method requires commit_count"));
            }
        }
        RepoVisualizeMethod::Random => {
            if dto.end_date.is_none() {
                return Err(ValidationError::new("Random method requires end_date"));
            }
        }
        RepoVisualizeMethod::PatternFile => {
            if dto.input_file.is_none() {
                return Err(ValidationError::new(
                    "PatternFile method requires input_file",
                ));
            }
        }
        RepoVisualizeMethod::Image => {
            if dto.input_file.is_none() {
                return Err(ValidationError::new("Image method requires input_file"));
            }
        }
        RepoVisualizeMethod::Text => {
            if dto.text.is_none() {
                return Err(ValidationError::new("Full method requires text"));
            }
            if dto.font.is_none() {
                return Err(ValidationError::new("Full method requires font"));
            }
            if dto.commit_count.is_none() {
                return Err(ValidationError::new("Full method requires commit_count"));
            }
        }
    }

    Ok(())
}

#[derive(Serialize, Debug)]
pub struct Preview {
    pub data: Vec<CommitCount>,
}

#[derive(Serialize, Debug)]
pub struct UploadResult {
    pub uuid: String,
    pub content_type: String,
    pub size: usize,
}
