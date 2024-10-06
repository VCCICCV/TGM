pub struct OrderRepositoryImpl {
    pool: PgPool,
}

impl OrderRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        OrderRepositoryImpl { pool }
    }
}
impl OrderService for OrderServiceImpl {}