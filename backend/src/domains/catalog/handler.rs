use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::core::error::{AppError, AppResult};
use crate::domains::catalog::service;
use crate::{core::database::DbPool, domains::catalog::model::AssetMaster};

use crate::domains::catalog::schema::{CreateAssetRequest, UpdateAssetRequest};

pub async fn create_asset(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateAssetRequest>,
) -> AppResult<impl IntoResponse> {
    let asset: AssetMaster =
        service::create_asset(&pool, payload.name, payload.ticker_symbol).await?;
    Ok((StatusCode::CREATED, Json(asset)))
}

pub async fn list_assets(State(pool): State<DbPool>) -> AppResult<impl IntoResponse> {
    let assets: Vec<AssetMaster> = service::get_all_assets(&pool).await?;
    Ok(Json(assets))
}

pub async fn get_asset(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
) -> AppResult<impl IntoResponse> {
    let asset: AssetMaster = service::get_asset_by_id(&pool, &id)
        .await?
        .ok_or(AppError::NotFound("Asset not found".into()))?;
    Ok(Json(asset))
}

pub async fn update_asset(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateAssetRequest>,
) -> AppResult<impl IntoResponse> {
    let asset: AssetMaster =
        service::update_asset(&pool, &id, payload.name, payload.ticker_symbol).await?;
    Ok(Json(asset))
}

pub async fn delete_asset(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
) -> AppResult<impl IntoResponse> {
    service::delete_asset(&pool, &id).await?;
    Ok(StatusCode::NO_CONTENT)
}
