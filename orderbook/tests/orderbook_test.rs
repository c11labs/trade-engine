use orderbook::OrderBook;

#[test]
fn orderbook_test() {
    let mut orderbook: OrderBook = OrderBook::new(10);
    assert_eq!(orderbook.instrument_id(), 10);

    bid_test(&mut orderbook);
    ask_test(&mut orderbook);

    
}

fn bid_test(orderbook: &mut OrderBook) {
    orderbook.bid(1.0, 1000);
    orderbook.bid(2.0, 1000);
    orderbook.bid(3.0, 1000);
    orderbook.bid(10.0, 1000);
    orderbook.bid(0.1, 1000);
    orderbook.bid(0.001, 1000);
    let mut price_list = orderbook.bid_price_list();
    let price_list: Vec<f32> = price_list.iter_mut().map(|x| x.into_inner()).collect();
    println!("bid price list: {price_list:?}");
    let best_bid = orderbook.best_bid();
    println!("best bid: {best_bid:?}");
    let worst_bid = orderbook.worst_bid();
    println!("worst bid: {worst_bid:?}");
}

fn ask_test(orderbook: &mut OrderBook) {
    orderbook.ask(11.0, 1000);
    orderbook.ask(0.04, 1000);
    orderbook.ask(100.0, 1000);
    orderbook.ask(33.0, 1000);
    orderbook.ask(5.0, 1000);
    orderbook.ask(60.0, 1000);
    let mut price_list = orderbook.ask_price_list();
    let price_list: Vec<f32> = price_list.iter_mut().map(|x| x.into_inner()).collect();
    println!("ask price list: {price_list:?}");
    let best_ask = orderbook.best_ask();
    println!("best ask: {best_ask:?}");
    let worst_ask = orderbook.worst_ask();
    println!("worst ask: {worst_ask:?}");
}
