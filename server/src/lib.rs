pub mod consumer;
pub mod utils;

pub mod proto {
    pub mod order {
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/order.rs"));
    }

    pub mod trade {
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/trade.rs"));
    }

    pub mod request {
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/request.rs"));
    }

    pub mod orderbook {
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/orderbook.rs"));
    }
}

use orderbook::{
    order::Order,
    order_side::OrderSide,
    order_type::OrderType,
    pair::PriceSizePair,
    trade::{MatchedOrder, MatchedPair, Trade},
};
use proto::{
    order::Order as OrderMessage,
    orderbook::PriceSizePair as PriceSizePairMessage,
    trade::{
        MatchedOrder as MatchedOrderMessage, MatchedPair as MatchedPairMessage,
        Trade as TradeMessage,
    },
};
use std::convert::From;

impl From<OrderMessage> for Order {
    fn from(item: OrderMessage) -> Self {
        Self {
            order_id: item.order_id,
            user_id: item.user_id,
            allowance: item.allowance,
            pair: item.pair,
            price: if item.price == 0.0 {
                None
            } else {
                Some(item.price)
            },
            amount: item.amount,
            side: match item.side {
                0 => OrderSide::Ask,
                1 => OrderSide::Bid,
                _ => OrderSide::Ask,
            },
            r#type: match item.r#type {
                0 => OrderType::Limit,
                1 => OrderType::Market,
                /* 2 => OrderType::StopLimit,
                3 => OrderType::Oco, */
                _ => OrderType::Limit,
            },
        }
    }
}

impl From<MatchedOrder> for MatchedOrderMessage {
    fn from(item: MatchedOrder) -> Self {
        Self {
            order_id: item.order_id,
            user_id: item.user_id,
            amount: item.amount,
        }
    }
}

impl From<MatchedPair> for MatchedPairMessage {
    fn from(item: MatchedPair) -> Self {
        Self {
            price: item.price,
            init_order: Some(MatchedOrderMessage::from(item.init_order)),
            matched_orders: item
                .matched_orders
                .into_iter()
                .map(|x| MatchedOrderMessage::from(x))
                .collect(),
        }
    }
}

impl From<Trade> for TradeMessage {
    fn from(item: Trade) -> Self {
        Self {
            init_side: match item.init_side {
                OrderSide::Ask => 0,
                OrderSide::Bid => 1,
            },
            init_type: match item.init_type {
                OrderType::Limit => 0,
                OrderType::Market => 1,
                /* OrderType::StopLimit => 2,
                OrderType::Oco => 3, */
            },
            trades: item
                .trades
                .into_iter()
                .map(|x| MatchedPairMessage::from(x))
                .collect(),
        }
    }
}

impl From<PriceSizePair> for PriceSizePairMessage {
    fn from(item: PriceSizePair) -> Self {
        Self {
            price: item.price,
            size: item.size,
        }
    }
}
