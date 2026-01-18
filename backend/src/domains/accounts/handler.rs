use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::core::database::DbPool;
use crate::core::error::{AppError, AppResult};
use crate::domains::accounts::schema::{CreateAccountRequest, UpdateAccountRequest};
use crate::domains::accounts::service;

pub async fn create_account(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateAccountRequest>,
) -> AppResult<impl IntoResponse> {
    let account = service::create_account(&pool, payload.name, payload.email).await?;
    Ok((StatusCode::CREATED, Json(account)))
}

pub async fn list_accounts(State(pool): State<DbPool>) -> AppResult<impl IntoResponse> {
    let accounts = service::get_all_accounts(&pool).await?;
    Ok(Json(accounts))
}

pub async fn get_account(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
) -> AppResult<impl IntoResponse> {
    let account = service::get_account_by_id(&pool, &id)
        .await?
        .ok_or(AppError::NotFound("Account not found".into()))?;
    Ok(Json(account))
}

pub async fn update_account(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateAccountRequest>,
) -> AppResult<impl IntoResponse> {
    let account = service::update_account(&pool, &id, payload.name, payload.email).await?;
    Ok(Json(account))
}

pub async fn delete_account(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
) -> AppResult<impl IntoResponse> {
    service::delete_account(&pool, &id).await?;
    Ok(StatusCode::NO_CONTENT)
}
