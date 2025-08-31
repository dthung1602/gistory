use std::io::Error as IOError;
use std::result::Result as StdResult;

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use diesel::result::Error as DieselError;
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
