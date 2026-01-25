use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use validator::Validate;

use crate::core::database::DbPool;
use crate::core::error::{AppError, AppResult};
use crate::domains::accounts::model::Account;
use crate::domains::accounts::schema::{CreateAccountRequest, UpdateAccountRequest};
use crate::domains::accounts::service::AccountService;

#[utoipa::path(
    post,
    path = "/api/v1/accounts",
    request_body = CreateAccountRequest,
    responses(
        (status = 201, description = "Account created successfully", body = Account),
        (status = 400, description = "Bad request")
    ),
    tag = "accounts"
)]
pub async fn create_account(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateAccountRequest>,
) -> AppResult<impl IntoResponse> {
    payload.validate()?;
    let account = AccountService::create(&pool, payload.name, payload.email).await?;
    Ok((StatusCode::CREATED, Json(account)))
}

#[utoipa::path(
    get,
    path = "/api/v1/accounts",
    responses(
        (status = 200, description = "List all accounts", body = [Account])
    ),
    tag = "accounts"
)]
pub async fn list_accounts(State(pool): State<DbPool>) -> AppResult<impl IntoResponse> {
    let accounts = AccountService::get_all(&pool).await?;
    Ok(Json(accounts))
}

#[utoipa::path(
    get,
    path = "/api/v1/accounts/{id}",
    params(
        ("id" = String, Path, description = "Account ID")
    ),
    responses(
        (status = 200, description = "Account found", body = Account),
        (status = 404, description = "Account not found")
    ),
    tag = "accounts"
)]
pub async fn get_account(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
) -> AppResult<impl IntoResponse> {
    let account = AccountService::get_by_id(&pool, &id)
        .await?
        .ok_or(AppError::NotFound("Account not found".into()))?;
    Ok(Json(account))
}

#[utoipa::path(
    put,
    path = "/api/v1/accounts/{id}",
    params(
        ("id" = String, Path, description = "Account ID")
    ),
    request_body = UpdateAccountRequest,
    responses(
        (status = 200, description = "Account updated successfully", body = Account),
        (status = 404, description = "Account not found")
    ),
    tag = "accounts"
)]
pub async fn update_account(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateAccountRequest>,
) -> AppResult<impl IntoResponse> {
    payload.validate()?;
    let account = AccountService::update(&pool, &id, payload.name, payload.email).await?;
    Ok(Json(account))
}

#[utoipa::path(
    delete,
    path = "/api/v1/accounts/{id}",
    params(
        ("id" = String, Path, description = "Account ID")
    ),
    responses(
        (status = 204, description = "Account deleted successfully"),
        (status = 404, description = "Account not found")
    ),
    tag = "accounts"
)]
pub async fn delete_account(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
) -> AppResult<impl IntoResponse> {
    AccountService::delete(&pool, &id).await?;
    Ok(StatusCode::NO_CONTENT)
}
