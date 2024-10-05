use domain::{model::user::User, repositories::user_repository::UserRepository, service::auth_service::AuthService};

use super::user_repository_impl::UserRepositoryImpl;

pub struct AuthServiceImpl {
    user_repository: UserRepositoryImpl,
    jwt_secret: String,
    refresh_jwt_secret: String,
}

impl AuthServiceImpl {
    pub fn new(user_repository: UserRepositoryImpl, jwt_secret: String,refresh_jwt_secret:String) -> Self {
        AuthServiceImpl {
            user_repository,
            jwt_secret,
            refresh_jwt_secret,
        }
    }
}

impl AuthService for AuthServiceImpl {
    async fn register_user(&self, user: &User) -> Result<bool, String> {
        self.user_repository.create(user.clone()).await;
        Ok(true)
    }

    async fn login_user(&self, user: &domain::model::dto::user_dto::LoginUserSchema) -> Result<(String,String), String> {
        todo!()
    }
    
    async fn logout_user(&self) -> Result<(), String> {
        todo!()
    }
    
    async fn get_user_by_token(&self, token: &str) -> Result<Option<User>, String> {
        todo!()
    }
    
    async fn verify_token(&self, token: &str) -> Result<domain::model::dto::user_dto::TokenClaims, String> {
        todo!()
    }
    
    async fn refresh_token(&self, refresh_token: &str) -> Result<(String, String), String> {
        todo!()
    }
    
    // async fn login_user(&self, user_schema: &LoginUserSchema) -> Result<String, String> {
    //     let user = self.user_repository.find_by_email(user_schema.email).await?;
    //     let user = user.ok_or_else(|| "Invalid email or password".to_string())?;
    //     let is_valid = match PasswordHash::new(&user.password) {
    //         Ok(parsed_hash) => Argon2::default()
    //            .verify_password(user_schema.password.as_bytes(), &parsed_hash)
    //            .map_or(false, |_| true),
    //         Err(_) => false,
    //     };
    //     if!is_valid {
    //         return Err("Invalid email or password".to_string());
    //     }
    //     let now = Utc::now();
    //     let iat = now.timestamp() as usize;
    //     let exp = (now + Duration::minutes(60)).timestamp() as usize;
    //     let claims = TokenClaims {
    //         sub: user.id.to_string(),
    //         exp,
    //         iat,
    //     };
    //     let token = encode(
    //         &Header::default(),
    //         &claims,
    //         &EncodingKey::from_secret(self.jwt_secret.as_ref()),
    //     )
    //    .map_err(|e| format!("Error encoding token: {}", e))?;
    //     Ok(token)
    // }

    // async fn logout_user(&self) -> Result<(), String> {
    //     // 在实际应用中，可能需要更多的逻辑来处理注销（如清除缓存中的用户信息等）
    //     Ok(())
    // }

    // async fn get_user_by_token(&self, token: &str) -> Result<Option<User>, String> {
    //     let claims = self.verify_token(token).await?;
    //     let user_id = Uuid::parse_str(&claims.sub).map_err(|_| "Invalid token".to_string())?;
    //     self.user_repository.find_user_by_id(user_id).await
    // }

    // async fn verify_token(&self, token: &str) -> Result<TokenClaims, String> {
    //     let claims = decode::<TokenClaims>(
    //         &token,
    //         &DecodingKey::from_secret(self.jwt_secret.as_ref()),
    //         &Validation::default(),
    //     )
    //    .map_err(|e| format!("Invalid token: {}", e))?
    //    .claims;
    //     Ok(claims)
    // }
}