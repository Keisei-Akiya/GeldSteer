use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateAccountRequest {
    pub name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct UpdateAccountRequest {
    pub name: String,
    pub email: String,
}
