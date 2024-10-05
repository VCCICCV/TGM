use domain::service::auth_service::AuthService;

pub struct LoginUserUseCase<A: AuthService> {
    auth_service: A,
}

impl<A: AuthService> LoginUserUseCase<A> {
    pub fn new(auth_service: A) -> Self {
        LoginUserUseCase { auth_service }
    }

    pub async fn login(&self, user_schema: &LoginUserSchema) -> Result<(String, String), String> {
        self.auth_service.login_user(user_schema).await
    }
}