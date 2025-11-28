use sqlx::{Pool, Postgres};
use std::env;

pub async fn create_db_pool() -> Pool<Postgres> {
    // Load environment variables
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in the .env file");

    // Create a PostgreSQL connection pool
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres. Check your DATABASE_URL.")
}
