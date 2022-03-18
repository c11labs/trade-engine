#![allow(dead_code)]

use orderbook::order::Order;
use orderbook::order_side::OrderSide;
use orderbook::order_type::OrderType;
use orderbook::OrderBook;

#[test]
fn orderbook_test_error() {
    let mut orderbook: OrderBook = OrderBook::new(String::from("test"));
    assert_eq!(orderbook.pair(), "test");

    let order = Order::new(
        2,
        2,
        "test".to_string(),
        None,
        40.0,
        OrderSide::Ask,
        OrderType::Limit,
    );
    if let Err(_err) = orderbook.add(order) {
        // println!("{err:?}");
    };
}

#[test]
fn orderbook_test() {
    let mut orderbook: OrderBook = OrderBook::new(String::from("test"));
    assert_eq!(orderbook.pair(), "test");

    let bid_price_size: Vec<(f32, f32)> = vec![
        (100.0, 10.0),
        (100.0, 20.0),
        (100.0, 30.0),
        (99.0, 89.0),
        (98.0, 67.0),
        (97.0, 77.0),
        (96.0, 67.0),
        (95.0, 48.0),
        (94.0, 42.0),
        (93.0, 34.0),
        (92.0, 20.0),
        (91.0, 11.5),
    ];
    let mut id = 0;
    for (price, size) in &bid_price_size {
        id += 1;
        let order: Order = Order::new(
            id,
            id,
            "test".to_string(),
            Some(*price),
            *size,
            OrderSide::Bid,
            OrderType::Limit,
        );
        orderbook.add(order).unwrap();
    }

    let ask_price_size: Vec<(f32, f32)> = vec![
        (105.0, 103.0),
        (106.0, 77.0),
        (107.0, 89.0),
        (108.0, 73.0),
        (109.0, 70.0),
        (110.0, 41.0),
        (111.0, 40.0),
        (112.0, 32.0),
        (113.0, 23.0),
        (114.0, 9.0),
        (114.0, 90.0),
    ];
    let mut id = 0;
    for (price, size) in &ask_price_size {
        id += 1;
        let order: Order = Order::new(
            id,
            id,
            "test".to_string(),
            Some(*price),
            *size,
            OrderSide::Ask,
            OrderType::Limit,
        );
        orderbook.add(order).unwrap();
    }

    let order = Order::new(
        2,
        2,
        "test".to_string(),
        Some(107.0),
        40.0,
        OrderSide::Ask,
        OrderType::Limit,
    );
    orderbook.add(order).unwrap();

    let order = Order::new(
        2,
        2,
        "test".to_string(),
        None,
        120.0,
        OrderSide::Ask,
        OrderType::Market,
    );
    orderbook.add(order).unwrap();

    let order = Order::new(
        2,
        2,
        "test".to_string(),
        Some(100.0),
        80.0,
        OrderSide::Bid,
        OrderType::Limit,
    );
    orderbook.add(order).unwrap();

    let order = Order::new(
        2,
        2,
        "test".to_string(),
        Some(104.0),
        60.0,
        OrderSide::Ask,
        OrderType::Limit,
    );
    orderbook.add(order).unwrap();

    let order = Order::new(
        2,
        2,
        "test".to_string(),
        None,
        150.0,
        OrderSide::Bid,
        OrderType::Market,
    );
    if let Some(trade) = orderbook.add(order).unwrap() {
        println!("{trade:#?}");
    }

    let order = Order::new(
        2,
        2,
        "test".to_string(),
        Some(110.0),
        1000.0,
        OrderSide::Bid,
        OrderType::Limit,
    );
    if let Some(trade) = orderbook.add(order).unwrap() {
        println!("{trade:#?}");
    }
    /* bid_test(&orderbook);
    ask_test(&orderbook); */

    let price_and_size = orderbook.price_and_size();
    println!("{price_and_size:?}");

    assert_eq!(orderbook.size(110.0, OrderSide::Bid).unwrap(), 597.0);
    assert_eq!(orderbook.size(100.0, OrderSide::Bid).unwrap(), 80.0);
    assert_eq!(orderbook.size(99.0, OrderSide::Bid).unwrap(), 29.0);
    assert_eq!(orderbook.size(98.0, OrderSide::Bid).unwrap(), 67.0);
    assert_eq!(orderbook.size(97.0, OrderSide::Bid).unwrap(), 77.0);
    assert_eq!(orderbook.size(96.0, OrderSide::Bid).unwrap(), 67.0);
    assert_eq!(orderbook.size(95.0, OrderSide::Bid).unwrap(), 48.0);
    assert_eq!(orderbook.size(94.0, OrderSide::Bid).unwrap(), 42.0);
    assert_eq!(orderbook.size(93.0, OrderSide::Bid).unwrap(), 34.0);
    assert_eq!(orderbook.size(92.0, OrderSide::Bid).unwrap(), 20.0);
    assert_eq!(orderbook.size(91.0, OrderSide::Bid).unwrap(), 11.5);

    /* assert_eq!(orderbook.size(105.0, OrderSide::Ask).unwrap(), 13.0);
    assert_eq!(orderbook.size(106.0, OrderSide::Ask).unwrap(), 77.0);
    assert_eq!(orderbook.size(107.0, OrderSide::Ask).unwrap(), 129.0);
    assert_eq!(orderbook.size(108.0, OrderSide::Ask).unwrap(), 73.0);
    assert_eq!(orderbook.size(109.0, OrderSide::Ask).unwrap(), 70.0);
    assert_eq!(orderbook.size(110.0, OrderSide::Ask).unwrap(), 41.0); */
    assert_eq!(orderbook.size(111.0, OrderSide::Ask).unwrap(), 40.0);
    assert_eq!(orderbook.size(112.0, OrderSide::Ask).unwrap(), 32.0);
    assert_eq!(orderbook.size(113.0, OrderSide::Ask).unwrap(), 23.0);
    assert_eq!(orderbook.size(114.0, OrderSide::Ask).unwrap(), 99.0);

    /* Modify order */
    let order = Order::new(
        10,
        10,
        "test".to_string(),
        Some(114.0),
        0.0,
        OrderSide::Ask,
        OrderType::Limit,
    );
    if let Err(err) = orderbook.modify(order, Some(114.0), Some(1000.0)) {
        println!("{err:?}");
    }
    assert_eq!(orderbook.size(114.0, OrderSide::Ask).unwrap(), 1090.0);
    /* bid_test(&orderbook);
    ask_test(&orderbook); */

    /* Cancel order */
    let prices = vec![111.0, 112.0, 113.0, 114.0, 114.0];
    let mut index = 7;
    for price in prices {
        let order = Order::new(
            index,
            index,
            "test".to_string(),
            Some(price),
            0.0,
            OrderSide::Ask,
            OrderType::Limit,
        );
        orderbook
            .cancel(order.side, order.price.unwrap(), order.order_id)
            .unwrap();
        index += 1;
    }
    /* bid_test(&orderbook);
    ask_test(&orderbook); */
}

fn bid_test(orderbook: &OrderBook) {
    println!("----bid----");

    let price_list = orderbook.price_list(OrderSide::Bid);
    for price in &price_list {
        let volume = orderbook.size(*price, OrderSide::Bid).unwrap();
        let size = orderbook.num_order(*price, OrderSide::Bid).unwrap();
        println!("price {price} => size: {size}, volume: {volume}");
    }
    println!("bid price list: {price_list:?}");
}

fn ask_test(orderbook: &OrderBook) {
    println!("----ask----");

    let price_list = orderbook.price_list(OrderSide::Ask);
    for price in &price_list {
        let volume = orderbook.size(*price, OrderSide::Ask).unwrap();
        let size = orderbook.num_order(*price, OrderSide::Ask).unwrap();
        println!("price {price} => size: {size}, volume: {volume}");
    }
    println!("ask price list: {price_list:?}");
}
