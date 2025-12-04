use dotenvy::dotenv;
use std::env;

#[derive(Clone)]
pub struct AppConfig {
    pub rpc_url: String,
    pub program_id: String,
    pub admin_keypair: String,
    #[allow(dead_code)]
    pub backend_port: u16,
}

// Single-asset MVP — only BTC/USDT is used everywhere in backend
pub const MARKET_ASSET: &str = "BTC/USDT";

impl AppConfig {
    pub fn load() -> Self {
        dotenv().ok(); // Load .env variables

        let rpc_url = env::var("RPC_URL")
            .expect("RPC_URL must be set");

        let program_id = env::var("PROGRAM_ID")
            .expect("PROGRAM_ID must be set");

        let admin_keypair = env::var("ADMIN_KEYPAIR")
            .expect("ADMIN_KEYPAIR must be set");

        let backend_port = env::var("BACKEND_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .expect("Invalid port number");

        AppConfig {
            rpc_url,
            program_id,
            admin_keypair,
            backend_port,
        }
    }
}
