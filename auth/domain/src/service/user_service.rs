use common::error::InfraError;
use domain::model::user::User;
// use domain::repositories::command_user_repository::CommandUserRepository;
// use domain::repositories::query_user_repository::QueryUserRepository;
use domain::repositories::user_repository::UserRepository;
use infrastructure::persistence::user_repository_impl::UserRepositoryImpl;

// use crate::event_bus::user_created_event::EventBus;
/// 如果使用anyhow的话，需要使用`anyhow::Error`类型，而不是`InfraError`类型
/// ```
///  use anyhow::Error;
/// ```
/// 定义一个UserRepository trait，用于处理用户相关的数据库操作
/// 封装infrastructure层的具体实现
pub struct UserService {
    repository: UserRepositoryImpl,
}

impl UserService {
    /// IOC
    pub fn new() -> Self {
        Self {
            repository: UserRepositoryImpl {},
        }
    }

    pub async fn find_all_users(&self) -> Result<Vec<User>, InfraError> {
        self.repository.find_all().await
    }

    pub async fn find_user_by_id(&self, id: i32) -> Result<Option<User>, InfraError> {
        self.repository.find_by_id(id).await
    }

    pub async fn create_user(&self, user: User) -> Result<User, InfraError> {
        self.repository.create(user).await
    }

    pub async fn update_user(&self, user: User) -> Result<User, InfraError> {
        self.repository.update(user).await
    }

    pub async fn delete_user(&self, id: i32) -> Result<bool, InfraError> {
        self.repository.delete(id).await
    }
}