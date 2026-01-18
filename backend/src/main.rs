use axum::Router;
use dotenvy::dotenv;

mod core;
mod domains;
mod shared;

use crate::core::database::init_db;
use crate::domains::catalog::catalog_routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = init_db().await;

    // TODO: Register other domain routes (accounts, portfolio) when they are implemented
    let app = Router::new()
        .nest("/api/v1/catalog", catalog_routes(pool));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Server running on http://localhost:8000");
    axum::serve(listener, app).await.unwrap();
}
