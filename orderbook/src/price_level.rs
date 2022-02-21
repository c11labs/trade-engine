use crate::order::Order;
use crate::order_side::OrderSide;
use anyhow::{anyhow, Result};

#[derive(Debug)]
pub struct PriceLevel<'a> {
    price: f32,  // price limit of this price level
    size: u32,   // number of order in this price level
    volume: u32, // total number of shares of all order in this price level
    orders: Vec<&'a Order>,
}

impl<'a> PriceLevel<'a> {
    pub fn new(price: f32) -> Self {
        Self {
            price,
            size: 0,
            volume: 0,
            orders: vec![],
        }
    }

    pub fn add(&mut self, order: &'a Order) -> Result<()> {
        self.orders.push(order);
        self.size += 1;
        self.volume += order.quantity;

        Ok(())
    }

    pub fn remove(&mut self, order_id: u32) -> Result<()> {
        let (index, order): (usize, &Order) = self.get_order(order_id)?;
        self.volume -= order.quantity;
        self.size -= 1;
        self.orders.remove(index);

        Ok(())
    }

    pub fn modify(&mut self, order_id: u32, order: &'a Order) -> Result<()> {
        self.remove(order_id)?;
        self.add(&order)?;

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

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn volume(&self) -> u32 {
        self.volume
    }
}
