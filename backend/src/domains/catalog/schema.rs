use serde::Deserialize;

#[derive(Deserialize, utoipa::ToSchema)]
pub struct CreateAssetRequest {
    pub name: String,
    pub ticker_symbol: Option<String>,
}

#[derive(Deserialize, utoipa::ToSchema)]
pub struct UpdateAssetRequest {
    pub name: String,
    pub ticker_symbol: Option<String>,
}
