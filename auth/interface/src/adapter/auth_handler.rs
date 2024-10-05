use application::use_case::user_use_case::UserUseCase;
use axum::{response::IntoResponse, Json};
use domain::model::user::User;

use crate::common::response::Res;

pub async fn register_handler(user: Json<User>) -> impl IntoResponse {
    let user_use_case = UserUseCase::new();
    match user_use_case.register(user.0).await {
        Ok(token) => Res::with_data(token),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

pub async fn refresh_token_handler(Json(refresh_token): Json<String>) -> impl IntoResponse {
    match refresh_token_use_case.refresh(&refresh_token).await {
        Ok((token, refresh_token)) => Res::with_data((token, refresh_token)),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
pub async fn login_handler(user: Json<User>) -> impl IntoResponse {
    let user_use_case = UserUseCase::new();
    match user_use_case.login(user.0).await {
        Ok(token) => Res::with_data(token),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
pub async fn authenticate_handler(user: Json<User>) -> impl IntoResponse {
    let user_use_case = UserUseCase::new();
    match user_use_case.authenticate(user.0).await {
        Ok(_) => Res::with_data("ok"),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
