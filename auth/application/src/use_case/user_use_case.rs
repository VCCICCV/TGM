use common::error::InfraError;
use domain::{
    model::user::User,
    service::{command_user_service::CommandUserService, query_user_service::QueryUserService},
};
use infrastructure::persistence::{
    command_user_repository_impl::CommandUserRepositoryImpl,
    query_user_repository_impl::QueryUserRepositoryImpl,
};

pub struct UserUseCase {
    query_service: QueryUserService<QueryUserRepositoryImpl>,
    command_service: CommandUserService<CommandUserRepositoryImpl>,
}
impl UserUseCase {
    pub fn new() -> Self {
        // 查询
        let query_repository = QueryUserRepositoryImpl {};
        let query_service = QueryUserService::new(query_repository);
        // 命令
        let command_repository = CommandUserRepositoryImpl {};
        let command_service: CommandUserService<CommandUserRepositoryImpl> =
            CommandUserService::new(command_repository);
        Self {
            query_service,
            command_service,
        }
    }
    pub async fn list_users(&self) -> Result<Vec<User>, InfraError> {
        // 这里使用领域服务来获取用户列表
        self.query_service.find_all_users().await
    }

    pub async fn get_user_by_id(&self, id: i32) -> Result<Option<User>, InfraError> {
        self.query_service.find_user_by_id(id).await
    }

    pub async fn create_user(&self, user: User) -> Result<User, InfraError> {
        self.command_service.create_user(user).await
    }

    pub async fn update_user(&self, user: User) -> Result<User, InfraError> {
        self.command_service.update_user(user).await
    }

    pub async fn delete_user(&self, id: i32) -> Result<bool, InfraError> {
        self.command_service.delete_user(id).await
    }
}
