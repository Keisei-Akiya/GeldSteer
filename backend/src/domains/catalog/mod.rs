pub mod handler;
pub mod model;
pub mod repository;
pub mod schema;
pub mod service;

use crate::core::database::DbPool;
use crate::domains::catalog::handler::AssetMasterHandler;
use axum::{
    Router,
    routing::{get, post},
};

pub fn catalog_routes(pool: DbPool) -> Router {
    Router::new()
        .route(
            "/",
            post(AssetMasterHandler::create).get(AssetMasterHandler::list),
        )
        .route(
            "/{id}",
            get(AssetMasterHandler::get)
                .put(AssetMasterHandler::update)
                .delete(AssetMasterHandler::delete),
        )
        .with_state(pool)
}
