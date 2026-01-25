use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use validator::Validate;

use crate::core::database::DbPool;
use crate::core::error::{AppError, AppResult};
use crate::domains::portfolio::model::{Asset, AssetCategory, UserAssetGrouping};
use crate::domains::portfolio::schema::{
    CreateCategoryRequest, CreateGroupingRequest, CreateUserAssetRequest, UpdateCategoryRequest,
    UpdateGroupingRequest, UpdateUserAssetRequest,
};
use crate::domains::portfolio::service::{
    AssetCategoryService, AssetGroupingService, AssetService,
};

use crate::middleware::extractor::get_account_id;

// --- Asset Category Handler ---

#[utoipa::path(
    post,
    path = "/api/v1/portfolio/categories",
    request_body = CreateCategoryRequest,
    responses(
        (status = 201, description = "Category created successfully", body = AssetCategory),
        (status = 400, description = "Bad request")
    ),
    tag = "portfolio"
)]
pub async fn create_category(
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

#[utoipa::path(
    get,
    path = "/api/v1/portfolio/categories",
    responses(
        (status = 200, description = "List all categories", body = [AssetCategory])
    ),
    tag = "portfolio"
)]
pub async fn list_categories(
    State(pool): State<DbPool>,
    headers: HeaderMap,
) -> AppResult<impl IntoResponse> {
    let account_id = get_account_id(&headers)?;
    let categories = AssetCategoryService::get_by_account(&pool, &account_id).await?;
    Ok(Json(categories))
}

#[utoipa::path(
    get,
    path = "/api/v1/portfolio/categories/{id}",
    params(
        ("id" = String, Path, description = "Category ID")
    ),
    responses(
        (status = 200, description = "Category found", body = AssetCategory),
        (status = 404, description = "Category not found")
    ),
    tag = "portfolio"
)]
pub async fn get_category(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
) -> AppResult<impl IntoResponse> {
    let category = AssetCategoryService::get_by_id(&pool, &id)
        .await?
        .ok_or_else(|| AppError::NotFound("Category not found".into()))?;
    Ok(Json(category))
}

#[utoipa::path(
    put,
    path = "/api/v1/portfolio/categories/{id}",
    params(
        ("id" = String, Path, description = "Category ID")
    ),
    request_body = UpdateCategoryRequest,
    responses(
        (status = 200, description = "Category updated successfully", body = AssetCategory),
        (status = 404, description = "Category not found")
    ),
    tag = "portfolio"
)]
pub async fn update_category(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Json(payload): Json<UpdateCategoryRequest>,
) -> AppResult<impl IntoResponse> {
    let account_id = get_account_id(&headers)?;
    payload.validate()?;
    let category =
        AssetCategoryService::update(&pool, &id, &account_id, payload.name, payload.target_ratio)
            .await?;
    Ok(Json(category))
}

#[utoipa::path(
    delete,
    path = "/api/v1/portfolio/categories/{id}",
    params(
        ("id" = String, Path, description = "Category ID")
    ),
    responses(
        (status = 204, description = "Category deleted successfully"),
        (status = 404, description = "Category not found")
    ),
    tag = "portfolio"
)]
pub async fn delete_category(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> AppResult<impl IntoResponse> {
    let account_id = get_account_id(&headers)?;
    AssetCategoryService::delete(&pool, &id, &account_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// --- Asset Grouping Handler ---

#[utoipa::path(
    post,
    path = "/api/v1/portfolio/groupings",
    request_body = CreateGroupingRequest,
    responses(
        (status = 201, description = "Grouping created successfully", body = UserAssetGrouping),
        (status = 400, description = "Bad request")
    ),
    tag = "portfolio"
)]
pub async fn create_grouping(
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

#[utoipa::path(
    get,
    path = "/api/v1/portfolio/groupings",
    responses(
        (status = 200, description = "List all groupings", body = [UserAssetGrouping])
    ),
    tag = "portfolio"
)]
pub async fn list_groupings(
    State(pool): State<DbPool>,
    headers: HeaderMap,
) -> AppResult<impl IntoResponse> {
    let account_id = get_account_id(&headers)?;
    let groupings = AssetGroupingService::get_by_account(&pool, &account_id).await?;
    Ok(Json(groupings))
}

