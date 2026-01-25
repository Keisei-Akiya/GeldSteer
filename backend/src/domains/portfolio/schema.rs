use crate::shared::validation::validate_non_blank;
use rust_decimal::Decimal;
use serde::Deserialize;
use validator::Validate;

// Asset Categories
#[derive(Deserialize, Validate, utoipa::ToSchema)]
pub struct CreateCategoryRequest {
    #[validate(custom(
        function = "validate_non_blank",
        message = "Name cannot be empty or blank"
    ))]
    pub name: String,
    pub target_ratio: Decimal,
}

#[derive(Deserialize, Validate, utoipa::ToSchema)]
pub struct UpdateCategoryRequest {
    #[validate(custom(
        function = "validate_non_blank",
        message = "Name cannot be empty or blank"
    ))]
    pub name: String,
    pub target_ratio: Decimal,
}

// User Asset Groupings
#[derive(Deserialize, Validate, utoipa::ToSchema)]
pub struct CreateGroupingRequest {
    #[validate(custom(
        function = "validate_non_blank",
        message = "Asset Master ID cannot be empty or blank"
    ))]
    pub asset_master_id: String,
    #[validate(custom(
        function = "validate_non_blank",
        message = "Category ID cannot be empty or blank"
    ))]
    pub category_id: String,
}

#[derive(Deserialize, Validate, utoipa::ToSchema)]
pub struct UpdateGroupingRequest {
    #[validate(custom(
        function = "validate_non_blank",
        message = "Category ID cannot be empty or blank"
    ))]
    pub category_id: String,
}

// Assets (Holdings)
#[derive(Deserialize, Validate, utoipa::ToSchema)]
pub struct CreateUserAssetRequest {
    #[validate(custom(
        function = "validate_non_blank",
        message = "Asset Master ID cannot be empty or blank"
    ))]
    pub asset_master_id: String,
    pub current_amount: Decimal,
}

#[derive(Deserialize, Validate, utoipa::ToSchema)]
pub struct UpdateUserAssetRequest {
    pub current_amount: Decimal,
}
