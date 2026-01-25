use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use validator::Validate;

use crate::core::database::DbPool;
use crate::core::error::{AppError, AppResult};
use crate::domains::portfolio::schema::{
    CreateCategoryRequest, CreateGroupingRequest, CreateUserAssetRequest, UpdateCategoryRequest,
    UpdateGroupingRequest, UpdateUserAssetRequest,
};
use crate::domains::portfolio::service::{
    AssetCategoryService, AssetGroupingService, AssetService,
};

use crate::middleware::extractor::get_account_id;

// --- Asset Category Handler ---

pub struct AssetCategoryHandler;

impl AssetCategoryHandler {
    pub async fn create(
        State(pool): State<DbPool>,
        headers: HeaderMap,
        Json(payload): Json<CreateCategoryRequest>,
    ) -> AppResult<impl IntoResponse> {
        let account_id = get_account_id(&headers)?;
        payload.validate()?;
        let category =
            AssetCategoryService::create(&pool, &account_id, payload.name, payload.target_ratio)
                .await?;
        Ok((StatusCode::CREATED, Json(category)))
    }

    pub async fn list(
        State(pool): State<DbPool>,
        headers: HeaderMap,
    ) -> AppResult<impl IntoResponse> {
        let account_id = get_account_id(&headers)?;
        let categories = AssetCategoryService::get_by_account(&pool, &account_id).await?;
        Ok(Json(categories))
    }

    pub async fn get(
        State(pool): State<DbPool>,
        Path(id): Path<String>,
    ) -> AppResult<impl IntoResponse> {
        let category = AssetCategoryService::get_by_id(&pool, &id)
            .await?
            .ok_or_else(|| AppError::NotFound("Category not found".into()))?;
        Ok(Json(category))
    }

    pub async fn update(
        State(pool): State<DbPool>,
        headers: HeaderMap,
        Path(id): Path<String>,
        Json(payload): Json<UpdateCategoryRequest>,
    ) -> AppResult<impl IntoResponse> {
        let account_id = get_account_id(&headers)?;
        payload.validate()?;
        let category = AssetCategoryService::update(
            &pool,
            &id,
            &account_id,
            payload.name,
            payload.target_ratio,
        )
        .await?;
        Ok(Json(category))
    }

    pub async fn delete(
        State(pool): State<DbPool>,
        headers: HeaderMap,
        Path(id): Path<String>,
    ) -> AppResult<impl IntoResponse> {
        let account_id = get_account_id(&headers)?;
        AssetCategoryService::delete(&pool, &id, &account_id).await?;
        Ok(StatusCode::NO_CONTENT)
    }
}

// --- Asset Grouping Handler ---

pub struct AssetGroupingHandler;

impl AssetGroupingHandler {
    pub async fn create(
        State(pool): State<DbPool>,
        headers: HeaderMap,
        Json(payload): Json<CreateGroupingRequest>,
    ) -> AppResult<impl IntoResponse> {
        let account_id = get_account_id(&headers)?;
        payload.validate()?;
        let grouping = AssetGroupingService::create(
            &pool,
            &account_id,
            payload.asset_master_id,
            payload.category_id,
        )
        .await?;
        Ok((StatusCode::CREATED, Json(grouping)))
    }

    pub async fn list(
        State(pool): State<DbPool>,
        headers: HeaderMap,
    ) -> AppResult<impl IntoResponse> {
        let account_id = get_account_id(&headers)?;
        let groupings = AssetGroupingService::get_by_account(&pool, &account_id).await?;
        Ok(Json(groupings))
    }

    pub async fn get(
        State(pool): State<DbPool>,
        Path(id): Path<String>,
    ) -> AppResult<impl IntoResponse> {
        let grouping = AssetGroupingService::get_by_id(&pool, &id)
            .await?
            .ok_or_else(|| AppError::NotFound("Grouping not found".into()))?;
        Ok(Json(grouping))
    }

    pub async fn update(
        State(pool): State<DbPool>,
        headers: HeaderMap,
        Path(id): Path<String>,
        Json(payload): Json<UpdateGroupingRequest>,
    ) -> AppResult<impl IntoResponse> {
        let account_id = get_account_id(&headers)?;
        payload.validate()?;
        let grouping =
            AssetGroupingService::update(&pool, &id, &account_id, payload.category_id).await?;
        Ok(Json(grouping))
    }

    pub async fn delete(
        State(pool): State<DbPool>,
        headers: HeaderMap,
        Path(id): Path<String>,
    ) -> AppResult<impl IntoResponse> {
        let account_id = get_account_id(&headers)?;
        AssetGroupingService::delete(&pool, &id, &account_id).await?;
        Ok(StatusCode::NO_CONTENT)
    }
}

// --- Asset Handler ---

pub struct AssetHandler;

impl AssetHandler {
    pub async fn create(
        State(pool): State<DbPool>,
        headers: HeaderMap,
        Json(payload): Json<CreateUserAssetRequest>,
    ) -> AppResult<impl IntoResponse> {
        let account_id = get_account_id(&headers)?;
        payload.validate()?;
        let asset = AssetService::create(
            &pool,
            &account_id,
            payload.asset_master_id,
            payload.current_amount,
        )
        .await?;
        Ok((StatusCode::CREATED, Json(asset)))
    }

    pub async fn list(
        State(pool): State<DbPool>,
        headers: HeaderMap,
    ) -> AppResult<impl IntoResponse> {
        let account_id = get_account_id(&headers)?;
        let assets = AssetService::get_by_account(&pool, &account_id).await?;
        Ok(Json(assets))
    }

    pub async fn get(
        State(pool): State<DbPool>,
        Path(id): Path<String>,
    ) -> AppResult<impl IntoResponse> {
        let asset = AssetService::get_by_id(&pool, &id)
            .await?
            .ok_or_else(|| AppError::NotFound("Asset not found".into()))?;
        Ok(Json(asset))
    }

    pub async fn update(
        State(pool): State<DbPool>,
        headers: HeaderMap,
        Path(id): Path<String>,
        Json(payload): Json<UpdateUserAssetRequest>,
    ) -> AppResult<impl IntoResponse> {
        let account_id = get_account_id(&headers)?;
        payload.validate()?;
        let asset = AssetService::update(&pool, &id, &account_id, payload.current_amount).await?;
        Ok(Json(asset))
    }

    pub async fn delete(
        State(pool): State<DbPool>,
        headers: HeaderMap,
        Path(id): Path<String>,
    ) -> AppResult<impl IntoResponse> {
        let account_id = get_account_id(&headers)?;
        AssetService::delete(&pool, &id, &account_id).await?;
        Ok(StatusCode::NO_CONTENT)
    }
}
