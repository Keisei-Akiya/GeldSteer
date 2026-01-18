use axum::Router;
use dotenvy::dotenv;

mod core;
mod domains;
mod shared;

use crate::core::database::init_db;
use crate::domains::catalog::catalog_routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let pool = init_db().await;

    // TODO: Register other domain routes (accounts, portfolio) when they are implemented
    let app = Router::new().nest("/api/v1/catalog", catalog_routes(pool));

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
