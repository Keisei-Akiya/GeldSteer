pub mod handler;
pub mod model;
pub mod repository;
pub mod schema;
pub mod service;

use crate::core::database::DbPool;
use crate::domains::portfolio::handler::{
    AssetCategoryHandler, AssetGroupingHandler, AssetHandler,
};
use axum::{
    Router,
    routing::{get, post},
};

pub fn portfolio_routes(pool: DbPool) -> Router {
    Router::new()
        // Categories
        .route(
            "/categories",
            post(AssetCategoryHandler::create).get(AssetCategoryHandler::list),
        )
        .route(
            "/categories/{id}",
            get(AssetCategoryHandler::get)
                .put(AssetCategoryHandler::update)
                .delete(AssetCategoryHandler::delete),
        )
        // Groupings
        .route(
            "/groupings",
            post(AssetGroupingHandler::create).get(AssetGroupingHandler::list),
        )
        .route(
            "/groupings/{id}",
            get(AssetGroupingHandler::get)
                .put(AssetGroupingHandler::update)
                .delete(AssetGroupingHandler::delete),
        )
        // Assets
        .route(
            "/assets",
            post(AssetHandler::create).get(AssetHandler::list),
        )
        .route(
            "/assets/{id}",
            get(AssetHandler::get)
                .put(AssetHandler::update)
                .delete(AssetHandler::delete),
        )
        .with_state(pool)
}
