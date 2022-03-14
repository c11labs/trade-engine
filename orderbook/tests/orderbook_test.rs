use orderbook::order::Order;
use orderbook::order_side::OrderSide;
use orderbook::price::IntoInner;
use orderbook::OrderBook;
use orderbook::order_type::OrderType;

#[test]
fn orderbook_test() {
    let mut orderbook: OrderBook = OrderBook::new(10);
    assert_eq!(orderbook.instrument_id(), 10);

    // init prices
    let bid_prices: Vec<(f32, i32)> = vec![(3.0, 3), (2.0, 2), (1.0, 1)];
    let ask_prices: Vec<(f32, i32)> = vec![(5.0, 5), (8.0, 8), (10.0, 10)];

    // init bid limit order
    for (price, times) in &bid_prices {
        for _ in 0..*times {
            let order: Order = Order::new(1, 1, "test".to_string(), Some(*price), 100, OrderSide::Bid, OrderType::Limit);
            orderbook.bid(order).unwrap();
        }
    }

    // init ask limit order
    for (price, times) in &ask_prices {
        for _ in 0..*times {
            let order: Order = Order::new(1, 1, "test".to_string(), Some(*price), 100, OrderSide::Ask, OrderType::Limit);
            orderbook.ask(order).unwrap();
        }
    }

    ask_test(&orderbook);
    bid_test(&orderbook);

    // limit order matching
    let order: Order = Order::new(1, 1, "test".to_string(), Some(8.0), 2000, OrderSide::Bid, OrderType::Limit);
    orderbook.bid(order).unwrap();

    let order: Order = Order::new(1, 1, "test".to_string(), Some(2.0), 2000, OrderSide::Ask, OrderType::Limit);
    orderbook.ask(order).unwrap();

    ask_test(&orderbook);
    bid_test(&orderbook);

    // market order matching
    let order = Order::new(1, 1, "test".to_string(), None, 2000, OrderSide::Bid, OrderType::Market);
    orderbook.bid(order).unwrap();

    ask_test(&orderbook);
    bid_test(&orderbook);
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
    /* println!("bid price list: {price_list:?}");
    let best_bid = orderbook.best_bid();
    println!("best bid: {best_bid:?}");
    let worst_bid = orderbook.worst_bid();
    println!("worst bid: {worst_bid:?}"); */
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
    /* println!("ask price list: {price_list:?}");
    let best_ask = orderbook.best_ask();
    println!("best ask: {best_ask:?}");
    let worst_ask = orderbook.worst_ask();
    println!("worst ask: {worst_ask:?}"); */
}
