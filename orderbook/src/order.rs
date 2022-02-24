use crate::order_side::OrderSide;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Order {
    pub order_id: u32,
    pub user_id: u32,
    pub instrument_id: String,
    pub quantity: u32,
    pub price: f32,
    pub side: OrderSide,
}

impl Order {
    pub fn new(
        order_id: u32,
        user_id: u32,
        instrument_id: String,
        price: f32,
        quantity: u32,
        side: OrderSide,
    ) -> Self {
        Self {
            order_id,
            user_id,
            instrument_id,
            price,
            quantity,
            side,
        }
    }
}
