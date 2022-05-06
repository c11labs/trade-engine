use clap::Parser;
use orderbook::OrderBook;
use server::consumer::consume_stream;
use server::utils::Args;
use std::env;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut orderbook: OrderBook = OrderBook::new(args.pair.clone());

    let incoming_topic: String = format!("{}-orders", orderbook.pair());
    let trade_topic: String = format!("{}-trades", orderbook.pair());
    let orderbook_data_topic: String = format!("{}-orderbook-data", orderbook.pair());

    let kafka_producer_broker: String = env::var("KAFKA_PRODUCER_BROKER").unwrap();
    let kafka_consumer_broker: String = env::var("KAFKA_CONSUMER_BROKER").unwrap();

    println!("kafka producer broker: {kafka_producer_broker}");
    println!("kafka consumer broker: {kafka_consumer_broker}");
    println!("incoming topic: {incoming_topic}");
    println!("trade topic: {trade_topic}");
    println!("orderbook data topic: {orderbook_data_topic}");

    if let Err(err) = consume_stream(
        &kafka_producer_broker,
        &kafka_consumer_broker,
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
