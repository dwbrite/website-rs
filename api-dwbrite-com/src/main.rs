mod blog;

use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    axum::Server::bind(&"0.0.0.0:41230".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}