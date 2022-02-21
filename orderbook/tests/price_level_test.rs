use orderbook::order::Order;
use orderbook::order_side::OrderSide;
use orderbook::price_level::PriceLevel;

#[test]
fn price_level_test() {
    let price = 2.0;
    let order_id = 1;
    let mut limit = PriceLevel::new(price);
    let order = Order::new(order_id, "quang".to_string(), 1, price, 100, OrderSide::Bid).unwrap();

    // add new order
    limit.add(&order).unwrap();
    // get order
    let (_index, found): (usize, &Order) = limit.get_order(order_id).unwrap();
    assert_eq!(found.order_id, order.order_id);
    assert_eq!(found.user_name, order.user_name);
    assert_eq!(found.security_id, order.security_id);
    assert_eq!(found.price, order.price);
    assert_eq!(found.quantity, order.quantity);
    assert_eq!(found.side, order.side);

    // modify order
    let modified_order = Order::new(
        order_id,
        order.user_name.clone(),
        order.security_id,
        3.0,
        1000,
        OrderSide::Bid,
    )
    .unwrap();
    limit.modify(order_id, &modified_order).unwrap();
    // get modified order
    let (_index, found): (usize, &Order) = limit.get_order(order_id).unwrap();
    assert_eq!(found.price, modified_order.price);
    assert_eq!(found.quantity, modified_order.quantity);

    // remove order
    let _removed = match limit.remove(order_id) {
        Ok(_order) => {
            println!("removed an order");
            assert_eq!(limit.size(), 0);
            assert_eq!(limit.volume(), 0);
        }
        Err(err) => println!("{err}"),
    };

    // get removed order
    if let Err(err) = limit.get_order(order_id) {
        println!("{err}");
    }
}
