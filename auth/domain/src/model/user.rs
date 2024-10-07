use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub user_id: Option<i32>,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: i32,
    pub salt: String,
    pub is_deleted: bool,
    pub create_time: chrono::NaiveDateTime,
    // 可能是空的
    pub update_time: Option<chrono::NaiveDateTime>,
}
