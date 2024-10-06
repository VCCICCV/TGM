// use argon2::{
//     password_hash::{
//         rand_core::OsRng, PasswordHasher, SaltString
//     },
//     Argon2
// };
// use common::error::AppError;
// use domain::{
//     model::{dto::user_dto::RegisterUserSchema, user::User},
//     service::auth_service::AuthService,
// };

// pub struct RegisterUserUseCase<A: AuthService> {
//     auth_service: A,
// }
// impl<U: UserRepository, A: AuthService> RegisterUserUseCase<U, A> {
//     pub fn new(user_repository: U, auth_service: A) -> Self {
//         RegisterUserUseCase {
//             auth_service,
//         }
//     }
//     // 注册用户
//     pub async fn register(&self, user_schema: &RegisterUserSchema) -> Result<String, AppError> {
//         self.auth_service.register_user(user_schema).await
//         // // 先查询是否存在
//         // match self.user_repository.user_exists(&user_schema.email).await {
//         //     Ok(is_exist) => if is_exist{
//         //         return Err(AppError::RegisterError("User already exists".to_owned()));
//         //     },
//         //     Err(e) => return Err(AppError::RegisterError(e.to_string())),
//         // };
//         // // 计算密码
//         // let hashed_password = infrastructure::middleware::password::hash_password(&user_schema.password).await?;
//         // // 创建用户
//         // let user = User {
//         //     id: None,
//         //     username: user_schema.username.clone(),
//         //     email: user_schema.email.clone(),
//         //     password: hashed_password,
//         //     role: 0, // 根据实际情况设置默认角色
//         //     create_time: chrono::Utc::now().naive_utc(),
//         // };
//         // let _ = self.auth_service.register_user(&user).await;
//         // // 生成jwt
//         // let jwt_token = self
//         //     .user_repository
//         //     .generate_jwt(user)
//         //     .await
//         //     .map_err(|e| AppError::RegisterError(e.to_string()))?;
//         // Ok(jwt_token)
//     }
// }
