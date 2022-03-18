use clap::Parser;
use orderbook::OrderBook;
use server::consumer::consume_stream;
use server::utils::Args;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut orderbook: OrderBook = OrderBook::new(args.pair.clone());

    let incoming_topic: String = format!("{}-orders", orderbook.pair());
    let trade_topic: String = format!("{}-trades", orderbook.pair());
    let orderbook_data_topic: String = format!("{}-orderbook-data", orderbook.pair());

    println!("incoming topic: {incoming_topic}");
    println!("trade topic: {trade_topic}");
    println!("orderbook data topic: {orderbook_data_topic}");

    if let Err(err) = consume_stream(
        "localhost:9093",
        &incoming_topic,
        &trade_topic,
        &orderbook_data_topic,
        &args.pair,
        &mut orderbook,
    )
    .await
    {
        println!("{err:?}");
    }
}
