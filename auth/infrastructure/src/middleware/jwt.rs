use chrono::{Duration, Utc};
use common::error::InfraError;
use domain::model::{dto::user_dto::TokenClaims, user::User};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

// 生成jwt
pub async fn encode_jwt(user: User) -> Result<String, InfraError> {
    let claims = TokenClaims {
        sub: user.email,
        iat: Utc::now().timestamp() as usize,
        exp: (Utc::now() + Duration::minutes(1)).timestamp() as usize,
        jti: Uuid::new_v4().to_string(),
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("mykey".as_bytes()),
    )
    .map_err(|e| InfraError::JwtEncodeError(e.to_string()))
}
pub async fn decode_jwt(token: &str) -> Result<TokenClaims, InfraError> {
    let key = "mykey";
    let validation = Validation::default();
    let decoded = decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret(key.as_bytes()),
        &validation,
    )
    .map_err(|e| InfraError::JwtDecodeError(e.to_string()))?;
    Ok(decoded.claims)
}
// async fn refresh_jwt(&self, token: String) -> Result<String, InfraError> {
//     todo!()
// }

// async fn verify_jwt(&self, token: String) -> Result<User, InfraError> {
//     let _ = token;
//     todo!()
// }

// async fn verify_password(&self, password: String, hash: String) -> Result<bool, InfraError> {
//     todo!()

// async fn refresh_jwt(&self, _token: String) -> Result<String, InfraError> {

// }

// async fn verify_jwt(&self, _token: String) -> Result<User, InfraError> {

// }
