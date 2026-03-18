use axum::{Router, routing::get};
use handler::handler::ping;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/v1/systems/ping", get(|| ping()))
        .route("/v1/companies", get(|| async { "companies" }));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
