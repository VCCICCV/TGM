use common::error::{AppError, InfraError};

use crate::model::dto::user_dto::LoginUserDTO;

pub trait AuthRepository {
    async fn generate_jwt(&self, user_dto: LoginUserDTO) -> Result<String, InfraError>;
    async fn generate_password(&self, password: &str) -> Result<String, InfraError>;
    async fn verify_password(&self, password: &str, hashed_password: &str) -> Result<bool, InfraError>;
    async fn verify_token(&self, token: &str) -> Result<bool, InfraError>;
}