use crate::order::Order;
use anyhow::{anyhow, Result};
use std::cmp::min;

#[derive(Debug)]
pub struct PriceLevel {
    price: f32,  // price limit of this price level
    volume: u32, // total number of shares of all order in this price level
    orders: Vec<Order>,
}

impl PriceLevel {
    pub fn new(price: f32) -> Self {
        Self {
            price,
            volume: 0,
            orders: Vec::new(),
        }
    }

    pub fn add(&mut self, order: Order) -> Result<()> {
        self.volume += order.quantity;
        self.orders.push(order);

        Ok(())
    }

    pub fn remove(&mut self, order_id: u32) -> Result<()> {
        let (index, order): (usize, &Order) = self.get_order(order_id)?;
        self.volume -= order.quantity;
        self.orders.remove(index);

        Ok(())
    }

    pub fn modify(&mut self, order_id: u32, order: Order) -> Result<()> {
        self.remove(order_id)?;
        self.add(order)?;

        Ok(())
    }

    pub fn get_order(&self, order_id: u32) -> Result<(usize, &Order)> {
        for (index, order) in self.orders.iter().enumerate() {
            if order.order_id == order_id {
                return Ok((index, &order));
            }
        }

        Err(anyhow!("order not found"))
    }

    pub fn match_order(&mut self, match_order: &mut Order) -> Result<()> {
        // println!("----------");
        let mut num_removed = 0;
        for order in &mut self.orders {
            let num_quantity = min(match_order.quantity, order.quantity);
            match_order.quantity -= num_quantity;
            order.quantity -= num_quantity;
            self.volume -= num_quantity;

            if order.quantity == 0 {
                num_removed += 1;
            }

            // println!("{order:?}");

            if match_order.quantity == 0 {
                break;
            }
        }

        self.orders.drain(0..num_removed);

        Ok(())
    }

    pub fn size(&self) -> u32 {
        self.orders.len().try_into().unwrap()
    }

    pub fn volume(&self) -> u32 {
        self.volume
    }

    pub fn is_empty(&self) -> bool {
        self.orders.is_empty()
    }
}
