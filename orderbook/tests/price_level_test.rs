use orderbook::order::Order;
use orderbook::order_side::OrderSide;
use orderbook::order_type::OrderType;
use orderbook::price_level::PriceLevel;

#[test]
fn price_level_test() {
    let price = 2.0;
    let order_id = 1;
    let mut limit = PriceLevel::new(price);
    let order = Order::new(
        order_id,
        1,
        0.0,
        "GOOGLE".to_string(),
        Some(price),
        100.0,
        OrderSide::Bid,
        OrderType::Limit,
    );

    // add new order
    limit.add(order).unwrap();
    // get order
    let (_index, found): (usize, &Order) = limit.get_order(order_id).unwrap();
    assert_eq!(found.order_id, order_id);
    assert_eq!(found.user_id, 1);
    assert_eq!(found.pair, "GOOGLE".to_string());
    assert_eq!(found.price.unwrap(), price);
    assert_eq!(found.amount, 100.0);
    assert_eq!(found.side, OrderSide::Bid);

    assert_eq!(limit.num_order(), 1);
    assert_eq!(limit.size(), 100.0);

    /* let modified_order = Order::new(
        order_id,
        1,
        "GOOGLE".to_string(),
        Some(3.0),
        1000.0,
        OrderSide::Bid,
        OrderType::Limit,
    );
    limit.modify(order_id, modified_order).unwrap();
    // get modified order
    let (_index, found): (usize, &Order) = limit.get_order(order_id).unwrap();
    assert_eq!(found.price.unwrap(), 3.0);
    assert_eq!(found.quantity, 1000.0);
    assert_eq!(limit.size(), 1);
    assert_eq!(limit.volume(), 1000.0); */

    // remove order
    let _removed = match limit.remove(order_id) {
        Ok(_order) => {
            println!("removed an order");
            assert_eq!(limit.num_order(), 0);
            assert_eq!(limit.size(), 0.0);
        }
        Err(err) => println!("{err}"),
    };

    // get removed order
    if let Err(err) = limit.get_order(order_id) {
        println!("{err}");
    }
}
