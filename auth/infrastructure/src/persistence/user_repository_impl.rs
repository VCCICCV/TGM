use crate::database::db_connection::establish_db_connection;
use crate::entities;
use crate::entities::prelude::User as UserEntity;
use crate::entities::user::ActiveModel;
use crate::entities::user::Entity;
use crate::middleware::jwt::encode_jwt;
use crate::utils::redis_util::RedisUtil;
use common::error::InfraError;
use domain::model::user::User;
use domain::repositories::user_repository::UserRepository;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::Set;
use tokio::task;
pub struct UserRepositoryImpl {}
impl UserRepository for UserRepositoryImpl {
    async fn find_all(&self) -> Result<Vec<User>, InfraError> {
        let redis_key = "all_users";
        let redis_util_result = RedisUtil::new().await.expect("InfraError::RedisError");
        let redis_util = redis_util_result;
        if let Ok(Some(cached)) = redis_util.get(redis_key).await {
            // 从缓存中获取到数据，反序列化
            let users: Vec<User> = serde_json::from_str(&cached).map_err(|e| {
                InfraError::RedisError(format!("Redis deserialization error: {}", e))
            })?;
            return Ok(users);
        }
        // 如果缓存中没有，则查询数据库
        let db = establish_db_connection().await?;
        let models = UserEntity::find().all(&db).await?;
        let users: Vec<User> = models
            .into_iter()
            .map(|model| User {
                id: Some(model.user_id.try_into().unwrap()),
                username: model.username,
                email: model.email,
                password: model.password,
                role: model.role,
                create_time: model.create_time,
            })
            .collect();
        // 将查询结果存储到Redis缓存中
        let serialized = serde_json::to_string(&users)
            .map_err(|e| InfraError::RedisError(format!("Redis serialization error: {}", e)))?;
        // 在后台异步存储到Redis缓存中，过期时间为60秒
        task::spawn(async move {
            redis_util
                .set_expire(redis_key, &serialized, 60)
                .await
                .expect("InfraError::RedisError");
        });
        Ok(users)
    }
    async fn find_by_id(&self, id: i32) -> Result<Option<User>, InfraError> {
        let db = establish_db_connection().await?;
        let model = UserEntity::find_by_id(id).one(&db).await?;
        let user = model.map(|model| User {
            id: Some(model.user_id),
            username: model.username,
            email: model.email,
            password: model.password,
            role: model.role,
            create_time: model.create_time,
        });
        Ok(user)
    }

    async fn find_by_email(&self, email: String) -> Result<Option<User>, InfraError> {
        let db = establish_db_connection().await?;
        let model = UserEntity::find()
            .filter(<entities::user::Entity as EntityTrait>::Column::Email.eq(email))
            .one(&db)
            .await?;
        let user = model.map(|model| User {
            id: Some(model.user_id),
            username: model.username,
            email: model.email,
            password: model.password,
            role: model.role,
            create_time: model.create_time,
        });
        Ok(user)
    }

    async fn create(&self, user: User) -> Result<bool, InfraError> {
        let db = establish_db_connection().await.map_err(InfraError::from)?;

        let active_model = ActiveModel {
            username: Set(user.username),
            email: Set(user.email),
            password: Set(user.password),
            role: Set(user.role),
            create_time: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        };

        let insert_result = UserEntity::insert(active_model)
            .exec(&db)
            .await
            .map_err(|e| InfraError::InsertError(format!("fail: {}", e)));

        match insert_result {
            Ok(_) => Ok(true),
            Err(e) => Err(InfraError::InsertError(format!("fail: {}", e))),
        }
    }

    async fn update(&self, user: User) -> Result<bool, InfraError> {
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
                user_id: Set(existing_user.user_id),
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
            Ok(true)
        } else {
            // 如果用户不存在，则返回错误
            Err(InfraError::UserNotFound)
        }
        //     // 重新查询更新后的记录
        //     let model = UserEntity::find_by_id(existing_user.user_id)
        //         .one(&db)
        //         .await
        //         .map_err(InfraError::from)?;
        //     // 将查询结果转换为User类型
        //     model
        //         .map(|model| User {
        //             id: Some(model.user_id),
        //             username: model.username,
        //             email: model.email,
        //             password: model.password,
        //             role: model.role,
        //             create_time: model.create_time,
        //         })
        //         .ok_or_else(|| InfraError::UserNotFound)
        // } else {
        //     Err(InfraError::UserNotFound)
        // }
    }

    async fn delete(&self, id: i32) -> Result<bool, InfraError> {
        let db = establish_db_connection().await?;
        let delete_result = UserEntity::delete_many()
            .filter(<Entity as EntityTrait>::Column::UserId.eq(id))
            .exec(&db)
            .await?;
        Ok(delete_result.rows_affected > 0)
    }
    async fn generate_jwt(&self, user: User) -> Result<String, InfraError> {
        let token = encode_jwt(user).await;
        token
    }

    async fn user_exists(&self, _email: &str) -> Result<bool, String> {
        todo!()
    }
}
