use bb8_redis::RedisConnectionManager;
use common::error::InfraError;
use std::env;
use tracing::info;
// 建立redis连接池
pub async fn establish_redis_pool_connection(
) -> Result<bb8::Pool<RedisConnectionManager>, InfraError> {
    // let redis_url = env::var("REDIS_URL").map_err(|e| {
    //     Err(InfraError::RedisError(
    //         "REDIS_URL is not set in.env file".to_string(),
    //     ))
    // });
    // let manager = RedisConnectionManager::new(redis_url.as_str()).or_else(|e| {
    //     Err(InfraError::RedisError(
    //         "Failed to create Redis connection manager".to_string(),
    //     ))
    // });
    // let pool = bb8::Pool::builder().build(manager).await.or_else(|e| {
    //     Err(InfraError::RedisError(
    //         "REDIS_URL is not set in.env file".to_string(),
    //     ))
    // });
    // info!("Redis connected");
    // Ok(pool)

    let redis_url = env::var("REDIS_URL")
        .map_err(|e| InfraError::RedisError(format!("REDIS_URL is not set in.env file: {}", e)))?;
    let manager = RedisConnectionManager::new(redis_url.as_str()).or_else(|e| {
        Err(InfraError::RedisError(format!(
            "Failed to create Redis connection manager: {}",
            e
        )))
    })?;
    let pool = bb8::Pool::builder().build(manager).await.or_else(|e| {
        Err(InfraError::RedisError(format!(
            "Failed to build Redis connection pool: {}",
            e
        )))
    })?;
    info!("Redis connected");
    Ok(pool)
}
