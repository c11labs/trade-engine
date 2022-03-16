#![allow(dead_code)]

use orderbook::order::Order;
use orderbook::order_side::OrderSide;
use orderbook::order_type::OrderType;
use orderbook::price::IntoInner;
use orderbook::OrderBook;

#[test]
fn orderbook_test() {
    let mut orderbook: OrderBook = OrderBook::new(10);
    assert_eq!(orderbook.instrument_id(), 10);

    let bid_price_size: Vec<(f32, u32)> = vec![
        (100.0, 100),
        (99.0, 89),
        (98.0, 67),
        (97.0, 77),
        (96.0, 67),
        (95.0, 48),
        (94.0, 42),
        (93.0, 34),
        (92.0, 20),
        (91.0, 11),
    ];
    for (price, size) in &bid_price_size {
        let order: Order = Order::new(
            1,
            1,
            "test".to_string(),
            Some(*price),
            *size,
            OrderSide::Bid,
            OrderType::Limit,
        );
        orderbook.bid(order).unwrap();
    }

    let ask_price_size: Vec<(f32, u32)> = vec![
        (105.0, 103),
        (106.0, 77),
        (107.0, 89),
        (108.0, 73),
        (109.0, 70),
        (110.0, 41),
        (111.0, 40),
        (112.0, 32),
        (113.0, 23),
        (114.0, 9),
    ];
    for (price, size) in &ask_price_size {
        let order: Order = Order::new(
            2,
            2,
            "test".to_string(),
            Some(*price),
            *size,
            OrderSide::Ask,
            OrderType::Limit,
        );
        orderbook.ask(order).unwrap();
    }
    /* bid_test(&orderbook);
    ask_test(&orderbook); */

    let order = Order::new(
        2,
        2,
        "test".to_string(),
        Some(107.0),
        40,
        OrderSide::Ask,
        OrderType::Limit,
    );
    orderbook.ask(order).unwrap();
    /* bid_test(&orderbook);
    ask_test(&orderbook); */

    let order = Order::new(
        2,
        2,
        "test".to_string(),
        None,
        120,
        OrderSide::Ask,
        OrderType::Market,
    );
    orderbook.ask(order).unwrap();
    /* bid_test(&orderbook);
    ask_test(&orderbook); */

    let order = Order::new(
        2,
        2,
        "test".to_string(),
        Some(100.0),
        80,
        OrderSide::Bid,
        OrderType::Limit,
    );
    orderbook.bid(order).unwrap();
    /* bid_test(&orderbook);
    ask_test(&orderbook); */

    let order = Order::new(
        2,
        2,
        "test".to_string(),
        Some(104.0),
        60,
        OrderSide::Ask,
        OrderType::Limit,
    );
    orderbook.ask(order).unwrap();
    /* bid_test(&orderbook);
    ask_test(&orderbook); */

    let order = Order::new(
        2,
        2,
        "test".to_string(),
        None,
        150,
        OrderSide::Bid,
        OrderType::Market,
    );
    orderbook.bid(order).unwrap();
    /* bid_test(&orderbook);
    ask_test(&orderbook); */

    assert_eq!(orderbook.bid_price_level_volume(100.0).unwrap(), 80);
    assert_eq!(orderbook.bid_price_level_volume(99.0).unwrap(), 69);
    assert_eq!(orderbook.bid_price_level_volume(98.0).unwrap(), 67);
    assert_eq!(orderbook.bid_price_level_volume(97.0).unwrap(), 77);
    assert_eq!(orderbook.bid_price_level_volume(96.0).unwrap(), 67);
    assert_eq!(orderbook.bid_price_level_volume(95.0).unwrap(), 48);
    assert_eq!(orderbook.bid_price_level_volume(94.0).unwrap(), 42);
    assert_eq!(orderbook.bid_price_level_volume(93.0).unwrap(), 34);
    assert_eq!(orderbook.bid_price_level_volume(92.0).unwrap(), 20);
    assert_eq!(orderbook.bid_price_level_volume(91.0).unwrap(), 11);

    assert_eq!(orderbook.ask_price_level_volume(105.0).unwrap(), 13);
    assert_eq!(orderbook.ask_price_level_volume(106.0).unwrap(), 77);
    assert_eq!(orderbook.ask_price_level_volume(107.0).unwrap(), 129);
    assert_eq!(orderbook.ask_price_level_volume(108.0).unwrap(), 73);
    assert_eq!(orderbook.ask_price_level_volume(109.0).unwrap(), 70);
    assert_eq!(orderbook.ask_price_level_volume(110.0).unwrap(), 41);
    assert_eq!(orderbook.ask_price_level_volume(111.0).unwrap(), 40);
    assert_eq!(orderbook.ask_price_level_volume(112.0).unwrap(), 32);
    assert_eq!(orderbook.ask_price_level_volume(113.0).unwrap(), 23);
    assert_eq!(orderbook.ask_price_level_volume(114.0).unwrap(), 9);
}

fn bid_test(orderbook: &OrderBook) {
    println!("----bid----");

    let mut price_list = orderbook.bid_price_list();
    let price_list: Vec<f32> = price_list.iter_mut().map(|x| x.into_inner()).collect();
    for price in &price_list {
        let volume = orderbook.bid_price_level_volume(*price).unwrap();
        let size = orderbook.bid_price_level_size(*price).unwrap();
        println!("price {price} => size: {size}, volume: {volume}");
    }
    println!("bid price list: {price_list:?}");
    let best_bid = orderbook.best_bid();
    println!("best bid: {best_bid:?}");
    let worst_bid = orderbook.worst_bid();
    println!("worst bid: {worst_bid:?}");
}

fn ask_test(orderbook: &OrderBook) {
    println!("----ask----");

    let mut price_list = orderbook.ask_price_list();
    let price_list: Vec<f32> = price_list.iter_mut().map(|x| x.into_inner()).collect();
    for price in &price_list {
        let volume = orderbook.ask_price_level_volume(*price).unwrap();
        let size = orderbook.ask_price_level_size(*price).unwrap();
        println!("price {price} => size: {size}, volume: {volume}");
    }
    println!("ask price list: {price_list:?}");
    let best_ask = orderbook.best_ask();
    println!("best ask: {best_ask:?}");
    let worst_ask = orderbook.worst_ask();
    println!("worst ask: {worst_ask:?}");
}