#[utoipa::path(
    get,
    path = "/api/v1/portfolio/groupings/{id}",
    params(
        ("id" = String, Path, description = "Grouping ID")
    ),
    responses(
        (status = 200, description = "Grouping found", body = UserAssetGrouping),
        (status = 404, description = "Grouping not found")
    ),
    tag = "portfolio"
)]
pub async fn get_grouping(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
) -> AppResult<impl IntoResponse> {
    let grouping = AssetGroupingService::get_by_id(&pool, &id)
        .await?
        .ok_or_else(|| AppError::NotFound("Grouping not found".into()))?;
    Ok(Json(grouping))
}

#[utoipa::path(
    put,
    path = "/api/v1/portfolio/groupings/{id}",
    params(
        ("id" = String, Path, description = "Grouping ID")
    ),
    request_body = UpdateGroupingRequest,
    responses(
        (status = 200, description = "Grouping updated successfully", body = UserAssetGrouping),
        (status = 404, description = "Grouping not found")
    ),
    tag = "portfolio"
)]
pub async fn update_grouping(
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

#[utoipa::path(
    delete,
    path = "/api/v1/portfolio/groupings/{id}",
    params(
        ("id" = String, Path, description = "Grouping ID")
    ),
    responses(
        (status = 204, description = "Grouping deleted successfully"),
        (status = 404, description = "Grouping not found")
    ),
    tag = "portfolio"
)]
pub async fn delete_grouping(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> AppResult<impl IntoResponse> {
    let account_id = get_account_id(&headers)?;
    AssetGroupingService::delete(&pool, &id, &account_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// --- Asset Handler ---

#[utoipa::path(
    post,
    path = "/api/v1/portfolio/assets",
    request_body = CreateUserAssetRequest,
    responses(
        (status = 201, description = "Asset (holding) created successfully", body = Asset),
        (status = 400, description = "Bad request")
    ),
    tag = "portfolio"
)]
pub async fn create_user_asset(
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

#[utoipa::path(
    get,
    path = "/api/v1/portfolio/assets",
    responses(
        (status = 200, description = "List all assets (holdings)", body = [Asset])
    ),
    tag = "portfolio"
)]
pub async fn list_user_assets(
    State(pool): State<DbPool>,
    headers: HeaderMap,
) -> AppResult<impl IntoResponse> {
    let account_id = get_account_id(&headers)?;
    let assets = AssetService::get_by_account(&pool, &account_id).await?;
    Ok(Json(assets))
}

#[utoipa::path(
    get,
    path = "/api/v1/portfolio/assets/{id}",
    params(
        ("id" = String, Path, description = "Asset holding ID")
    ),
    responses(
        (status = 200, description = "Asset holding found", body = Asset),
        (status = 404, description = "Asset holding not found")
    ),
    tag = "portfolio"
)]
pub async fn get_user_asset(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
) -> AppResult<impl IntoResponse> {
    let asset = AssetService::get_by_id(&pool, &id)
        .await?
        .ok_or_else(|| AppError::NotFound("Asset not found".into()))?;
    Ok(Json(asset))
}

#[utoipa::path(
    put,
    path = "/api/v1/portfolio/assets/{id}",
    params(
        ("id" = String, Path, description = "Asset holding ID")
    ),
    request_body = UpdateUserAssetRequest,
    responses(
        (status = 200, description = "Asset holding updated successfully", body = Asset),
        (status = 404, description = "Asset holding not found")
    ),
    tag = "portfolio"
)]
pub async fn update_user_asset(
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

#[utoipa::path(
    delete,
    path = "/api/v1/portfolio/assets/{id}",
    params(
        ("id" = String, Path, description = "Asset holding ID")
    ),
    responses(
        (status = 204, description = "Asset holding deleted successfully"),
        (status = 404, description = "Asset holding not found")
    ),
    tag = "portfolio"
)]
pub async fn delete_user_asset(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> AppResult<impl IntoResponse> {
    let account_id = get_account_id(&headers)?;
    AssetService::delete(&pool, &id, &account_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
