use crate::database::connection::establish_connection;
use crate::entities::user::Entity;
use crate::entities::prelude::User as UserEntity;
use crate::entities::user::ActiveModel;
use common::error::InfraError;
use domain::model::user::User;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::Set;
pub struct UserRepositoryImpl {}

impl UserRepository for UserRepositoryImpl {
    async fn find_all(&self) -> Result<Vec<User>, InfraError> {
        let db = establish_connection().await?;
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
        let db = establish_connection().await?;
        let model = UserEntity::find_by_id(id).one(&db).await?;
        let user = model.map(|model| User {
            id: Some(model.id),
            username: model.username,
            email: model.email,
        });
        Ok(user)
    }
    async fn create(&self, user: User) -> Result<User, InfraError> {
        let db = establish_connection().await.map_err(InfraError::from)?;
        let active_model = ActiveModel {
            username: Set(user.username),
            email: Set(user.email),
            ..Default::default()
        };

        let insert_result = UserEntity::insert(active_model)
            .exec(&db)
            .await
            .map_err(InfraError::from)?;

        // Re-query the newly inserted record
        let model = UserEntity::find_by_id(insert_result.last_insert_id)
            .one(&db)
            .await
            .map_err(InfraError::from)?;
        let new_user = model.map(|model| User {
            id: Some(model.id),
            username: model.username,
            email: model.email,
        });
        // 如果你想用anyhow的话，需要使用`anyhow::Error`类型，而不是`InfraError`类型
        // # Example
        // ```
        // new_user.ok_or_else(|| anyhow!("Failed to retrieve new user"))
        // ```
        new_user.ok_or(InfraError::UserNotFound)
    }
    async fn update(&self, user: User) -> Result<User, InfraError> {
        let db = establish_connection().await?;
        let active_model = ActiveModel {
            username: Set(user.username),
            email: Set(user.email),
            ..Default::default()
        };
        let update_result = UserEntity::update_many()
            .filter(<Entity as EntityTrait>::Column::Id.eq(user.id.unwrap()))
            .set(active_model)
            .exec(&db)
            .await?;
        if update_result.rows_affected == 0 {
            return Err(InfraError::UserNotFound);
        }
        // 重新查询更新后的记录
        let model = UserEntity::find_by_id(user.id.unwrap()).one(&db).await?;
        let updated_user_result = model.map(|model| User {
            id: Some(model.id),
            username: model.username,
            email: model.email,
        });
        updated_user_result.ok_or_else(|| InfraError::UserNotFound)
    }
    async fn delete(&self, id: i32) -> Result<bool, InfraError> {
        let db = establish_connection().await?;
        let delete_result = UserEntity::delete_many()
            .filter(<Entity as EntityTrait>::Column::Id.eq(id))
            .exec(&db)
            .await?;
        Ok(delete_result.rows_affected > 0)
    }
}
