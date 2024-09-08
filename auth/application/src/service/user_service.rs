use axum::extract::{ Path, Form };
use infrastructure::persistence::user_persistence::UserPersistence;
use tracing::info;
// use serde_json::json;
// use crate::common::res::ResJson;

use infrastructure::config::DB;
use common::res::Res;

pub struct UserService;

impl UserService {
    /// get_user_list 获取用户列表
    /// page_params 分页参数
    pub async fn get_users_list(
        Query(page_params): Query<PageParams>,
        Query(req): Query<SysUserSearchReq>
    ) -> Res<ListData<UserWithDept>> {
        info!("get_users_list");
        let db = DB.get_or_init(get_db_connection).await;
        let res = system::sys_user::get_users_list(db, page_params, req).await;
        match res {
            Ok(x) => Res::with_data(x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }

    // pub async fn create_user(form: Form<(String, String)>) -> Result<(), String> {
    //     let (username, email) = form.0;
    //     info!("User created successfully");
    //     println!("User created successfully");
    //     match UserPersistence::create_user(username.as_str(), email.as_str()).await {
    //         Ok(_) => Ok(()),

    //         Err(e) => Err(format!("Failed to create user: {}", e)),
    //     }
    // }

    // pub async fn get_user_by_id(id: Path<i32>) -> Result<Option<String>, String> {
    //     let user_id = *id;
    //     match UserPersistence::get_user_by_id(user_id).await {
    //         Ok(user) => Ok(user.map(|u| format!("User: id={}, username={}, email={}", u.id, u.username, u.email))),
    //         Err(e) => Err(format!("Failed to get user: {}", e)),
    //     }
    // }
    pub async fn get_user_by_id(id: Path<i32>) -> Result<Option<String>, String> {
        let user_id = *id;

        match UserPersistence::get_user_by_id(user_id).await {
            Ok(user) =>
                Ok(
                    user.map(|u|
                        format!("User: id={}, username={}, email={}", u.id, u.username, u.email)
                    )
                ),
            Err(e) => Err(format!("Failed to get user: {}", e)),
        }
        // info!("user_id:{}", user_id);
        // Err(format!("Failed to get user"))
    }

    pub async fn update_user(id: Path<i32>, form: Form<(String, String)>) -> Result<(), String> {
        let user_id = *id;
        let (username, email) = form.0;
        match UserPersistence::update_user(user_id, username.as_str(), email.as_str()).await {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to update user: {}", e)),
        }
    }

    pub async fn delete_user(id: Path<i32>) -> Result<(), String> {
        let user_id = *id;
        match UserPersistence::delete_user(user_id).await {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to delete user: {}", e)),
        }
    }
}
