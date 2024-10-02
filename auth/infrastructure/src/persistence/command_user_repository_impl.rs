use crate::database::db_connection::establish_db_connection;
use crate::entities::prelude::User as UserEntity;
use crate::entities::user::ActiveModel;
use crate::entities::user::Entity;
use common::error::InfraError;
use domain::model::user::User;
use domain::repositories::command_user_repository::CommandUserRepository;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::Set;
pub struct CommandUserRepositoryImpl {}

impl CommandUserRepository for CommandUserRepositoryImpl {
    async fn create(&self, user: User) -> Result<User, InfraError> {
        let db = establish_db_connection().await.map_err(InfraError::from)?;
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
        new_user.ok_or(InfraError::UserNotFound)
    }

    async fn update(&self, user: User) -> Result<User, InfraError> {
        let db = establish_db_connection().await.map_err(InfraError::from)?;

        //通过emial查
        let existing_user = UserEntity::find()
            .filter(<Entity as EntityTrait>::Column::Email.eq(user.email.clone()))
            .one(&db)
            .await
            .map_err(InfraError::from)?;
        // 存在则更新
        if let Some(existing_user) = existing_user {
            let active_model = ActiveModel {
                id: Set(existing_user.id),
                username: Set(user.username),
                email: Set(user.email),
                ..Default::default()
            };
            // 执行更新
            let update_result = UserEntity::update_many()
                .filter(<Entity as EntityTrait>::Column::Email.eq(existing_user.email))
                .set(active_model)
                .exec(&db)
                .await
                .map_err(InfraError::from)?;
            // 如果更新的行数为0，则表示用户不存在
            if update_result.rows_affected == 0 {
                return Err(InfraError::UserNotFound);
            }
            // 重新查询更新后的记录
            let model = UserEntity::find_by_id(existing_user.id)
                .one(&db)
                .await
                .map_err(InfraError::from)?;
            // 将查询结果转换为User类型
            model
                .map(|model| User {
                    id: Some(model.id),
                    username: model.username,
                    email: model.email,
                })
                .ok_or_else(|| InfraError::UserNotFound)
        } else {
            Err(InfraError::UserNotFound)
        }
    }

    async fn delete(&self, id: i32) -> Result<bool, InfraError> {
        let db = establish_db_connection().await?;
        let delete_result = UserEntity::delete_many()
            .filter(<Entity as EntityTrait>::Column::Id.eq(id))
            .exec(&db)
            .await?;
        Ok(delete_result.rows_affected > 0)
    }
}
