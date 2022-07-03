use crate::future_order::FutureOrder;
use crate::future_order_side::FutureOrderSide;
use crate::future_order_type::FutureOrderType;
// use crate::trade::MatchedOrder;
use anyhow::{anyhow, Result};

#[derive(Debug)]
pub struct FuturePriceLevel {
    price: f64, // price limit of this price level
    size: f64,  // total number of shares of all order in this price level
    orders: Vec<FutureOrder>,
}

fn min(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}

impl FuturePriceLevel {
    pub fn new(price: f64) -> Self {
        Self {
            price,
            size: 0.0,
            orders: Vec::new(),
        }
    }

    pub fn add(&mut self, order: FutureOrder) -> Result<()> {
        self.size += order.amount;
        self.orders.push(order);

        Ok(())
    }

    pub fn remove(&mut self, order_id: u32) -> Result<()> {
        let (index, order): (usize, &FutureOrder) = self.get_order(order_id)?;
        self.size -= order.amount;
        self.orders.remove(index);

        Ok(())
    }

    pub fn get_order(&self, order_id: u32) -> Result<(usize, &FutureOrder)> {
        for (index, order) in self.orders.iter().enumerate() {
            if order.order_id == order_id {
                return Ok((index, &order));
            }
        }

        Err(anyhow!("order not found"))
    }

    pub fn match_order(
        &mut self,
        init_order: &mut FutureOrder,
        // ) -> Result<(MatchedOrder, Vec<MatchedOrder>)> {
    ) -> Result<()> {
        Ok(())
        /* let mut matched_orders: Vec<MatchedOrder> = Vec::new();
        let mut total_matched_amount: f64 = 0.0;
        let mut num_order_removed = 0;

        for order in &mut self.orders {
            let amount = match init_order.r#type {
                FutureOrderType::Market => match init_order.side {
                    FutureOrderSide::Short => min(init_order.amount, order.amount),
                    FutureOrderSide::Long => {
                        let available_amount = init_order.allowance / self.price;
                        min(available_amount, order.amount)
                    }
                },
                FutureOrderType::Limit => min(init_order.amount, order.amount),
                _ => min(init_order.amount, order.amount)
            };

            match init_order.side {
                FutureOrderSide::Short => {
                    init_order.allowance -= amount;
                    // println!("ask {}", init_order.allowance);
                }
                FutureOrderSide::Long => {
                    init_order.allowance -= self.price * amount;
                    // println!("bid {}", init_order.allowance);
                }
            };

            init_order.amount -= amount;
            order.amount -= amount;
            self.size -= amount;
            total_matched_amount += amount;

            matched_orders.push(MatchedOrder::new(order.order_id, order.user_id, amount));

            if order.amount <= 0.0 {
                num_order_removed += 1;
            }

            if init_order.amount <= 0.0
                || (init_order.r#type == OrderType::Market && init_order.allowance <= 0.0)
            {
                break;
            }
        }
        self.orders.drain(0..num_order_removed);

        let init_order = MatchedOrder::new(
            init_order.order_id,
            init_order.user_id,
            total_matched_amount,
        );

        Ok((init_order, matched_orders)) */
    }

    pub fn num_order(&self) -> u32 {
        self.orders.len().try_into().unwrap()
    }

    pub fn size(&self) -> f64 {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.orders.is_empty()
    }
}
