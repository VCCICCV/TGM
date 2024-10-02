// // 模拟数据库连接和查询结果
// struct MockOrderEntity {
//     id: i32,
//     user_id: i32,
//     total_price: f64,
// }

// impl MockOrderEntity {
//     async fn find_by_id(_id: i32) -> Option<Self> {
//         Some(Self {
//             id: 1,
//             user_id: 2,
//             total_price: 3.0,
//         })
//     }
// }

// // 模拟建立连接函数
// async fn establish_connection() -> Result<(), InfraError> {
//     Ok(())
// }

// #[tokio::test]
// async fn test_find_by_id() {
//     let repository = UserRepositoryImpl {};
//     let result = repository.find_by_id(1).await;
//     assert_matches!(result, Ok(Some(order)) => {
//         assert_eq!(order.id, Some(1));
//         assert_eq!(order.user_id, 2);
//         assert_eq!(order.total_price, 3.0);
//     });
// }