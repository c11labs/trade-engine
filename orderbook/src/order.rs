use crate::order_side::OrderSide;
use crate::order_type::OrderType;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Order {
    pub order_id: u32,
    pub user_id: u32,
    pub pair: String,
    pub allowance: f64,
    pub amount: f64,
    pub price: Option<f64>,
    pub side: OrderSide,
    pub r#type: OrderType,
}

impl Order {
    pub fn new(
        order_id: u32,
        user_id: u32,
        allowance: f64,
        pair: String,
        price: Option<f64>,
        amount: f64,
        side: OrderSide,
        r#type: OrderType,
    ) -> Self {
        Self {
            order_id,
            user_id,
            allowance,
            pair,
            price,
            amount,
            side,
            r#type,
        }
    }
}
