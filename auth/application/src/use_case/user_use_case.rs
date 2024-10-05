use common::error::InfraError;
use domain::{model::user::User, service::user_service::UserService};
use infrastructure::persistence::user_repository_impl::UserRepositoryImpl;

pub struct UserUseCase {
    user_service: UserService<UserRepositoryImpl>,
}
impl UserUseCase {
    pub fn new() -> Self {
        Self {
            user_service: UserService::new(UserRepositoryImpl {}),
        }
    }
    pub async fn list_users(&self) -> Result<Vec<User>, InfraError> {
        // 这里使用领域服务来获取用户列表
        self.user_service.find_all_users().await
    }

    pub async fn get_user_by_id(&self, id: i32) -> Result<Option<User>, InfraError> {
        self.user_service.find_user_by_id(id).await
    }

    pub async fn create_user(&self, user: User) -> Result<bool, InfraError> {
        self.user_service.create_user(user).await
    }

    pub async fn update_user(&self, user: User) -> Result<bool, InfraError> {
        self.user_service.update_user(user).await
    }

    pub async fn delete_user(&self, id: i32) -> Result<bool, InfraError> {
        self.user_service.delete_user(id).await
    }
    // pub async fn register(&self, user: User) -> Result<String, InfraError> {
    //     // 先创建用户（假设create_user方法在userUserService中定义且返回Result<(), InfraError>）
    //     self.user_service.create_user(user.clone()).await?;
    //     // 再生成JWT
    //     self.user_service.generate_jwt(user).await
    // }
    // pub async fn login(&self, user: User) -> Result<String, InfraError> {
    //     self.user_service
    //         .find_user_by_email(user.clone().email)
    //         .await?;
    //     self.user_service.generate_jwt(user).await
    // }
    // pub async fn authenticate_jwt(&self, user: User) -> Result<User, InfraError> {
    //     let claims = self.user_service.authenticate(user).await;
    //     Ok(claims)=>{
    //         if claims.exp<(chrono::Utc::now().timestamp() as usize) {
    //             return Err(InfraError::TokenExpired)
    //         }
    //         Ok(true)
    //     }
    //     Err(_)=>Ok(flase)
    // }

}
