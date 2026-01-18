use axum::{Json, Router, routing::get};
use serde::Serialize;

#[derive(Serialize)]
struct Message {
    message: String,
}

#[tokio::main]
async fn main() {
    // 1. ルーティングの設定
    let app = Router::new().route("/", get(handler));

    // 2. サーバーの起動
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Server running on http://localhost:8000");
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Json<Message> {
    Json(Message {
        message: "Hello, Rust API!".to_string(),
    })
}
