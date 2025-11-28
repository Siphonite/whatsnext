use backend_rs::db::create_db_pool;

#[tokio::main]
async fn main() {
    let pool = create_db_pool().await;

    println!("Connected! Fetching tables...");

    let rows = sqlx::query!(
        r#"
        SELECT table_name 
        FROM information_schema.tables 
        WHERE table_schema = 'public'
        ORDER BY table_name
        "#
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch tables");

    for r in rows {
        println!("Table: {}", r.table_name.unwrap_or("<none>".into()));
    }
}
