use crate::order_data::OrderData;
use anyhow::{anyhow, Result};
use orderbook_derive::OrderTrait;
use std::fmt::Debug;

trait OrderTrait {
    fn print(&self);
    fn increase_quantity(&mut self, quantity: u64) -> Result<()>;
    fn decrease_quantity(&mut self, quantity: u64) -> Result<()>;

    fn cancel(order_id: u64) -> Result<()> {
        println!("cancel order {order_id}");

        Ok(())
    }

    fn new(data: OrderData, price: i64, quantity: u64, is_buy_side: bool) -> Result<Order> {
        Ok(Order {
            data,
            price,
            initial_quantity: quantity,
            current_quantity: quantity,
            is_buy_side
        })
    }

    fn get(order_id: u64) -> Result<Order> {
        Ok(Order {
            data: OrderData {
                order_id,
                ..Default::default()
            },
            ..Default::default()
        })
    }
}

#[derive(OrderTrait, Debug, Default)]
pub struct Order {
    pub data: OrderData,
    pub initial_quantity: u64,
    pub current_quantity: u64,
    pub price: i64,
    pub is_buy_side: bool,
}

pub enum OrderType {
    NewOrder {
        data: OrderData,
        price: i64,
        quantity: u64,
        is_buy_side: bool
    },
    ModifyOrder {
        order_id: u64,
        new_price: i64,
        new_quantity: u64,
        is_buy_side: bool,
    },
    CancelOrder {
        order_id: u64,
    },
}

impl OrderType {
    pub fn process(&self) -> Result<()> {
        match self {
            OrderType::NewOrder { data, price, quantity, is_buy_side } => {
                let data = OrderData {
                    ..Default::default()
                };
                let new_order = Order::new(data, *price, *quantity, *is_buy_side);
                println!("{new_order:?}");
            }
            OrderType::ModifyOrder {
                order_id,
                new_price,
                new_quantity,
                is_buy_side,
            } => {
                let order: Order = Order::get(*order_id)?;
                Order::cancel(*order_id)?;
                
                let modified_order = Order::new(order.data, *new_price, *new_quantity, *is_buy_side);
                println!("{modified_order:?}");
            }
            OrderType::CancelOrder { order_id } => {
                Order::cancel(*order_id)?;
            }
        }

        Ok(())
    }
}
