use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use validator::Validate;

use crate::core::database::DbPool;
use crate::core::error::{AppError, AppResult};
use crate::domains::accounts::schema::{CreateAccountRequest, UpdateAccountRequest};
use crate::domains::accounts::service::AccountService;

pub struct AccountHandler;

impl AccountHandler {
    pub async fn create(
        State(pool): State<DbPool>,
        Json(payload): Json<CreateAccountRequest>,
    ) -> AppResult<impl IntoResponse> {
        payload.validate()?;
        let account = AccountService::create(&pool, payload.name, payload.email).await?;
        Ok((StatusCode::CREATED, Json(account)))
    }

    pub async fn list(State(pool): State<DbPool>) -> AppResult<impl IntoResponse> {
        let accounts = AccountService::get_all(&pool).await?;
        Ok(Json(accounts))
    }

    pub async fn get(
        State(pool): State<DbPool>,
        Path(id): Path<String>,
    ) -> AppResult<impl IntoResponse> {
        let account = AccountService::get_by_id(&pool, &id)
            .await?
            .ok_or(AppError::NotFound("Account not found".into()))?;
        Ok(Json(account))
    }

    pub async fn update(
        State(pool): State<DbPool>,
        Path(id): Path<String>,
        Json(payload): Json<UpdateAccountRequest>,
    ) -> AppResult<impl IntoResponse> {
        payload.validate()?;
        let account = AccountService::update(&pool, &id, payload.name, payload.email).await?;
        Ok(Json(account))
    }

    pub async fn delete(
        State(pool): State<DbPool>,
        Path(id): Path<String>,
    ) -> AppResult<impl IntoResponse> {
        AccountService::delete(&pool, &id).await?;
        Ok(StatusCode::NO_CONTENT)
    }
}
