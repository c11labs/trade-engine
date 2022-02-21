/* use crate::order::{Order, OrderTrait};
use crate::order_data::OrderData;
use crate::order_side::OrderSide;
use anyhow::Result;

pub enum OrderAction {
    NewOrder {
        data: OrderData,
        price: f32,
        quantity: u32,
        side: OrderSide,
    },
    ModifyOrder {
        order_id: u32,
        new_price: f32,
        new_quantity: u32,
        side: OrderSide,
    },
    CancelOrder {
        order_id: u32,
    },
}

impl OrderAction {
    pub fn process(&self) -> Result<()> {
        match self {
            OrderAction::NewOrder {
                price,
                quantity,
                side,
                ..
            } => {
                /* let data = OrderData {
                    ..Default::default()
                };
                let new_order = Order::new(data, *price, *quantity, *side);
                println!("{new_order:?}"); */
            }
            OrderAction::ModifyOrder {
                order_id,
                new_price,
                new_quantity,
                side,
            } => {
                /* let order: Order = Order::get(*order_id)?;
                Order::cancel(*order_id)?;
                let modified_order = Order::new(order.data, *new_price, *new_quantity, *side);
                println!("{modified_order:?}"); */
            }
            OrderAction::CancelOrder { order_id } => {
                // Order::cancel(*order_id)?;
            }
        }

        Ok(())
    }
} */
