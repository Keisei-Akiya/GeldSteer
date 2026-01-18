use crate::core::database::DbPool;
use crate::core::error::AppResult;
use crate::domains::portfolio::model::{Asset, AssetCategory, UserAssetGrouping};
use crate::domains::portfolio::repository;
use crate::shared::util::generate_id;
use rust_decimal::Decimal;

// --- Asset Categories ---

pub async fn create_category(
    pool: &DbPool,
    account_id: &str,
    name: String,
    target_ratio: Decimal,
) -> AppResult<AssetCategory> {
    let id = generate_id();
    repository::create_category(pool, &id, account_id, name, target_ratio).await
}

pub async fn get_categories_by_account(
    pool: &DbPool,
    account_id: &str,
) -> AppResult<Vec<AssetCategory>> {
    repository::find_categories_by_account(pool, account_id).await
}

pub async fn get_category_by_id(pool: &DbPool, id: &str) -> AppResult<Option<AssetCategory>> {
    repository::find_category_by_id(pool, id).await
}

pub async fn update_category(
    pool: &DbPool,
    id: &str,
    account_id: &str,
    name: String,
    target_ratio: Decimal,
) -> AppResult<AssetCategory> {
    repository::update_category(pool, id, account_id, name, target_ratio).await
}

pub async fn delete_category(pool: &DbPool, id: &str, account_id: &str) -> AppResult<()> {
    repository::delete_category(pool, id, account_id).await
}

// --- User Asset Groupings ---

pub async fn create_grouping(
    pool: &DbPool,
    account_id: &str,
    asset_master_id: String,
    category_id: String,
) -> AppResult<UserAssetGrouping> {
    let id = generate_id();
    repository::create_grouping(pool, &id, account_id, asset_master_id, category_id).await
}

pub async fn get_groupings_by_account(
    pool: &DbPool,
    account_id: &str,
) -> AppResult<Vec<UserAssetGrouping>> {
    repository::find_groupings_by_account(pool, account_id).await
}

pub async fn get_grouping_by_id(pool: &DbPool, id: &str) -> AppResult<Option<UserAssetGrouping>> {
    repository::find_grouping_by_id(pool, id).await
}

pub async fn update_grouping(
    pool: &DbPool,
    id: &str,
    account_id: &str,
    category_id: String,
) -> AppResult<UserAssetGrouping> {
    repository::update_grouping(pool, id, account_id, category_id).await
}

pub async fn delete_grouping(pool: &DbPool, id: &str, account_id: &str) -> AppResult<()> {
    repository::delete_grouping(pool, id, account_id).await
}

// --- Assets (Holdings) ---

pub async fn create_asset(
    pool: &DbPool,
    account_id: &str,
    asset_master_id: String,
    current_amount: Decimal,
) -> AppResult<Asset> {
    let id = generate_id();
    repository::create_asset(pool, &id, account_id, asset_master_id, current_amount).await
}

pub async fn get_assets_by_account(pool: &DbPool, account_id: &str) -> AppResult<Vec<Asset>> {
    repository::find_assets_by_account(pool, account_id).await
}

pub async fn get_asset_by_id(pool: &DbPool, id: &str) -> AppResult<Option<Asset>> {
    repository::find_asset_by_id(pool, id).await
}

pub async fn update_asset(
    pool: &DbPool,
    id: &str,
    account_id: &str,
    current_amount: Decimal,
) -> AppResult<Asset> {
    repository::update_asset(pool, id, account_id, current_amount).await
}

pub async fn delete_asset(pool: &DbPool, id: &str, account_id: &str) -> AppResult<()> {
    repository::delete_asset(pool, id, account_id).await
}
