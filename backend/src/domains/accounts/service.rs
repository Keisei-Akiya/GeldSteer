use crate::core::database::DbPool;
use crate::core::error::AppResult;
use crate::domains::accounts::model::Account;
use crate::domains::accounts::repository::AccountRepository;
use crate::shared::util::generate_id;

pub struct AccountService;

impl AccountService {
    pub async fn create(pool: &DbPool, name: String, email: String) -> AppResult<Account> {
        let id = generate_id();
        AccountRepository::create(pool, &id, name, email).await
    }

    pub async fn get_all(pool: &DbPool) -> AppResult<Vec<Account>> {
        AccountRepository::find_all(pool).await
    }

    pub async fn get_by_id(pool: &DbPool, id: &str) -> AppResult<Option<Account>> {
        AccountRepository::find_by_id(pool, id).await
    }

    pub async fn update(
        pool: &DbPool,
        id: &str,
        name: String,
        email: String,
    ) -> AppResult<Account> {
        AccountRepository::update(pool, id, name, email).await
    }

    pub async fn delete(pool: &DbPool, id: &str) -> AppResult<()> {
        AccountRepository::delete(pool, id).await
    }
}
