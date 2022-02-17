use orderbook::order::{Order, OrderType};
use orderbook::order_data::OrderData;

#[test]
fn order_data() {
    let data = OrderData::new(1, "quang".to_string(), 10);
    assert_eq!(data.order_id, 1);
    assert_eq!(data.user_name, "quang".to_string());
    assert_eq!(data.security_id, 10);
}

#[test]
fn new_order() {
    let data = OrderData {
        ..Default::default()
    };
    let order = OrderType::NewOrder { data, price: 1, quantity: 1, is_buy_side: false };
    order.process().unwrap();
}

#[test]
fn modify_order() {
    let order = OrderType::ModifyOrder {
        order_id: 1,
        new_price: 100,
        new_quantity: 1000,
        is_buy_side: true,
    };
    order.process().unwrap();
}

#[test]
fn cancel_order() {
    let order_id = 1;
    let order = OrderType::CancelOrder { order_id };
    order.process().unwrap();
}
