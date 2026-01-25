use crate::shared::validation::validate_non_blank;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, utoipa::ToSchema)]
pub struct CreateAccountRequest {
    #[validate(custom(
        function = "validate_non_blank",
        message = "Name cannot be empty or blank"
    ))]
    pub name: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

#[derive(Deserialize, Validate, utoipa::ToSchema)]
pub struct UpdateAccountRequest {
    #[validate(custom(
        function = "validate_non_blank",
        message = "Name cannot be empty or blank"
    ))]
    pub name: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}
