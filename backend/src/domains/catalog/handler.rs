use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::core::database::DbPool;
use crate::core::error::{AppError, AppResult};
use crate::domains::catalog::model::AssetMaster;
use crate::domains::catalog::schema::{CreateAssetRequest, UpdateAssetRequest};
use crate::domains::catalog::service::AssetMasterService;

pub struct AssetMasterHandler;

impl AssetMasterHandler {
    pub async fn create(
        State(pool): State<DbPool>,
        Json(payload): Json<CreateAssetRequest>,
    ) -> AppResult<impl IntoResponse> {
        let asset: AssetMaster =
            AssetMasterService::create(&pool, payload.name, payload.ticker_symbol).await?;
        Ok((StatusCode::CREATED, Json(asset)))
    }

    pub async fn list(State(pool): State<DbPool>) -> AppResult<impl IntoResponse> {
        let assets: Vec<AssetMaster> = AssetMasterService::get_all(&pool).await?;
        Ok(Json(assets))
    }

    pub async fn get(
        State(pool): State<DbPool>,
        Path(id): Path<String>,
    ) -> AppResult<impl IntoResponse> {
        let asset: AssetMaster = AssetMasterService::get_by_id(&pool, &id)
            .await?
            .ok_or(AppError::NotFound("Asset not found".into()))?;
        Ok(Json(asset))
    }

    pub async fn update(
        State(pool): State<DbPool>,
        Path(id): Path<String>,
        Json(payload): Json<UpdateAssetRequest>,
    ) -> AppResult<impl IntoResponse> {
        let asset: AssetMaster =
            AssetMasterService::update(&pool, &id, payload.name, payload.ticker_symbol).await?;
        Ok(Json(asset))
    }

    pub async fn delete(
        State(pool): State<DbPool>,
        Path(id): Path<String>,
    ) -> AppResult<impl IntoResponse> {
        AssetMasterService::delete(&pool, &id).await?;
        Ok(StatusCode::NO_CONTENT)
    }
}
