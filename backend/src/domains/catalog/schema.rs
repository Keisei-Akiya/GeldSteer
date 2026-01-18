use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateAssetRequest {
    pub name: String,
    pub ticker_symbol: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateAssetRequest {
    pub name: String,
    pub ticker_symbol: Option<String>,
}
