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

pub fn catalog_routes(pool: DbPool) -> Router {
    Router::new()
        .route("/", post(handler::create_asset).get(handler::list_assets))
        .route(
            "/{id}",
            get(handler::get_asset)
                .put(handler::update_asset)
                .delete(handler::delete_asset),
        )
        .with_state(pool)
}
