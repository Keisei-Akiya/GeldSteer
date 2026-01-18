use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    Database(sqlx::Error),
    NotFound(String),
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Database(err) => {
                eprintln!("Database error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string())
            }
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
        };

        (
            status,
            Json(json!({
                "error": message
            })),
        )
        .into_response()
    }
}
