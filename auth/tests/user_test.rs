// #[cfg(test)]
// mod tests {
//     use super::*;
//     use anyhow::Result;
//     use sea_orm::{ DatabaseConnection, DbErr };

//     async fn mock_establish_connection() -> Result<DatabaseConnection, DbErr> {
//         // 返回一个模拟的数据库连接，或者可以使用测试数据库连接
//         // 这里只是为了测试目的，实际情况可能需要连接到测试数据库
//         unimplemented!();
//     }

//     #[tokio::test]
//     async fn test_find_all() {
//         // 创建 UserRepositoryImpl 的实例并临时替换 establish_connection 函数
//         let mut repository = UserRepositoryImpl {};
//         let original_establish_connection =
//             *crate::infrastructure::repositories::connection::establish_connection;
//         *crate::infrastructure::repositories::connection::establish_connection =
//             mock_establish_connection;

//         let result = repository.find_all().await;

//         // 恢复原始的 establish_connection 函数
//         *crate::infrastructure::repositories::connection::establish_connection =
//             original_establish_connection;

//         assert!(result.is_ok());
//         // 根据实际情况添加更多的断言来验证结果
//     }
// }
