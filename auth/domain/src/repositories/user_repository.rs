use crate::model::user::User;
use common::error::InfraError;
pub trait UserRepository {
    async fn find_by_id(&self, id: i32) -> Result<Option<User>, InfraError>;
    async fn find_all(&self) -> Result<Vec<User>, InfraError>;
    async fn find_by_email(&self, email: String) -> Result<Option<User>, InfraError>;
    async fn user_exists(&self, email: &str) -> Result<bool, String>;
    async fn create(&self, user: User) -> Result<bool, InfraError>;
    async fn update(&self, user: User) -> Result<bool, InfraError>;
    async fn delete(&self, id: i32) -> Result<bool, InfraError>;
    async fn generate_jwt(&self, user: User) -> Result<String, InfraError>;
}
