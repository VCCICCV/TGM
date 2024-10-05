use domain::service::auth_service::AuthService;

pub struct GetInfoUseCase<A: AuthService> {
    auth_service: A,
}

impl<A: AuthService> GetInfoUseCase<A> {
    pub fn new(auth_service: A) -> Self {
        GetMeUseCase { auth_service }
    }

    pub async fn get_info(&self, token: &str) -> Result<FilteredUser, String> {
        let user = self.auth_service.get_user_by_token(token).await?;
        let user = user.ok_or_else(|| "User not found".to_string())?;
        Ok(self.filter_user(&user))
    }

    fn filter_user(&self, user: &User) -> FilteredUser {
        FilteredUser {
            id: user.id.to_string(),
            email: user.email.to_owned(),
            name: user.name.to_owned(),
            photo: user.photo.to_owned(),
            role: user.role.to_owned(),
            verified: user.verified,
            createdAt: user.created_at.unwrap(),
            updatedAt: user.updated_at.unwrap(),
        }
    }
}