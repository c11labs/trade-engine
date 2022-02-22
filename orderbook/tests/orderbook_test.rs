use orderbook::OrderBook;
// use orderbook::order_side::OrderSide;
// use ordered_float::OrderedFloat;

#[test]
fn orderbook_test() {
    let mut orderbook: OrderBook = OrderBook::new(10);
    assert_eq!(orderbook.instrument_id(), 10);

    orderbook.bid(1.0, 1000).unwrap();
    orderbook.bid(2.0, 1000).unwrap();
    orderbook.bid(3.0, 1000).unwrap();
    orderbook.bid(10.0, 1000).unwrap();
    orderbook.bid(0.1, 1000).unwrap();
    orderbook.bid(0.001, 1000).unwrap();

    let mut price_list = orderbook.bid_price_list().unwrap();
    let price_list: Vec<f32> = price_list.iter_mut().map(|x| x.into_inner()).collect();
    println!("price list: {price_list:?}");

    let best_bid = orderbook.best_bid().unwrap();
    println!("best bid: {best_bid:?}");

    let worst_bid = orderbook.worst_bid().unwrap();
    println!("worst bid: {worst_bid:?}");
}
