use axum::Router;
use dotenvy::dotenv;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod core;
mod domains;
mod middleware;
mod shared;

use crate::core::database::init_db;
use crate::domains::accounts::account_routes;
use crate::domains::catalog::catalog_routes;
use crate::domains::portfolio::portfolio_routes;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::domains::catalog::handler::create_asset,
        crate::domains::catalog::handler::list_assets,
        crate::domains::catalog::handler::get_asset,
        crate::domains::catalog::handler::update_asset,
        crate::domains::catalog::handler::delete_asset,
        crate::domains::accounts::handler::create_account,
        crate::domains::accounts::handler::list_accounts,
        crate::domains::accounts::handler::get_account,
        crate::domains::accounts::handler::update_account,
        crate::domains::accounts::handler::delete_account,
        crate::domains::portfolio::handler::create_category,
        crate::domains::portfolio::handler::list_categories,
        crate::domains::portfolio::handler::get_category,
        crate::domains::portfolio::handler::update_category,
        crate::domains::portfolio::handler::delete_category,
        crate::domains::portfolio::handler::create_grouping,
        crate::domains::portfolio::handler::list_groupings,
        crate::domains::portfolio::handler::get_grouping,
        crate::domains::portfolio::handler::update_grouping,
        crate::domains::portfolio::handler::delete_grouping,
        crate::domains::portfolio::handler::create_user_asset,
        crate::domains::portfolio::handler::list_user_assets,
        crate::domains::portfolio::handler::get_user_asset,
        crate::domains::portfolio::handler::update_user_asset,
        crate::domains::portfolio::handler::delete_user_asset,
    ),
    components(
        schemas(
            crate::domains::catalog::model::AssetMaster,
            crate::domains::catalog::schema::CreateAssetRequest,
            crate::domains::catalog::schema::UpdateAssetRequest,
            crate::domains::accounts::model::Account,
            crate::domains::accounts::schema::CreateAccountRequest,
            crate::domains::accounts::schema::UpdateAccountRequest,
            crate::domains::portfolio::model::AssetCategory,
            crate::domains::portfolio::model::UserAssetGrouping,
            crate::domains::portfolio::model::Asset,
            crate::domains::portfolio::schema::CreateCategoryRequest,
            crate::domains::portfolio::schema::UpdateCategoryRequest,
            crate::domains::portfolio::schema::CreateGroupingRequest,
            crate::domains::portfolio::schema::UpdateGroupingRequest,
            crate::domains::portfolio::schema::CreateUserAssetRequest,
            crate::domains::portfolio::schema::UpdateUserAssetRequest,
        )
    ),
    tags(
        (name = "catalog", description = "Asset Catalog Management"),
        (name = "accounts", description = "Account Management"),
        (name = "portfolio", description = "Portfolio Management")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let pool = init_db().await;

    // TODO: Register other domain routes (accounts, portfolio) when they are implemented
    let app = Router::new()
        .nest("/api/v1/catalog", catalog_routes(pool.clone()))
        .nest("/api/v1/accounts", account_routes(pool.clone()))
        .nest("/api/v1/portfolio", portfolio_routes(pool))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    let addr = "0.0.0.0:8000";
    let listener = tokio::net::TcpListener::bind(addr).await.map_err(|e| {
        eprintln!("Failed to bind to address {}: {}", addr, e);
        e
    })?;

    println!("Server running on http://localhost:8000");
    axum::serve(listener, app).await.map_err(|e| {
        eprintln!("Server error: {}", e);
        e
    })?;

    Ok(())
}
