// use application::service::query_order_service::QueryOrderService;
// use axum::response::IntoResponse;
// // use application::service::query_order_service::CommandOrderService;
// use domain::{event::event_bus::order_event_bus::SimpleEventBus, repositories::query_order_repository::QueryOrderRepository};
// use crate::common::response::Res;
// pub async fn get_order(id: axum::extract::Path<i32>) -> impl IntoResponse {

// }
// DI
// pub async fn list_orders() -> impl IntoResponse {
//     let orders = OrderService::new().find_all_orders().await.unwrap();
//     Res::with_data(orders)
// }

// pub async fn create_order(order: axum::extract::Json<Order>) -> impl IntoResponse {
//     // let result = CommandOrderService::new().create_order(order.0).await;
//     // match result {
//     //     Ok(new_order) => Res::with_data(new_order),
//     //     Err(err) => Res::with_err(&err.to_string()),
//     // }
//     let command_service = CommandOrderService::new(/*传入具体的命令存储库实现*/);
//     let result = command_service.create_order(order.0).await;
//     match result {
//         Ok(new_order) => new_order,
//         Err(err) => Err(err),
//     }
// }

// pub async fn update_order(order: axum::extract::Json<Order>) -> impl IntoResponse {
//     let result = OrderService::new().update_order(order.0).await;
//     match result {
//         Ok(new_order) => Res::with_data(new_order),
//         Err(err) => Res::with_err(&err.to_string()),
//     }
// }

// pub async fn delete_order(id: axum::extract::Path<i32>) -> impl IntoResponse {
//     let result = OrderService::new().delete_order(*id).await;
//     match result {
//         Ok(deleted) => {
//             if deleted {
//                 Res::with_data("order deleted successfully")
//             } else {
//                 Res::with_err("order not found or deletion failed")
//             }
//         }
//         Err(err) => Res::with_err(&err.to_string()),
//     }
// }
