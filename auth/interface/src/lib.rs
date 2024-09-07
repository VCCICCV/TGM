use axum::{ routing::get, Router, extract::State };
use sea_orm::DatabaseConnection;
use tracing::info;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{ fmt, layer::SubscriberExt, util::SubscriberInitExt };
use std::env;
use serde::Serialize;
use serde_json::json;
use infrastructure::config::db_connection;
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
        db: db_connection().await,
    };

    let app = Router::new()
        .route("/auth", get(hello))
        // .merge("/post", )
        // .merge("/post", )
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
    // 调用 `tracing` 包的 `info!`
    info!("🚀 listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
    Ok(())
}

/// 响应结构体
#[derive(Debug, Serialize)]
struct ResJson {
    code: i32,
    data: String,
    message: String,
}
/// handler
async fn hello(state: axum::extract::State<AppState>) -> String {
    let res_json = ResJson {
        code: 200,
        data: json!({
            "name":"cci",
            "age":18,
        }).to_string(),
        message: "success".to_string(),
    };
    let json_string = json!(res_json).to_string();
    info!("hello tracing");
    println!("{:?}", state);
    json_string
}
