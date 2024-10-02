use common::error::InfraError;
use crate::model::user::User;
#[allow(async_fn_in_trait)]
pub trait CommandUserRepository {
    async fn create(&self, user: User) -> Result<User, InfraError>;
    async fn update(&self,user: User) -> Result<User, InfraError>;
    async fn delete(&self, id: i32) -> Result<bool, InfraError>;
    // 其他命令方法
}
