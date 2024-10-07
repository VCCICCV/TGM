use axum::{routing::post, Router};

use crate::adapter::auth_handler::register_handler;
pub async fn setup_auth_routes() -> Router {
    Router::new()
        .route("/register", post(register_handler))
        // .route("/login", post(login_handler))
        // .route("/logout", method_router)
        // .route("/refresh-token", post(refresh_token_handler))
}
