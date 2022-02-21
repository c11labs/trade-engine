use orderbook::order::Order;
use orderbook::order_side::OrderSide;

#[test]
fn order_test() {
    let _order = Order::new(1, "test".to_string(), 1, 1.0, 100, OrderSide::Ask);
}
