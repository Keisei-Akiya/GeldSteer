use crate::core::database::DbPool;
use crate::core::error::AppResult;
use crate::domains::catalog::model::AssetMaster;
use crate::domains::catalog::repository;
use crate::shared::util::generate_id;

pub async fn create_asset(pool: &DbPool, name: String, ticker_symbol: Option<String>) -> AppResult<AssetMaster> {
    let id = generate_id();
    let asset = AssetMaster {
        id,
        name,
        ticker_symbol,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    repository::create(pool, &asset).await
}

pub async fn get_all_assets(pool: &DbPool) -> AppResult<Vec<AssetMaster>> {
    repository::find_all(pool).await
}

pub async fn get_asset_by_id(pool: &DbPool, id: &str) -> AppResult<Option<AssetMaster>> {
    repository::find_by_id(pool, id).await
}

pub async fn update_asset(pool: &DbPool, id: &str, name: String, ticker_symbol: Option<String>) -> AppResult<AssetMaster> {
    repository::update(pool, id, &name, ticker_symbol).await
}

pub async fn delete_asset(pool: &DbPool, id: &str) -> AppResult<()> {
    repository::delete(pool, id).await
}
