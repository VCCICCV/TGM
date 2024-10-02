use std::error::Error;

use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use redis::AsyncCommands;

use crate::database::redis_connection::establish_redis_pool_connection;

pub struct RedisUtil {
    pool: Pool<RedisConnectionManager>,
}
impl RedisUtil {
    // 创建一个新的RedisUtil实例，建立连接池
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let pool = establish_redis_pool_connection().await?;
        Ok(RedisUtil { pool })
    }

    // 设置键值对
    pub async fn set(&self, key: &str, value: &str) -> Result<(), Box<dyn Error>> {
        let mut conn = self.pool.get().await?;
        let value = conn.set(key, value).await?;
        Ok(value)
    }

    // 获取键对应的值
    pub async fn get(&self, key: &str) -> Result<Option<String>, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;
        let value: Option<String> = conn.get(key).await?;
        Ok(value)
    }

    // 删除键
    pub async fn del(&self, key: &str) -> Result<usize, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;
        let num_deleted: usize = conn.del(key).await?;
        Ok(num_deleted)
    }
}
