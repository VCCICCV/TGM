//! 应用层
/// 协调各层的具体业务流程，不应该实现具体的业务逻辑，只应该调用下层的接口
/// 例如订单服务需要调用用户服务，订单服务应该调用用户服务的接口，而不是直接调用用户服务的具体实现
/// 通过关联函数注入下层的接口，这样可以方便地替换下层的实现
/// # Example
/// ```rust
/// let user_service = UserService::new();
/// let order_service = OrderService::new(user_service, database_connection);
/// ```
/// usecase就是能做的事情，这里不做CQRS，主要是为了简单，如果系统足够复杂，可以考虑分离职责
/// 一个用例一般是一个业务流程
/// 例如创建订单，创建订单需要调用用户服务的接口->调用商品服务的接口->调用订单服务的接口
/// excute执行具体的domain能力
// 在这里注入依赖
pub mod use_case{
    pub mod login_user_use_case;
    pub mod register_user_use_case;
    pub mod get_info_use_case;
    pub mod logout_user_use_case;
    pub mod refresh_token_use_case;
    pub mod user_use_case;
}

