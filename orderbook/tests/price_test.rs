use orderbook::price::{AskPrice, BidPrice};

#[test]
fn bid_price_test() {
    assert_eq!(BidPrice(0.0) > BidPrice(1.0), true);
    assert_eq!(BidPrice(0.0) >= BidPrice(1.0), true);
    assert_eq!(BidPrice(0.0) < BidPrice(1.0), false);
    assert_eq!(BidPrice(0.0) <= BidPrice(1.0), false);
    assert_eq!(BidPrice(0.0) == BidPrice(1.0), false);
    assert_eq!(BidPrice(0.0) == BidPrice(0.0), true);

    assert_eq!(AskPrice(0.0) > AskPrice(1.0), false);
    assert_eq!(AskPrice(0.0) >= AskPrice(1.0), false);
    assert_eq!(AskPrice(0.0) < AskPrice(1.0), true);
    assert_eq!(AskPrice(0.0) <= AskPrice(1.0), true);
    assert_eq!(AskPrice(0.0) == AskPrice(1.0), false);
    assert_eq!(AskPrice(0.0) == AskPrice(0.0), true);
}
