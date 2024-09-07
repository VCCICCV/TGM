/*
 * @Author: cci
 * @Date: 2024-08-25 02:37:22
 * @LastEditors: cci
 * @LastEditTime: 2024-09-07 19:30:18
 * @Description:
 *
 * Copyright (c) 2024 by TGM All Rights Reserved.
 */
// use axum::{ routing::get, Router };
// use tracing::info;
// use tower_http::trace::TraceLayer;
// use tracing_subscriber::{ fmt, layer::SubscriberExt,util::SubscriberInitExt};
// async fn hello()-> String{
//     info!("hello tracing");
//     "hello".to_string()
// }
// use std::env;
// #[tokio::main]
// async fn main() {

//     for (key, value) in env::vars() {
//         println!("{key}: {value}");
//     }

//     // 只有注册 subscriber 后， 才能在控制台上看到日志输出
//     tracing_subscriber::registry().with(fmt::layer()).init();

//     let app = Router::new().route("/",get(hello));

//     let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
//     // 调用 `tracing` 包的 `info!`
//     info!("🚀 listening on {}", listener.local_addr().unwrap());

//     axum::serve(listener, app).await.unwrap();
// }



use interface;
fn main() {
    interface::main();
}
