use crate::core::database::DbPool;
use crate::core::error::AppResult;
use crate::domains::portfolio::model::{Asset, AssetCategory, UserAssetGrouping};
use crate::domains::portfolio::repository::{
    AssetCategoryRepository, AssetGroupingRepository, AssetRepository,
};
use crate::shared::util::generate_id;
use rust_decimal::Decimal;

// --- Asset Category Service ---

pub struct AssetCategoryService;

impl AssetCategoryService {
    pub async fn create(
        pool: &DbPool,
        account_id: &str,
        name: String,
        target_ratio: Decimal,
    ) -> AppResult<AssetCategory> {
        let id = generate_id();
        AssetCategoryRepository::create(pool, &id, account_id, name, target_ratio).await
    }

    pub async fn get_by_account(pool: &DbPool, account_id: &str) -> AppResult<Vec<AssetCategory>> {
        AssetCategoryRepository::find_by_account(pool, account_id).await
    }

    pub async fn get_by_id(pool: &DbPool, id: &str) -> AppResult<Option<AssetCategory>> {
        AssetCategoryRepository::find_by_id(pool, id).await
    }

    pub async fn update(
        pool: &DbPool,
        id: &str,
        account_id: &str,
        name: String,
        target_ratio: Decimal,
    ) -> AppResult<AssetCategory> {
        AssetCategoryRepository::update(pool, id, account_id, name, target_ratio).await
    }

    pub async fn delete(pool: &DbPool, id: &str, account_id: &str) -> AppResult<()> {
        AssetCategoryRepository::delete(pool, id, account_id).await
    }
}

// --- Asset Grouping Service ---

pub struct AssetGroupingService;

impl AssetGroupingService {
    pub async fn create(
        pool: &DbPool,
        account_id: &str,
        asset_master_id: String,
        category_id: String,
    ) -> AppResult<UserAssetGrouping> {
        let id = generate_id();
        AssetGroupingRepository::create(pool, &id, account_id, asset_master_id, category_id).await
    }

    pub async fn get_by_account(
        pool: &DbPool,
        account_id: &str,
    ) -> AppResult<Vec<UserAssetGrouping>> {
        AssetGroupingRepository::find_by_account(pool, account_id).await
    }

    pub async fn get_by_id(pool: &DbPool, id: &str) -> AppResult<Option<UserAssetGrouping>> {
        AssetGroupingRepository::find_by_id(pool, id).await
    }

    pub async fn update(
        pool: &DbPool,
        id: &str,
        account_id: &str,
        category_id: String,
    ) -> AppResult<UserAssetGrouping> {
        AssetGroupingRepository::update(pool, id, account_id, category_id).await
    }

    pub async fn delete(pool: &DbPool, id: &str, account_id: &str) -> AppResult<()> {
        AssetGroupingRepository::delete(pool, id, account_id).await
    }
}

// --- Asset Service ---

pub struct AssetService;

impl AssetService {
    pub async fn create(
        pool: &DbPool,
        account_id: &str,
        asset_master_id: String,
        current_amount: Decimal,
    ) -> AppResult<Asset> {
        let id = generate_id();
        AssetRepository::create(pool, &id, account_id, asset_master_id, current_amount).await
    }

    pub async fn get_by_account(pool: &DbPool, account_id: &str) -> AppResult<Vec<Asset>> {
        AssetRepository::find_by_account(pool, account_id).await
    }

    pub async fn get_by_id(pool: &DbPool, id: &str) -> AppResult<Option<Asset>> {
        AssetRepository::find_by_id(pool, id).await
    }

    pub async fn update(
        pool: &DbPool,
        id: &str,
        account_id: &str,
        current_amount: Decimal,
    ) -> AppResult<Asset> {
        AssetRepository::update(pool, id, account_id, current_amount).await
    }

    pub async fn delete(pool: &DbPool, id: &str, account_id: &str) -> AppResult<()> {
        AssetRepository::delete(pool, id, account_id).await
    }
}
