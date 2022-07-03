use future_orderbook::future_margin::FutureMargin;

#[test]
fn future_margin_test() {
    let cross = FutureMargin::Cross(5.55);
    let isolated = FutureMargin::Isolated(5.55);
    println!("{cross:?}");
    println!("{isolated:?}");

    if let FutureMargin::Cross(margin) = cross {
        println!("{margin:?}");
    };

    if let FutureMargin::Isolated(margin) = isolated {
        println!("{margin:?}");
    };
}
