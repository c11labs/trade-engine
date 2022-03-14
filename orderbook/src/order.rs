use crate::order_side::OrderSide;
use crate::order_type::OrderType;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Order {
    pub order_id: u32,
    pub user_id: u32,
    pub instrument_id: String,
    pub quantity: u32,
    pub price: Option<f32>,
    pub side: OrderSide,
    pub r#type: OrderType,
}

impl Order {
    pub fn new(
        order_id: u32,
        user_id: u32,
        instrument_id: String,
        price: Option<f32>,
        quantity: u32,
        side: OrderSide,
        r#type: OrderType,
    ) -> Self {
        Self {
            order_id,
            user_id,
            instrument_id,
            price,
            quantity,
            side,
            r#type,
        }
    }
}
