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

pub fn account_routes(pool: DbPool) -> Router {
    Router::new()
        .route(
            "/",
            post(handler::create_account).get(handler::list_accounts),
        )
        .route(
            "/{id}",
            get(handler::get_account)
                .put(handler::update_account)
                .delete(handler::delete_account),
        )
        .with_state(pool)
}
