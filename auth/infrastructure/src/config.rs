use sea_orm::{ ConnectOptions, Database, DatabaseConnection };
use tracing::info;
use std::time::Duration;
use std::env;
pub async fn db_connection() -> DatabaseConnection {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let mut opt = ConnectOptions::new(&db_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);

    info!("db_url: {}", db_url);
    let db = Database::connect(opt).await.expect("Failed to connect to the database");
    db
}
