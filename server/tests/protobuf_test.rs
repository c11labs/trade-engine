use orderbook::pair::PriceSizePair;
use server::proto::orderbook::{OrderBook, PriceSizePair as PriceSizePairMessage};

#[test]
fn price_size_pair_test() {
    let pair = PriceSizePair {
        price: 5.5,
        size: 10.1,
    };

    let message = PriceSizePairMessage::from(pair);
    let pairs = vec![message; 5];

    let orderbook = OrderBook {
        ask_pairs: pairs,
        bid_pairs: Vec::new(),
    };
    println!("{orderbook:?}");
}
