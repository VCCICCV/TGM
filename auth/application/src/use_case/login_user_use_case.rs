// use domain::{ model::dto::user_dto::LoginUserSchema, service::auth_service::AuthService };

// pub struct LoginUserUseCase<A: AuthService> {
//     auth_service: A,
// }

// impl<A: AuthService> LoginUserUseCase<A> {
//     pub fn new(auth_service: A) -> Self {
//         LoginUserUseCase { auth_service }
//     }

//     pub async fn login(&self, user_schema: &LoginUserSchema) -> Result<(String, String), String> {
//         self.auth_service.login(user_schema).await
//         //     // 先查询是否存在
//         //     match self.user_repository.user_exists(&user_schema.email).await {
//         //         Ok(is_exist) => {
//         //             if is_exist {
//         //                 return Err(AppError::RegisterError("User already exists".to_owned()));
//         //             }
//         //         }
//         //         Err(e) => return Err(AppError::RegisterError(e.to_string())),
//         //     };
//         //     // 计算密码
//         //     let hashed_password = self.hash_password(&user_schema.password).await?;
//         //     let user = self.user_repository.find_by_email(&user_schema.email).await?;
//         //     if hashed_password!= user.password {
//         //         return Err(AppError::RegisterError("Invalid password".to_owned()));
//         //     }
//         //     let access_token = self.user_repository.generate_jwt(user).await?;
//         //     let refresh_token = self.user_repository.generate_refresh_jwt(user).await?;
//         // }
//         // // 加盐加密
//         // async fn hash_password(&self, password: &str) -> Result<String, AppError> {
//         //     let salt = SaltString::generate(&mut OsRng);
//         //     let argon2 = Argon2::default();
//         //     argon2
//         //         .hash_password(password.as_bytes(), &salt)
//         //         .map_err(|e| match e {
//         //             _ => AppError::RegisterError("Failed to hash password".to_owned()),
//         //         })
//         //         .map(|hash| hash.to_string())
//         // }
//     }
// }
