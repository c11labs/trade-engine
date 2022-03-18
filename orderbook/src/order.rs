use crate::order_side::OrderSide;
use crate::order_type::OrderType;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Order {
    pub order_id: u32,
    pub user_id: u32,
    pub pair: String,
    pub amount: f32,
    pub price: Option<f32>,
    pub side: OrderSide,
    pub r#type: OrderType,
}

impl Order {
    pub fn new(
        order_id: u32,
        user_id: u32,
        pair: String,
        price: Option<f32>,
        amount: f32,
        side: OrderSide,
        r#type: OrderType,
    ) -> Self {
        Self {
            order_id,
            user_id,
            pair,
            price,
            amount,
            side,
            r#type,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MatchedOrder {
    pub order_id: u32,
    pub user_id: u32,
    pub price: f32,
    pub amount: f32,
}

impl MatchedOrder {
    pub fn new(order_id: u32, user_id: u32, price: f32, amount: f32) -> Self {
        Self {
            order_id,
            user_id,
            price,
            amount,
        }
    }
}
