pub mod handler;
pub mod model;
pub mod repository;
pub mod schema;
pub mod service;

use crate::core::database::DbPool;

use axum::{
    Router,
    routing::{get, post},
};

pub fn portfolio_routes(pool: DbPool) -> Router {
    Router::new()
        // Categories
        .route(
            "/categories",
            post(handler::create_category).get(handler::list_categories),
        )
        .route(
            "/categories/{id}",
            get(handler::get_category)
                .put(handler::update_category)
                .delete(handler::delete_category),
        )
        // Groupings
        .route(
            "/groupings",
            post(handler::create_grouping).get(handler::list_groupings),
        )
        .route(
            "/groupings/{id}",
            get(handler::get_grouping)
                .put(handler::update_grouping)
                .delete(handler::delete_grouping),
        )
        // Assets
        .route(
            "/assets",
            post(handler::create_user_asset).get(handler::list_user_assets),
        )
        .route(
            "/assets/{id}",
            get(handler::get_user_asset)
                .put(handler::update_user_asset)
                .delete(handler::delete_user_asset),
        )
        .with_state(pool)
}
