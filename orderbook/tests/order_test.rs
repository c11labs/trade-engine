use orderbook::order::Order;
use orderbook::order_side::OrderSide;

#[test]
fn order_test() {
    let order = Order::new(1, 1, String::from("GOOG"), 1.0, 100, OrderSide::Ask);
    take_ownership(order);
}

fn take_ownership(order: Order) {
    println!("{order:?}");
}
