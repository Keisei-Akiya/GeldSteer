use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row, sqlite::SqliteRow};

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct AssetCategory {
    pub id: String,
    pub account_id: String,
    pub name: String,
    pub target_ratio: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl<'r> FromRow<'r, SqliteRow> for AssetCategory {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let target_ratio_f64: f64 = row.try_get("target_ratio")?;
        let target_ratio = Decimal::from_f64(target_ratio_f64)
            .ok_or_else(|| sqlx::Error::Decode("Failed to convert f64 to Decimal".into()))?;

        Ok(Self {
            id: row.try_get("id")?,
            account_id: row.try_get("account_id")?,
            name: row.try_get("name")?,
            target_ratio,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, utoipa::ToSchema)]
pub struct UserAssetGrouping {
    pub id: String,
    pub account_id: String,
    pub asset_master_id: String,
    pub category_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Asset {
    pub id: String,
    pub account_id: String,
    pub asset_master_id: String,
    pub current_amount: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl<'r> FromRow<'r, SqliteRow> for Asset {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let current_amount_f64: f64 = row.try_get("current_amount")?;
        let current_amount = Decimal::from_f64(current_amount_f64)
            .ok_or_else(|| sqlx::Error::Decode("Failed to convert f64 to Decimal".into()))?;

        Ok(Self {
            id: row.try_get("id")?,
            account_id: row.try_get("account_id")?,
            asset_master_id: row.try_get("asset_master_id")?,
            current_amount,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}
