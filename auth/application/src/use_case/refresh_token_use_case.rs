use domain::service::auth_service::AuthService;

pub struct RefreshTokenUseCase<A: AuthService> {
    auth_service: A,
}

impl<A: AuthService> RefreshTokenUseCase<A> {
    pub fn new(auth_service: A) -> Self {
        RefreshTokenUseCase { auth_service }
    }

    pub async fn refresh(&self, refresh_token: &str) -> Result<(String, String), String> {
        self.auth_service.refresh_token(refresh_token).await
    }
}