use axum::{routing::get, Router};
use tokio::net::TcpListener;

mod config;

#[tokio::main]
async fn main() {
    // Load environment config
    let cfg = config::AppConfig::load();

    println!("Loaded config:");
    println!("RPC URL: {}", cfg.rpc_url);
    println!("Program ID: {}", cfg.program_id);

    // Define routes
    let app = Router::new().route("/", get(root));

    // Bind listener for axum 0.7
    let listener = TcpListener::bind(("127.0.0.1", cfg.backend_port))
        .await
        .unwrap();

    println!("Backend running on http://127.0.0.1:{}", cfg.backend_port);

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Rust Backend Online 🚀"
}
