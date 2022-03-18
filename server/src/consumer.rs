use crate::proto::{
    orderbook::{OrderBook as OrderBookMessage, PriceSizePair as PriceSizePairMessage},
    request,
    request::{request::Command, Request},
    trade::Trade as TradeMessage,
};
use crate::utils::{create_consumer, create_producer};
use anyhow::{bail, Context, Result};
use futures::future;
use log::warn;
use orderbook::{order::Order, order_side::OrderSide, OrderBook};
use prost::Message;
use rdkafka::{producer::FutureRecord, Message as KafkaMessage};
use std::time::Duration;

fn deserialize_payload(payload: Option<&[u8]>) -> Result<Request> {
    if let Some(bytes) = payload {
        return Ok(Request::decode(bytes)?);
    }

    bail!("error deserializing payload")
}

fn process<M: KafkaMessage>(
    message: &M,
    orderbook: &mut OrderBook,
) -> Result<Option<TradeMessage>> {
    match deserialize_payload(message.payload())?
        .command
        .context("error deserializing request")?
    {
        Command::Add(request::Add { order }) => {
            let order = Order::from(order.context("error deserialize order")?);

            println!(
                "Consumed record from topic {} partition [{}] @ offset {} with key {} and value {:?}",
                message.topic(),
                message.partition(),
                message.offset(),
                message.key_view::<str>().context("key view error")??,
                order
            );

            if let Some(trade) = orderbook.add(order)? {
                let message: TradeMessage = trade.into();
                println!("{message:#?}");
                return Ok(Some(message));
            }
        }
        Command::Cancel(request::Cancel {
            side,
            price,
            order_id,
        }) => {
            let side = if side == 0 {
                OrderSide::Ask
            } else {
                OrderSide::Bid
            };
            if let Ok(()) = orderbook.cancel(side, price, order_id) {}
        }
        Command::Modify(request::Modify {
            order,
            new_price,
            new_amount,
        }) => {
            let order = Order::from(order.context("error deserialize order")?);
            let new_price: Option<f32> = if new_price > 0.0 {
                Some(new_price)
            } else {
                None
            };
            let new_amount: Option<f32> = if new_amount > 0.0 {
                Some(new_amount)
            } else {
                None
            };
            if let Some(trade) = orderbook.modify(order, new_price, new_amount)? {
                let message: TradeMessage = trade.into();
                println!("{message:#?}");
                return Ok(Some(message));
            }
        }
    }
    Ok(None)
}

pub async fn consume_stream(
    brokers: &str,
    incoming_topic: &str,
    trade_topic: &str,
    orderbook_data_topic: &str,
    pair: &str,
    orderbook: &mut OrderBook,
) -> Result<()> {
    let producer = create_producer(brokers);

    let send_data_to_kafka = move |trade: Vec<u8>, orderbook_data: Vec<u8>| {
        let producer = producer.clone();
        async move {
            let mut tasks = Vec::new();

            if trade.len() != 0 {
                let matched_order_record = FutureRecord::to(trade_topic).payload(&trade).key(pair);
                let task = producer.send(matched_order_record, Duration::from_secs(0));
                tasks.push(task);
            };

            let orderbook_data = FutureRecord::to(orderbook_data_topic)
                .payload(&orderbook_data)
                .key(pair);
            let task = producer.send(orderbook_data, Duration::from_secs(0));
            tasks.push(task);

            if let Ok(_results) = future::try_join_all(tasks).await {
                /* for result in results {
                    println!("{result:?}");
                } */
            }
        }
    };

    let consumer = create_consumer(brokers, orderbook.pair(), incoming_topic);
    loop {
        match consumer.recv().await {
            Ok(message) => {
                match process(&message, orderbook) {
                    Ok(data) => {
                        let (ask_pairs, bid_pairs) = orderbook.price_and_size();
                        let orderbook_data: OrderBookMessage = OrderBookMessage {
                            ask_pairs: ask_pairs
                                .into_iter()
                                .map(|x| PriceSizePairMessage::from(x))
                                .collect(),
                            bid_pairs: bid_pairs
                                .into_iter()
                                .map(|x| PriceSizePairMessage::from(x))
                                .collect(),
                        };
                        let orderbook_data: Vec<u8> = orderbook_data.encode_to_vec();

                        if let Some(trade) = data {
                            let trade: Vec<u8> = trade.encode_to_vec();
                            send_data_to_kafka(trade, orderbook_data).await;
                        } else {
                            send_data_to_kafka(Vec::new(), orderbook_data).await;
                        }
                    }
                    Err(err) => {
                        eprintln!("{err:?}");
                    }
                }

                // Now that the message is completely processed, add it's position to the offset
                // store. The actual offset will be committed every 5 seconds.
                /* if let Err(e) = consumer.store_offset_from_message(&message) {
                    warn!("Error while storing offset: {}", e);
                } */
            }
            Err(e) => {
                warn!("Kafka error: {}", e);
            }
        }
    }
}
