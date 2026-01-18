use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

pub type AppResult<T> = Result<T, AppError>;

use validator::ValidationErrors;

#[derive(Debug)]
pub enum AppError {
    Database(sqlx::Error),
    NotFound(String),
    Validation(ValidationErrors),
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err)
    }
}

impl From<ValidationErrors> for AppError {
    fn from(err: ValidationErrors) -> Self {
        AppError::Validation(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Database(err) => {
                eprintln!("Database error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    json!({"error": "Internal Server Error"}),
                )
            }
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, json!({"error": msg})),
            AppError::Validation(err) => (
                StatusCode::BAD_REQUEST,
                json!({"error": "Validation error", "details": err}),
            ),
        };

        (status, Json(message)).into_response()
    }
}
