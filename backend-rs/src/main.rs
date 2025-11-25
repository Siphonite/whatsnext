use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Define a simple route
    let app = Router::new().route("/", get(root));

    // Run server on localhost:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    
    println!("Backend running on http://{}", addr);

    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Rust Backend Online"
}