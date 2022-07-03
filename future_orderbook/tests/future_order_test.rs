use future_orderbook::future_margin::FutureMargin;
use future_orderbook::future_order::FutureOrder;
use future_orderbook::future_order_side::FutureOrderSide;
use future_orderbook::future_order_type::FutureOrderType;

#[test]
fn future_order_test() {
    let order = FutureOrder::new(
        1,
        1,
        FutureMargin::Isolated(5.55),
        String::from("sol-usdt"),
        Some(10.11111),
        7.777,
        10,
        FutureOrderSide::Long,
        FutureOrderType::Limit,
    );

    println!("{order:?}");
}
