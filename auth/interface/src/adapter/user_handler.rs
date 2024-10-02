use crate::common::response::Res;
use application::use_case::user_use_case::UserUseCase;
use axum::response::IntoResponse;
pub use domain::model::user::User;
pub use domain::service::command_user_service::CommandUserService;
pub use domain::service::query_user_service::QueryUserService;
/// DI：我们把查询用户的trait和命令用户的trait注入到handler中
pub async fn list_users() -> impl IntoResponse {
    let use_case = UserUseCase::new();
    let users = use_case.list_users().await.unwrap();
    // json!(users).to_string()
    Res::with_data(users)
}

pub async fn get_user(id: axum::extract::Path<i32>) -> impl IntoResponse {
    let use_case = UserUseCase::new();
    let user = use_case.get_user_by_id(*id).await.unwrap();
    // json!(user).to_string()
    Res::with_data(user)
}

pub async fn create_user(user: axum::extract::Json<User>) -> impl IntoResponse {
    let use_case = UserUseCase::new();
    let result = use_case.create_user(user.0).await;
    match result {
        Ok(new_user) => Res::with_data(new_user),
        Err(err) => Res::with_err(&err.to_string()),
    }
}
pub async fn update_user(user: axum::extract::Json<User>) -> impl IntoResponse {
    let use_case = UserUseCase::new();
    let result = use_case.update_user(user.0).await;
    match result {
        Ok(new_user) => Res::with_data(new_user),
        Err(err) => Res::with_err(&err.to_string()),
    }
}
pub async fn delete_user(id: axum::extract::Path<i32>) -> impl IntoResponse {
    let use_case = UserUseCase::new();
    let result = use_case.delete_user(*id).await;
    match result {
        Ok(deleted) => {
            if deleted {
                Res::with_data("User deleted successfully")
            } else {
                Res::with_err("User not found or deletion failed")
            }
        }
        Err(err) => Res::with_err(&err.to_string()),
    }
}
