use crate::order_side::OrderSide;
use anyhow::Result;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Order {
    pub order_id: u32,
    pub user_name: String,
    pub security_id: u32,
    pub quantity: u32,
    pub price: f32,
    pub side: OrderSide,
}

impl Order {
    pub fn new(
        order_id: u32,
        user_name: String,
        security_id: u32,
        price: f32,
        quantity: u32,
        side: OrderSide,
    ) -> Result<Self> {
        Ok(Self {
            order_id,
            user_name,
            security_id,
            price,
            quantity,
            side,
        })
    }
}
