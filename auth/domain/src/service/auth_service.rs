use crate::model::{dto::user_dto::{LoginUserSchema, TokenClaims}, user::User};

pub trait AuthService {
    async fn register_user(&self, user: &User) -> Result<bool, String>;
    async fn login_user(&self, user: &LoginUserSchema) -> Result<(String,String), String>;
    async fn logout_user(&self) -> Result<(), String>;
    async fn get_user_by_token(&self, token: &str) -> Result<Option<User>, String>;
    async fn verify_token(&self, token: &str) -> Result<TokenClaims, String>;
    async fn refresh_token(&self, refresh_token: &str) -> Result<(String, String), String>; // 添加刷新令牌的方法
}