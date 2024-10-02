//! 基础设施层
//! 底层具体技术实现
mod entities;

/// 数据库连接
pub mod database{
    pub mod db_connection;
    pub mod redis_connection;
}
/// 工具类
pub mod utils{
    pub mod redis_util;
}
/// 与表的映射实体
// pub mod entities{
//     // pub mod prelude;
//     // pub mod user;
// }
/// 持久层具体实现
pub mod persistence{
    // 不实现CQRS
    // pub mod user_repository_impl;
    // CQRS，将存储库的操作抽象为命令和查询
    pub mod command_user_repository_impl;
    pub mod query_user_repository_impl;
}
// RPC调用
pub mod remote{}