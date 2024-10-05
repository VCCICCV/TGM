use crate::{model::user::User, repositories::user_repository::UserRepository};
use common::error::InfraError;

pub struct UserService<R>
where
    R: UserRepository,
{
    repository: R,
}

impl<R> UserService<R>
where
    R: UserRepository,
{
    /// IOC
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn find_all_users(&self) -> Result<Vec<User>, InfraError> {
        self.repository.find_all().await
    }

    pub async fn find_user_by_id(&self, id: i32) -> Result<Option<User>, InfraError> {
        self.repository.find_by_id(id).await
    }

    pub async fn create_user(&self, user: User) -> Result<bool, InfraError> {
        self.repository.create(user).await
    }

    pub async fn update_user(&self, user: User) -> Result<bool, InfraError> {
        self.repository.update(user).await
    }

    pub async fn delete_user(&self, id: i32) -> Result<bool, InfraError> {
        self.repository.delete(id).await
    }
}
