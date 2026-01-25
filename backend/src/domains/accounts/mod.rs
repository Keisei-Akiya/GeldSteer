pub mod handler;
pub mod model;
pub mod repository;
pub mod schema;
pub mod service;

use crate::core::database::DbPool;
use crate::domains::accounts::handler::AccountHandler;
use axum::{
    Router,
    routing::{get, post},
};

pub fn account_routes(pool: DbPool) -> Router {
    Router::new()
        .route("/", post(AccountHandler::create).get(AccountHandler::list))
        .route(
            "/{id}",
            get(AccountHandler::get)
                .put(AccountHandler::update)
                .delete(AccountHandler::delete),
        )
        .with_state(pool)
}
