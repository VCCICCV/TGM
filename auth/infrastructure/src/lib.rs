pub mod config{
    pub mod db_config;
}
pub mod persistence{
    pub mod user_persistence;
}
/// 代表数据库中的表结构
pub mod entities{
    pub mod user;
    pub mod order;
    pub mod product; 
}
#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
