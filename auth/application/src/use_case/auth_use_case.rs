use common::error::AppError;
use domain::{
    model::{ dto::user_dto::LoginUserDTO, user::User },
    repositories::user_repository::UserRepository,
    service::user_service::UserService,
};

pub struct LoginUseCase<U: UserRepository> {
    user_service: UserService<U>,
}

impl<U: UserRepository> LoginUseCase<U> {
    pub fn new(user_service: UserService<U>) -> Self {
        LoginUseCase { user_service }
    }

    pub async fn execute(&self, login_user_dto: &LoginUserDTO) -> Result<(User, String), AppError> {
        self.user_service.login(login_user_dto).await
    }
}
pub struct LogoutUseCase<U: UserRepository> {
    user_service: UserService<U>,
}

impl<U: UserRepository> LogoutUseCase<U> {
    pub fn new(user_service: UserService<U>) -> Self {
        LogoutUseCase { user_service }
    }

    pub async fn execute(&self, user: &User) -> Result<(), DbErr> {
        self.user_service.logout(user).await
    }
}
pub struct ForgotPasswordUseCase<U: UserRepository> {
    user_service: UserService<U>,
}

impl<U: UserRepository> ForgotPasswordUseCase<U> {
    pub fn new(user_service: UserService<U>) -> Self {
        ForgotPasswordUseCase { user_service }
    }

    pub async fn execute(&self, forgot_password_request_dto: &ForgotPasswordRequestDto) -> Result<(), DbErr> {
        self.user_service.forgot_password(forgot_password_request_dto).await
    }
}
pub struct ResetPasswordUseCase<U: UserRepository> {
    user_service: UserService<U>,
}

impl<U: UserRepository> ResetPasswordUseCase<U> {
    pub fn new(user_service: UserService<U>) -> Self {
        ResetPasswordUseCase { user_service }
    }

    pub async fn execute(&self, reset_password_request_dto: &ResetPasswordRequestDto, user: &User) -> Result<(), DbErr> {
        self.user_service.reset_password(reset_password_request_dto, user).await
    }
}