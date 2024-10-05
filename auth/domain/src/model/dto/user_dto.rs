use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserDTO {
    pub username: String,
    pub email: String,
    pub password: String,
}
#[derive(Debug, Clone)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String,
}
#[derive(Serialize, Deserialize)]

pub struct LoginInfoDTO {
    pub username: String,
    pub jwt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
    pub jti:String,// jwt的唯一标识
}
#[derive(Debug, Clone)]
pub struct RegisterUserSchema {
    pub name: String,
    pub email: String,
    pub password: String,
}
