pub struct PlaceOrderUseCase<O: OrderRepository, S: OrderService> {
    order_repository: O,
    order_service: S,
}

impl<O: OrderRepository, S: OrderService> PlaceOrderUseCase<O, S> {
    pub fn new(order_repository: O, order_service: S) -> Self {
        PlaceOrderUseCase {
            order_repository,
            order_service,
        }
    }

    pub async fn place_order(&self, order_dto: &PlaceOrderDto) -> Result<(), String> {
        self.order_service.place_order(order_dto).await
    }
}