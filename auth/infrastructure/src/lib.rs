//! 基础设施层
//! 底层具体技术实现
// po
mod entities;

/// 数据库连接
pub mod database{
    pub mod db_connection;
    pub mod redis_connection;
}
/// 工具类
pub mod utils{
    pub mod redis_util;
    pub mod jwt_util;
    pub mod password_util;
}
/// 与表的映射实体
// pub mod entities{
//     // pub mod prelude;
//     // pub mod user;
// }
/// 持久层具体实现
pub mod persistence{
    pub mod user_repository_impl;
    pub mod auth_repository_impl;
}
// RPC调用
pub mod remote{}