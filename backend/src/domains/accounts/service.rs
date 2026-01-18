use crate::core::database::DbPool;
use crate::core::error::AppResult;
use crate::domains::accounts::model::Account;
use crate::domains::accounts::repository;
use crate::shared::util::generate_id;

pub async fn create_account(pool: &DbPool, name: String, email: String) -> AppResult<Account> {
    let id = generate_id();
    repository::create(pool, &id, name, email).await
}

pub async fn get_all_accounts(pool: &DbPool) -> AppResult<Vec<Account>> {
    repository::find_all(pool).await
}

pub async fn get_account_by_id(pool: &DbPool, id: &str) -> AppResult<Option<Account>> {
    repository::find_by_id(pool, id).await
}

pub async fn update_account(
    pool: &DbPool,
    id: &str,
    name: String,
    email: String,
) -> AppResult<Account> {
    repository::update(pool, id, name, email).await
}

pub async fn delete_account(pool: &DbPool, id: &str) -> AppResult<()> {
    repository::delete(pool, id).await
}
