use crate::future_margin::FutureMargin;
use crate::future_order_side::FutureOrderSide;
use crate::future_order_type::FutureOrderType;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct FutureOrder {
    pub order_id: u32,
    pub user_id: u32,
    pub pair: String,
    pub margin: FutureMargin,
    pub amount: f64,
    pub price: Option<f64>,
    pub leverage: u32,
    pub side: FutureOrderSide,
    pub r#type: FutureOrderType,
}

impl FutureOrder {
    pub fn new(
        order_id: u32,
        user_id: u32,
        margin: FutureMargin,
        pair: String,
        price: Option<f64>,
        amount: f64,
        leverage: u32,
        side: FutureOrderSide,
        r#type: FutureOrderType,
    ) -> Self {
        Self {
            order_id,
            user_id,
            margin,
            pair,
            price,
            amount,
            leverage,
            side,
            r#type,
        }
    }
}
