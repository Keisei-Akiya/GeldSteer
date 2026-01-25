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

#[utoipa::path(
    post,
    path = "/api/v1/catalog",
    request_body = CreateAssetRequest,
    responses(
        (status = 201, description = "Asset created successfully", body = AssetMaster),
        (status = 400, description = "Bad request")
    ),
    tag = "catalog"
)]
pub async fn create_asset(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateAssetRequest>,
) -> AppResult<impl IntoResponse> {
    let asset: AssetMaster =
        AssetMasterService::create(&pool, payload.name, payload.ticker_symbol).await?;
    Ok((StatusCode::CREATED, Json(asset)))
}

#[utoipa::path(
    get,
    path = "/api/v1/catalog",
    responses(
        (status = 200, description = "List all assets", body = [AssetMaster])
    ),
    tag = "catalog"
)]
pub async fn list_assets(State(pool): State<DbPool>) -> AppResult<impl IntoResponse> {
    let assets: Vec<AssetMaster> = AssetMasterService::get_all(&pool).await?;
    Ok(Json(assets))
}

#[utoipa::path(
    get,
    path = "/api/v1/catalog/{id}",
    params(
        ("id" = String, Path, description = "Asset ID")
    ),
    responses(
        (status = 200, description = "Asset found", body = AssetMaster),
        (status = 404, description = "Asset not found")
    ),
    tag = "catalog"
)]
pub async fn get_asset(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
) -> AppResult<impl IntoResponse> {
    let asset: AssetMaster = AssetMasterService::get_by_id(&pool, &id)
        .await?
        .ok_or(AppError::NotFound("Asset not found".into()))?;
    Ok(Json(asset))
}

#[utoipa::path(
    put,
    path = "/api/v1/catalog/{id}",
    params(
        ("id" = String, Path, description = "Asset ID")
    ),
    request_body = UpdateAssetRequest,
    responses(
        (status = 200, description = "Asset updated successfully", body = AssetMaster),
        (status = 404, description = "Asset not found")
    ),
    tag = "catalog"
)]
pub async fn update_asset(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateAssetRequest>,
) -> AppResult<impl IntoResponse> {
    let asset: AssetMaster =
        AssetMasterService::update(&pool, &id, payload.name, payload.ticker_symbol).await?;
    Ok(Json(asset))
}

#[utoipa::path(
    delete,
    path = "/api/v1/catalog/{id}",
    params(
        ("id" = String, Path, description = "Asset ID")
    ),
    responses(
        (status = 204, description = "Asset deleted successfully"),
        (status = 404, description = "Asset not found")
    ),
    tag = "catalog"
)]
pub async fn delete_asset(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
) -> AppResult<impl IntoResponse> {
    AssetMasterService::delete(&pool, &id).await?;
    Ok(StatusCode::NO_CONTENT)
}
