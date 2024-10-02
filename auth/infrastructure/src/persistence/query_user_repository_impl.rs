use crate::database::db_connection::establish_db_connection;
use crate::entities::prelude::User as UserEntity;
use common::error::InfraError;
use domain::model::user::User;
use domain::repositories::query_user_repository::QueryUserRepository;
use sea_orm::EntityTrait;
pub struct QueryUserRepositoryImpl {}

impl QueryUserRepository for QueryUserRepositoryImpl {
    // async fn find_all(&self) -> Result<Vec<User>, InfraError> {
    //     let db = establish_db_connection().await?;
    //     let models = UserEntity::find().all(&db).await?;
    //     let users: Vec<User> = models
    //         .into_iter()
    //         .map(|model| User {
    //             id: Some(model.id),
    //             username: model.username,
    //             email: model.email,
    //         })
    //         .collect();
    //     Ok(users)
    // }
    async fn find_all(&self) -> Result<Vec<User>, InfraError> {

        // 建立数据库连接
        let db = establish_db_connection().await?;
        let models = UserEntity::find().all(&db).await?;
        let users: Vec<User> = models
            .into_iter()
            .map(|model| User {
                id: Some(model.id),
                username: model.username,
                email: model.email,
            })
            .collect();
        Ok(users)
    }
    async fn find_by_id(&self, id: i32) -> Result<Option<User>, InfraError> {
        let db = establish_db_connection().await?;
        let model = UserEntity::find_by_id(id).one(&db).await?;
        let user = model.map(|model| User {
            id: Some(model.id),
            username: model.username,
            email: model.email,
        });
        Ok(user)
    }
}
