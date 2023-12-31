use axum::{http::StatusCode, Json, response::{IntoResponse, Response}};
use serde_json::json;
use thiserror::Error;

pub type Result<T, E = AppError> = core::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum AppError {
    #[allow(dead_code)]
    #[error("Not Found: {0}")]
    NotFound(&'static str),
    #[allow(dead_code)]
    #[error("Invalid params: {0:?}")]
    InvalidParams(Vec<&'static str>),
    #[error("Invalid file format")]
    InvalidFileFormat,
    #[error("Error parsing `multipart/form-data` request.\n{0}")]
    MultipartError(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, err_msg) = match self {
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::InvalidParams(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            AppError::MultipartError(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            AppError::InvalidFileFormat => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            AppError::Other(_) => {
                println!("{:?}", self);
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            },
        };

        let body = Json(json!({
            "error": err_msg,
        }));
        println!("{}", body.to_string());
        (status, body).into_response()
    }
}
