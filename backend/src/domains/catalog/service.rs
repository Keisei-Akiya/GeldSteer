use crate::core::database::DbPool;
use crate::core::error::AppResult;
use crate::domains::catalog::model::AssetMaster;
use crate::domains::catalog::repository::AssetMasterRepository;
use crate::shared::util::generate_id;

pub struct AssetMasterService;

impl AssetMasterService {
    pub async fn create(
        pool: &DbPool,
        name: String,
        ticker_symbol: Option<String>,
    ) -> AppResult<AssetMaster> {
        let id = generate_id();
        let asset = AssetMaster {
            id,
            name,
            ticker_symbol,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        AssetMasterRepository::create(pool, &asset).await
    }

    pub async fn get_all(pool: &DbPool) -> AppResult<Vec<AssetMaster>> {
        AssetMasterRepository::find_all(pool).await
    }

    pub async fn get_by_id(pool: &DbPool, id: &str) -> AppResult<Option<AssetMaster>> {
        AssetMasterRepository::find_by_id(pool, id).await
    }

    pub async fn update(
        pool: &DbPool,
        id: &str,
        name: String,
        ticker_symbol: Option<String>,
    ) -> AppResult<AssetMaster> {
        AssetMasterRepository::update(pool, id, &name, ticker_symbol).await
    }

    pub async fn delete(pool: &DbPool, id: &str) -> AppResult<()> {
        AssetMasterRepository::delete(pool, id).await
    }
}
