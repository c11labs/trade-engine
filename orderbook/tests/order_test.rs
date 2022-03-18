use orderbook::order::Order;
use orderbook::order_side::OrderSide;
use orderbook::order_type::OrderType;

#[test]
fn order_test() {
    let order = Order::new(
        1,
        1,
        String::from("GOOG"),
        Some(1.0),
        100.10,
        OrderSide::Ask,
        OrderType::Limit,
    );
    take_ownership(order);
}

fn take_ownership(order: Order) {
    println!("{order:?}");
}
