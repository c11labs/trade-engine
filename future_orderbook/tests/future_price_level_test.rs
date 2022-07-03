use future_orderbook::future_price_level::FuturePriceLevel;

#[test]
fn future_price_level_test() {
    let price_level = FuturePriceLevel::new(5.55);
    println!("{price_level:?}");
}
