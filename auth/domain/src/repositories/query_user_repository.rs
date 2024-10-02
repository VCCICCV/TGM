use crate::model::user::User;
use common::error::InfraError;
#[allow(async_fn_in_trait)]
pub trait QueryUserRepository {
    async fn find_by_id(&self, id: i32) -> Result<Option<User>, InfraError>;
    async fn find_all(&self) -> Result<Vec<User>, InfraError>;
    // 添加其他查询相关方法
}
