use domain::service::auth_service::AuthService;

pub struct LogoutUserUseCase<A: AuthService> {
    auth_service: A,
}

impl<A: AuthService> LogoutUserUseCase<A> {
    pub fn new(auth_service: A) -> Self {
        LogoutUserUseCase { auth_service }
    }

    pub async fn logout(&self) -> Result<(), String> {
        self.auth_service.logout_user().await
    }
}