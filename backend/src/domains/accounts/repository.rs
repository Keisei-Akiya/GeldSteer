use crate::core::database::DbPool;
use crate::core::error::{AppError, AppResult};
use crate::domains::accounts::model::Account;
use sqlx::query_as;

pub async fn create(pool: &DbPool, id: &str, name: String, email: String) -> AppResult<Account> {
    sqlx::query(
        r#"
        INSERT INTO accounts (id, name, email)
        VALUES (?, ?, ?)
        "#,
    )
    .bind(id)
    .bind(name)
    .bind(email)
    .execute(pool)
    .await?;

    find_by_id(pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound("Created account not found".into()))
}

pub async fn find_all(pool: &DbPool) -> AppResult<Vec<Account>> {
    let accounts = query_as::<_, Account>(
        r#"
        SELECT id, name, email, created_at, updated_at
        FROM accounts
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(accounts)
}

pub async fn find_by_id(pool: &DbPool, id: &str) -> AppResult<Option<Account>> {
    let account = query_as::<_, Account>(
        r#"
        SELECT id, name, email, created_at, updated_at
        FROM accounts
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(account)
}

pub async fn update(pool: &DbPool, id: &str, name: String, email: String) -> AppResult<Account> {
    sqlx::query(
        r#"
        UPDATE accounts
        SET name = ?, email = ?, updated_at = CURRENT_TIMESTAMP
        WHERE id = ?
        "#,
    )
    .bind(name)
    .bind(email)
    .bind(id)
    .execute(pool)
    .await?;

    find_by_id(pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound("Account not found after update".into()))
}

pub async fn delete(pool: &DbPool, id: &str) -> AppResult<()> {
    let result = sqlx::query(
        r#"
        DELETE FROM accounts
        WHERE id = ?
        "#,
    )
    .bind(id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!(
            "Account with id {} not found",
            id
        )));
    }

    Ok(())
}
