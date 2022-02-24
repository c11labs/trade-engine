use orderbook::order::Order;
use orderbook::order_side::OrderSide;
use orderbook::price::IntoInner;
use orderbook::OrderBook;

#[test]
fn orderbook_test() {
    let mut orderbook: OrderBook = OrderBook::new(10);
    assert_eq!(orderbook.instrument_id(), 10);

    let bid_prices: Vec<(f32, i32)> = vec![(3.0, 2), (2.0, 1), (1.0, 3)];
    let ask_prices: Vec<(f32, i32)> = vec![(0.01, 2), (8.0, 8), (5.0, 5)];

    for (price, times) in &bid_prices {
        for _ in 0..*times {
            let order: Order = Order::new(1, 1, "test".to_string(), *price, 100, OrderSide::Bid);
            orderbook.bid(*price, order).unwrap();
        }
    }

    for (price, times) in &ask_prices {
        for _ in 0..*times {
            let order: Order = Order::new(1, 1, "test".to_string(), *price, 100, OrderSide::Bid);
            orderbook.ask(*price, order).unwrap();
        }
    }

    bid_test(&orderbook, &bid_prices);
    ask_test(&orderbook, &ask_prices);
}

fn bid_test(orderbook: &OrderBook, prices: &Vec<(f32, i32)>) {
    println!("------");
    for (price, _) in prices {
        let volume = orderbook.bid_price_level_volume(*price).unwrap();
        let size = orderbook.bid_price_level_size(*price).unwrap();
        println!("level {price} => size: {size}, volume: {volume}");
    }
    let mut price_list = orderbook.bid_price_list();
    let price_list: Vec<f32> = price_list.iter_mut().map(|x| x.into_inner()).collect();
    println!("bid price list: {price_list:?}");
    let best_bid = orderbook.best_bid();
    println!("best bid: {best_bid:?}");
    let worst_bid = orderbook.worst_bid();
    println!("worst bid: {worst_bid:?}");
}

fn ask_test(orderbook: &OrderBook, prices: &Vec<(f32, i32)>) {
    println!("------");
    for (price, _) in prices {
        let volume = orderbook.ask_price_level_volume(*price).unwrap();
        let size = orderbook.ask_price_level_size(*price).unwrap();
        println!("level {price} => size: {size}, volume: {volume}");
    }
    let mut price_list = orderbook.ask_price_list();
    let price_list: Vec<f32> = price_list.iter_mut().map(|x| x.into_inner()).collect();
    println!("ask price list: {price_list:?}");
    let best_ask = orderbook.best_ask();
    println!("best ask: {best_ask:?}");
    let worst_ask = orderbook.worst_ask();
    println!("worst ask: {worst_ask:?}");
}
