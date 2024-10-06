pub trait OrderRepository {
    async fn save_order(&self, order: Order) -> Result<(), String>;
    async fn find_order_by_id(&self, id: i32) -> Result<Option<Order>, String>;
    async fn find_orders_by_user_id(&self, user_id: i32) -> Result<Vec<Order>, String>;
}