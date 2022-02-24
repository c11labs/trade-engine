use orderbook::order::Order;
use orderbook::order_side::OrderSide;
use orderbook::price_level::PriceLevel;

#[test]
fn price_level_test() {
    let price = 2.0;
    let order_id = 1;
    let mut limit = PriceLevel::new(price);
    let order = Order::new(order_id, 1, "quang".to_string(), price, 100, OrderSide::Bid);

    // add new order
    limit.add(order).unwrap();
    // get order
    let (_index, found): (usize, &Order) = limit.get_order(order_id).unwrap();
    assert_eq!(found.order_id, order_id);
    assert_eq!(found.user_id, 1);
    assert_eq!(found.instrument_id, "quang".to_string());
    assert_eq!(found.price, price);
    assert_eq!(found.quantity, 100);
    assert_eq!(found.side, OrderSide::Bid);

    assert_eq!(limit.size(), 1);
    assert_eq!(limit.volume(), 100);

    // modify order
    let modified_order = Order::new(order_id, 1, "quang".to_string(), 3.0, 1000, OrderSide::Bid);
    limit.modify(order_id, modified_order).unwrap();
    // get modified order
    let (_index, found): (usize, &Order) = limit.get_order(order_id).unwrap();
    assert_eq!(found.price, 3.0);
    assert_eq!(found.quantity, 1000);
    assert_eq!(limit.size(), 1);
    assert_eq!(limit.volume(), 1000);

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
