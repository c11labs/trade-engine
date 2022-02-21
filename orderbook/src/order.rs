use crate::order_side::OrderSide;
use anyhow::Result;
// use orderbook_derive::OrderTrait;
use std::fmt::Debug;

pub trait OrderTrait {
    /* fn cancel(order_id: u32) -> Result<()> {
        println!("cancel order {order_id}");

        Ok(())
    }

    fn new(
        order_id: u32,
        user_name: String,
        security_id: u32,
        price: f32,
        quantity: u32,
        side: OrderSide,
    ) -> Result<Order> {
        Ok(Order {
            order_id,
            user_name,
            security_id,
            price,
            initial_quantity: quantity,
            current_quantity: quantity,
            side,
        })
    }

    fn get(order_id: u32) -> Result<Order> {
        Ok(Order {
            order_id,
            user_name: "".to_string(),
            security_id: 1,
            initial_quantity: 0,
            current_quantity: 0,
            price: 0.0,
            side: OrderSide::Ask,
        })
    } */
}

#[derive(Debug)]
pub struct Order {
    pub order_id: u32,
    pub user_name: String,
    pub security_id: u32,
    pub initial_quantity: u32,
    pub current_quantity: u32,
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
    ) -> Result<Order> {
        Ok(Order {
            order_id,
            user_name,
            security_id,
            price,
            initial_quantity: quantity,
            current_quantity: quantity,
            side,
        })
    }

    pub fn get(order_id: u32) -> Result<Order> {
        Ok(Order {
            order_id,
            user_name: "".to_string(),
            security_id: 1,
            initial_quantity: 0,
            current_quantity: 0,
            price: 0.0,
            side: OrderSide::Ask,
        })
    }
    
    pub fn cancel(order_id: u32) -> Result<()> {
        println!("cancel order {order_id}");

        Ok(())
    }
}
