use common::error::InfraError;

use crate::{model::user::User, repositories::query_user_repository::QueryUserRepository};

pub struct QueryUserService<Q>
where
    Q: QueryUserRepository,
{
    repository: Q,
}

impl<Q> QueryUserService<Q>
where
    Q: QueryUserRepository,
{
    pub fn new(repository: Q) -> Self {
        Self { repository }
    }

    pub async fn find_all_users(&self) -> Result<Vec<User>, InfraError> {
        self.repository.find_all().await
    }

    pub async fn find_user_by_id(&self, id: i32) -> Result<Option<User>, InfraError> {
        self.repository.find_by_id(id).await
    }
}
