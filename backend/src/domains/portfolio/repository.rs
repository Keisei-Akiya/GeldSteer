use crate::core::database::DbPool;
use crate::core::error::{AppError, AppResult};
use crate::domains::portfolio::model::{Asset, AssetCategory, UserAssetGrouping};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use sqlx::query_as;

// --- Asset Categories Repository ---

pub struct AssetCategoryRepository;

impl AssetCategoryRepository {
    pub async fn create(
        pool: &DbPool,
        id: &str,
        account_id: &str,
        name: String,
        target_ratio: Decimal,
    ) -> AppResult<AssetCategory> {
        sqlx::query(
            r#"
            INSERT INTO asset_categories (id, account_id, name, target_ratio)
            VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(id)
        .bind(account_id)
        .bind(name)
        .bind(target_ratio.to_f64().unwrap_or(0.0))
        .execute(pool)
        .await?;

        Self::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Created category not found".into()))
    }

    pub async fn find_by_account(pool: &DbPool, account_id: &str) -> AppResult<Vec<AssetCategory>> {
        let categories = query_as::<_, AssetCategory>(
            r#"
            SELECT id, account_id, name, target_ratio, created_at, updated_at
            FROM asset_categories
            WHERE account_id = ?
            ORDER BY created_at DESC
            "#,
        )
        .bind(account_id)
        .fetch_all(pool)
        .await?;

        Ok(categories)
    }

    pub async fn find_by_id(pool: &DbPool, id: &str) -> AppResult<Option<AssetCategory>> {
        let category = query_as::<_, AssetCategory>(
            r#"
            SELECT id, account_id, name, target_ratio, created_at, updated_at
            FROM asset_categories
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(category)
    }

    pub async fn update(
        pool: &DbPool,
        id: &str,
        account_id: &str,
        name: String,
        target_ratio: Decimal,
    ) -> AppResult<AssetCategory> {
        let result = sqlx::query(
            r#"
            UPDATE asset_categories
            SET name = ?, target_ratio = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ? AND account_id = ?
            "#,
        )
        .bind(name)
        .bind(target_ratio.to_f64().unwrap_or(0.0))
        .bind(id)
        .bind(account_id)
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(
                "Category not found or access denied".into(),
            ));
        }

        Self::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Category not found after update".into()))
    }

    pub async fn delete(pool: &DbPool, id: &str, account_id: &str) -> AppResult<()> {
        let result = sqlx::query(
            r#"
            DELETE FROM asset_categories
            WHERE id = ? AND account_id = ?
            "#,
        )
        .bind(id)
        .bind(account_id)
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(
                "Category not found or access denied".into(),
            ));
        }

        Ok(())
    }
}

// --- Asset Groupings Repository ---

pub struct AssetGroupingRepository;

impl AssetGroupingRepository {
    pub async fn create(
        pool: &DbPool,
        id: &str,
        account_id: &str,
        asset_master_id: String,
        category_id: String,
    ) -> AppResult<UserAssetGrouping> {
        sqlx::query(
            r#"
            INSERT INTO user_asset_groupings (id, account_id, asset_master_id, category_id)
            VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(id)
        .bind(account_id)
        .bind(asset_master_id)
        .bind(category_id)
        .execute(pool)
        .await?;

        Self::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Created grouping not found".into()))
    }

    pub async fn find_by_account(
        pool: &DbPool,
        account_id: &str,
    ) -> AppResult<Vec<UserAssetGrouping>> {
        let groupings = query_as::<_, UserAssetGrouping>(
            r#"
            SELECT id, account_id, asset_master_id, category_id, created_at, updated_at
            FROM user_asset_groupings
            WHERE account_id = ?
            ORDER BY created_at DESC
            "#,
        )
        .bind(account_id)
        .fetch_all(pool)
        .await?;

        Ok(groupings)
    }

    pub async fn find_by_id(pool: &DbPool, id: &str) -> AppResult<Option<UserAssetGrouping>> {
        let grouping = query_as::<_, UserAssetGrouping>(
            r#"
            SELECT id, account_id, asset_master_id, category_id, created_at, updated_at
            FROM user_asset_groupings
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(grouping)
    }

    pub async fn update(
        pool: &DbPool,
        id: &str,
        account_id: &str,
        category_id: String,
    ) -> AppResult<UserAssetGrouping> {
        let result = sqlx::query(
            r#"
            UPDATE user_asset_groupings
            SET category_id = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ? AND account_id = ?
            "#,
        )
        .bind(category_id)
        .bind(id)
        .bind(account_id)
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(
                "Grouping not found or access denied".into(),
            ));
        }

        Self::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Grouping not found after update".into()))
    }

    pub async fn delete(pool: &DbPool, id: &str, account_id: &str) -> AppResult<()> {
        let result = sqlx::query(
            r#"
            DELETE FROM user_asset_groupings
            WHERE id = ? AND account_id = ?
            "#,
        )
        .bind(id)
        .bind(account_id)
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(
                "Grouping not found or access denied".into(),
            ));
        }

        Ok(())
    }
}

// --- Assets (Holdings) Repository ---

pub struct AssetRepository;

impl AssetRepository {
    pub async fn create(
        pool: &DbPool,
        id: &str,
        account_id: &str,
        asset_master_id: String,
        current_amount: Decimal,
    ) -> AppResult<Asset> {
        sqlx::query(
            r#"
            INSERT INTO assets (id, account_id, asset_master_id, current_amount)
            VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(id)
        .bind(account_id)
        .bind(asset_master_id)
        .bind(current_amount.to_f64().unwrap_or(0.0))
        .execute(pool)
        .await?;

        Self::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Created asset not found".into()))
    }

    pub async fn find_by_account(pool: &DbPool, account_id: &str) -> AppResult<Vec<Asset>> {
        let assets = query_as::<_, Asset>(
            r#"
            SELECT id, account_id, asset_master_id, current_amount, created_at, updated_at
            FROM assets
            WHERE account_id = ?
            ORDER BY created_at DESC
            "#,
        )
        .bind(account_id)
        .fetch_all(pool)
        .await?;

        Ok(assets)
    }

    pub async fn find_by_id(pool: &DbPool, id: &str) -> AppResult<Option<Asset>> {
        let asset = query_as::<_, Asset>(
            r#"
            SELECT id, account_id, asset_master_id, current_amount, created_at, updated_at
            FROM assets
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(asset)
    }

    pub async fn update(
        pool: &DbPool,
        id: &str,
        account_id: &str,
        current_amount: Decimal,
    ) -> AppResult<Asset> {
        let result = sqlx::query(
            r#"
            UPDATE assets
            SET current_amount = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ? AND account_id = ?
            "#,
        )
        .bind(current_amount.to_f64().unwrap_or(0.0))
        .bind(id)
        .bind(account_id)
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(
                "Asset not found or access denied".into(),
            ));
        }

        Self::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Asset not found after update".into()))
    }

    pub async fn delete(pool: &DbPool, id: &str, account_id: &str) -> AppResult<()> {
        let result = sqlx::query(
            r#"
            DELETE FROM assets
            WHERE id = ? AND account_id = ?
            "#,
        )
        .bind(id)
        .bind(account_id)
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(
                "Asset not found or access denied".into(),
            ));
        }

        Ok(())
    }
}
