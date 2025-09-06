use std::io::Error as IOError;
use std::result::Result as StdResult;

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use diesel::result::Error as DieselError;
use gistory::error::Error as GistoryError;
use serde_json::json;

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub struct FieldErr {
    pub field: String,
    pub message: String,
}

#[derive(Debug)]
pub enum Error {
    NotFound,
    InvalidInput(Vec<FieldErr>),
    Database(DieselError),
    GistoryError(GistoryError),
    IO(IOError),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let res = match self {
            Error::NotFound => (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "error": "Not found"
                })),
            ),
            Error::InvalidInput(field_errs) => {
                let details: Vec<_> = field_errs
                    .iter()
                    .map(|fe| {
                        json!({
                            "field": fe.field,
                            "message": fe.message,
                        })
                    })
                    .collect();
                let value = json!({
                    "error": "Invalid input",
                    "details": details
                });
                (StatusCode::BAD_REQUEST, Json(value))
            }
            err => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("Internal server error: {err:?}").to_string()
                })),
            ),
        };
        res.into_response()
    }
}

impl From<DieselError> for Error {
    fn from(value: DieselError) -> Self {
        Error::Database(value)
    }
}

impl From<IOError> for Error {
    fn from(value: IOError) -> Self {
        Error::IO(value)
    }
}

impl From<GistoryError> for Error {
    fn from(value: GistoryError) -> Self {
        use GistoryError::*;
        match value {
            Io(e) => Error::IO(e),
            InvalidArg(message) => Error::InvalidInput(vec![FieldErr {
                field: "__all__".to_string(),
                message,
            }]),
            Utf8(e) => Error::InvalidInput(vec![FieldErr {
                field: "__all__".to_string(),
                message: format!("{e}"),
            }]),
            InvalidData(message) => Error::InvalidInput(vec![FieldErr {
                field: "__all__".to_string(),
                message,
            }]),
            _ => Error::GistoryError(value),
        }
    }
}
