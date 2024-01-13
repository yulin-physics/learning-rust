use std::net::SocketAddr;

use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));
    let addr = SocketAddr::from(([127,0,0,1], 3000));
    
    // axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn handler() -> &'static str {
    "Hello, world!"
}