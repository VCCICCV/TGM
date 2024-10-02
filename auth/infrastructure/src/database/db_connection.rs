use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::env;
use std::time::Duration;
use tracing::info;
// 默认有连接池
pub async fn establish_db_connection() -> Result<DatabaseConnection, DbErr> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    // 可选配置
    let mut config = ConnectOptions::new(&db_url);
    config
        .max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false); // 关闭 sqlx 的日志输出，默认打印到控制台
    let db_connection = Database::connect(config).await.unwrap();
    info!("Database connected");
    Ok(db_connection)
}