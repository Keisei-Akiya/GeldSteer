use crate::core::error::{AppError, AppResult};
use axum::http::HeaderMap;

/// Helper to extract account_id from headers
pub fn get_account_id(headers: &HeaderMap) -> AppResult<String> {
    headers
        .get("X-Account-ID")
        .and_then(|val| val.to_str().ok())
        .map(|s| s.to_string())
        .ok_or_else(|| AppError::NotFound("X-Account-ID header missing".into()))
}
