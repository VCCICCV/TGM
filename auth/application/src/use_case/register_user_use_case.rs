use common::error::AppError;
use domain::{model::dto::user_dto::RegisterUserSchema, repositories::user_repository::UserRepository, service::auth_service::AuthService};

pub struct RegisterUserUseCase<U: UserRepository, A: AuthService> {
    user_repository: U,
    auth_service: A,
}
impl<U: UserRepository, A: AuthService> RegisterUserUseCase<U, A> {
    pub fn new(user_repository: U, auth_service: A) -> Self {
        RegisterUserUseCase {
            user_repository,
            auth_service,
        }
    }
    // 注册用户
    pub async fn register(&self, user_schema: &RegisterUserSchema) -> Result<String, String> {
        // 先查询是否存在
        if self.user_repository.user_exists(user_schema.email).await? {
            return Err(AppError::RegisterError("User with that email already exists".to_owned()));
        }
        let hashed_password = self.hash_password(&user_schema.password).await?;
        let user = User {
            id: uuid::Uuid::new_v4(),
            name: user_schema.name.clone(),
            email: user_schema.email.clone(),
            password: hashed_password,
            role: "user".to_string(),
            verified: false,
            created_at: Some(chrono::Utc::now()),
            updated_at: Some(chrono::Utc::now()),
        };
        self.auth_service.register_user(&user).await;
        self.user_repository.generate_jwt(&user).await
    }
    // 加盐加密
    async fn hash_password(&self, password: &str) -> Result<String, String> {
        let salt = SaltString::generate(&mut OsRng);
        Argon2::default()
           .hash_password(password.as_bytes(), &salt)
           .map_err(|e| AppError::RegisterError("Failed to hash password".to_owned()))
           .map(|hash| hash.to_string())
    }
}