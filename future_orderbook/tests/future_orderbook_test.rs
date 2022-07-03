use future_orderbook::future_margin::FutureMargin;
use future_orderbook::future_order::FutureOrder;
use future_orderbook::future_order_side::FutureOrderSide;
use future_orderbook::future_order_type::FutureOrderType;
use future_orderbook::future_price::LongPrice;
use future_orderbook::FutureOrderBook;

#[test]
fn future_orderbook_test() {
    let mut orderbook = FutureOrderBook::new(String::from("sol-usdt"));

    add_long_order(&mut orderbook);
    add_short_order(&mut orderbook);

    let (long, short) = orderbook.price_and_size();
    println!("long {long:?}");
    println!("short {short:?}");

    long_test(&orderbook);
    short_test(&orderbook);

    /* orderbook.impact_price_series(FutureOrderSide::Short);
    orderbook.impact_price_series(FutureOrderSide::Long); */

    orderbook.calc_premium_index();
    let premium_index = orderbook.avg_premium_index();
    println!("avg premuim index {premium_index}");

    let funding_rate = orderbook.funding_rate();
    println!("funding rate {funding_rate}");
}

fn add_long_order(orderbook: &mut FutureOrderBook) {
    let price_size: Vec<(f64, f64)> = vec![
        (11409.62, 0.893),
        (11408.74, 0.215),
        (11408.69, 0.140),
        (11408.04, 0.120),
        (11407.96, 0.438),
        (11407.92, 0.620),
    ];

    for (price, size) in &price_size {
        let order = FutureOrder::new(
            1,
            1,
            FutureMargin::Isolated(5.55),
            String::from("sol-usdt"),
            Some(*price),
            *size,
            /*leverage=*/ 10,
            FutureOrderSide::Long,
            FutureOrderType::Limit,
        );
        // println!("{order:?}");
        orderbook.add(order).unwrap();
    }
}

fn add_short_order(orderbook: &mut FutureOrderBook) {
    let price_size: Vec<(f64, f64)> = vec![
        (11410.54, 2.850),
        (11410.50, 0.065),
        (11410.49, 0.079),
        (11410.08, 0.616),
        (11409.78, 0.008),
        (11409.63, 0.499),
    ];

    for (price, size) in &price_size {
        let order = FutureOrder::new(
            1,
            1,
            FutureMargin::Isolated(5.55),
            String::from("sol-usdt"),
            Some(*price),
            *size,
            /*leverage=*/ 10,
            FutureOrderSide::Short,
            FutureOrderType::Limit,
        );
        // println!("{order:?}");
        orderbook.add(order).unwrap();
    }
}

fn long_test(orderbook: &FutureOrderBook) {
    println!("----long----");
    let price_list = orderbook.price_list(FutureOrderSide::Long);
    for price in &price_list {
        let volume = orderbook.size(*price, FutureOrderSide::Long).unwrap();
        let size = orderbook.num_order(*price, FutureOrderSide::Long).unwrap();
        println!("price {price} => size: {size}, volume: {volume}");
    }
    println!("long price list: {price_list:?}");
}

fn short_test(orderbook: &FutureOrderBook) {
    println!("----short----");
    let price_list = orderbook.price_list(FutureOrderSide::Short);
    for price in &price_list {
        let volume = orderbook.size(*price, FutureOrderSide::Short).unwrap();
        let size = orderbook.num_order(*price, FutureOrderSide::Short).unwrap();
        println!("price {price} => size: {size}, volume: {volume}");
    }
    println!("short price list: {price_list:?}");
}
