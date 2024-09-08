use axum::{
    body,
    extract::{ Query, State },
    http::{ HeaderMap, StatusCode },
    response::{ IntoResponse, Response },
    routing::{ delete, get, post, put },
    Router,
};
use sea_orm::DatabaseConnection;
use serde::{ Deserialize, Serialize };
use tracing::info;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{ fmt, layer::SubscriberExt, util::SubscriberInitExt };
use std::env;
use application::common::res::ResJson;
use serde_json::json;
use infrastructure::{ config::db_config::get_db_connection, entities::user };
use application::service::user_service::UserService;
// 状态路由
#[derive(Clone, Debug)]
pub struct AppState {
    db: DatabaseConnection,
}
/// 状态路由， 用于共享状态，需要返回路由时使用
// type StateRouter = Router<AppState>;
pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Start Error: {err}");
    }
}
#[tokio::main]
pub async fn start() -> anyhow::Result<()> {
    // 加载.env 文件，成功返回包含的值，失败返回None
    dotenvy::dotenv().ok();

    // 读取日志级别
    let rust_log = env::var("RUST_LOG").unwrap_or("debug".to_string());
    env::set_var("RUST_LOG", &rust_log);

    // 只有注册 subscriber 后， 才能在控制台上看到日志输出
    tracing_subscriber::registry().with(fmt::layer()).init();

    // 读取值
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    // 状态路由
    let state = AppState {
        db: get_db_connection().await,
    };
    //  跨域
    let cors = CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any)
        .allow_headers(Any);
    let app = Router::new()
        .route("/users", get(UserService::get_users_list))
        // .route("/users", post(UserService::create_user))
        .route("/users/:id", get(UserService::get_user_by_id))
        .route("/users/:id", put(UserService::update_user))
        .route("/users/:id", delete(UserService::delete_user))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();

    // 调用 `tracing` 包的 `info!`
    info!("🚀 listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
    Ok(())
}
// 后备路由
async fn handle_404() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not found")
}