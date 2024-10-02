use common::error::InfraError;

use crate::{model::user::User, repositories::command_user_repository::CommandUserRepository};

pub struct CommandUserService<C>
where
    C: CommandUserRepository,
{
    repository: C,
}

impl<C> CommandUserService<C>
where
    C: CommandUserRepository,
{
    pub fn new(repository: C) -> Self {
        Self { repository }
    }
    pub async fn create_user(&self, user: User) -> Result<User, InfraError> {
        // 这里调用的是仓库层的trait方法command_user_repository
        self.repository.create(user).await
    }

    pub async fn update_user(&self, user: User) -> Result<User, InfraError> {
        self.repository.update(user).await
    }

    pub async fn delete_user(&self, id: i32) -> Result<bool, InfraError> {
        self.repository.delete(id).await
    }
}
