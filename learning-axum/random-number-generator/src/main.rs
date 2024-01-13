use std::net::SocketAddr;

use axum::{routing::get, Router, extract::Query, response::Html};
use rand::{thread_rng, Rng};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize)]
struct RangeParameters {
    start: usize,
    end: usize,
}

// http://127.0.0.1:3000/?start=30&end=35
async fn handler(Query(range):Query<RangeParameters>) -> Html<String> {
    let random_number = thread_rng().gen_range(range.start..range.end);
    Html(format!("<h1>Random Number: {}</h1>", random_number))
}
