// use common::error::InfraError;

// use crate::model::{dto::user_dto::LoginUserSchema, user::User};

// pub trait AuthService {
//     async fn register_user(&self, user: &User) -> Result<User, InfraError>;
//     async fn login_user(&self, user: &LoginUserSchema) -> Result<String, InfraError>;
//     async fn logout_user(&self) -> Result<(), String>;
//     async fn verify_token(&self, token: &str) -> Result<bool, String>;
//     async fn refresh_token(&self, refresh_token: &str) -> Result<String, String>; // 添加刷新令牌的方法
// }