use crate::core::database::DbPool;
use crate::core::error::{AppError, AppResult};
use crate::domains::catalog::model::AssetMaster;
use sqlx::query_as;

pub async fn create(pool: &DbPool, asset: &AssetMaster) -> AppResult<AssetMaster> {
    sqlx::query(
        r#"
        INSERT INTO asset_master (id, name, ticker_symbol)
        VALUES (?, ?, ?)
        "#,
    )
    .bind(&asset.id)
    .bind(&asset.name)
    .bind(&asset.ticker_symbol)
    .execute(pool)
    .await?;

    find_by_id(pool, &asset.id).await?.ok_or_else(|| AppError::NotFound("Created asset not found".into()))
}

pub async fn find_all(pool: &DbPool) -> AppResult<Vec<AssetMaster>> {
    let assets = query_as::<_, AssetMaster>(
        r#"
        SELECT id, name, ticker_symbol, created_at, updated_at
        FROM asset_master
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(assets)
}

pub async fn find_by_id(pool: &DbPool, id: &str) -> AppResult<Option<AssetMaster>> {
    let asset = query_as::<_, AssetMaster>(
        r#"
        SELECT id, name, ticker_symbol, created_at, updated_at
        FROM asset_master
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(asset)
}

pub async fn update(pool: &DbPool, id: &str, name: &str, ticker_symbol: Option<String>) -> AppResult<AssetMaster> {
        sqlx::query(
        r#"
        UPDATE asset_master
        SET name = ?, ticker_symbol = ?
        WHERE id = ?
        "#,
    )
    .bind(name)
    .bind(ticker_symbol)
    .bind(id)
    .execute(pool)
    .await?;

    find_by_id(pool, id).await?.ok_or(AppError::NotFound("Asset not found after update".into()))
}

pub async fn delete(pool: &DbPool, id: &str) -> AppResult<()> {
    let result = sqlx::query("DELETE FROM asset_master WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Asset with id {} not found", id)));
    }

    Ok(())
}
