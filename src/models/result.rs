use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use sqlx::Error;
use crate::error::AppError;

#[derive(Serialize)]
pub struct R<T: Serialize> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

impl<T: Serialize> R<T> {
    pub fn ok(data: T) -> Self {
        R { code: 200, msg: String::from("success"), data: Some(data) }
    }
    pub fn ok_nil() -> Self {
        R { code: 200, msg: String::from("success"), data: None }
    }

    pub fn error(msg: String) -> Self {
        R { code: 500, msg, data: None }
    }

}

impl<T: Serialize> IntoResponse for R<T> {
    fn into_response(self) -> Response {
        Json(serde_json::json!(self)).into_response()
    }
}

pub trait ToResult<T: Serialize> {
    fn to_result(self) -> R<T>;
}

impl<T: Serialize> ToResult<T> for Result<T, AppError> {
    fn to_result(self) -> R<T> {
        match self {
            Ok(v) => R::ok(v),
            Err(e) => {
                println!("{:?}", e);
                R::error(e.to_string())
            }
        }
    }
}

