pub mod order;
pub mod order_action;
pub mod order_data;
pub mod order_side;
pub mod order_type;
pub mod price;
pub mod price_level;

use anyhow::{anyhow, Result};
use order_side::OrderSide;
use ordered_float::OrderedFloat;
use price_level::PriceLevel;
use std::collections::BTreeMap;

#[derive(Debug)]
struct BidTree<'a> {
    tree: BTreeMap<OrderedFloat<f32>, PriceLevel<'a>>,
    best_price: OrderedFloat<f32>,
    worst_price: OrderedFloat<f32>,
}

impl<'a> BidTree<'a> {
    pub fn new() -> Self {
        Self {
            tree: BTreeMap::new(),
            best_price: OrderedFloat(0.0),
            worst_price: OrderedFloat(10000000000000000.0),
        }
    }

    pub fn add(&mut self, price: f32) -> Result<()> {
        let limit = PriceLevel::new(price);
        let price = OrderedFloat(price);
        self.tree.insert(price, limit);

        if self.best_price < price {
            self.best_price = price;
        }

        if self.worst_price > price {
            self.worst_price = price;
        }

        Ok(())
    }

    pub fn contains_price(&self, price: f32) -> bool {
        self.tree.contains_key(&OrderedFloat(price))
    }

    pub fn price_list(&self) -> Vec<OrderedFloat<f32>> {
        self.tree.keys().cloned().collect()
    }

    pub fn best_price(&self) -> f32 {
        self.best_price.into_inner()
    }

    pub fn worst_price(&self) -> f32 {
        self.worst_price.into_inner()
    }
}

#[derive(Debug)]
struct AskTree<'a> {
    tree: BTreeMap<OrderedFloat<f32>, PriceLevel<'a>>,
    best_price: f32,
    worst_price: f32,
}

impl<'a> AskTree<'a> {
    pub fn new() -> Self {
        Self {
            tree: BTreeMap::new(),
            best_price: 0.0,
            worst_price: 0.0,
        }
    }
}

#[derive(Debug)]
pub struct OrderBook<'a> {
    instrument_id: u32,
    bid: BidTree<'a>,
    ask: AskTree<'a>,
}

impl<'a> OrderBook<'a> {
    pub fn new(instrument_id: u32) -> Self {
        Self {
            instrument_id,
            bid: BidTree::new(),
            ask: AskTree::new(),
        }
    }

    pub fn add_price(&mut self, price: f32, side: OrderSide) -> Result<()> {
        match side {
            OrderSide::Bid => {
                self.bid.add(price)?;
            }
            OrderSide::Ask => {}
        }
        Ok(())
    }

    pub fn bid(&mut self, price: f32, shares: u32) -> Result<()> {
        if self.bid.contains_price(price) {
            return Err(anyhow!("already contain key"));
        } else {
            self.add_price(price, OrderSide::Bid)?;
            // self.bid.
        }
        Ok(())
    }

    pub fn ask(&self, price: f32, shares: u32) -> Result<()> {
        Ok(())
    }

    pub fn bid_price_list(&self) -> Result<Vec<OrderedFloat<f32>>> {
        Ok(self.bid.price_list())
    }

    pub fn best_bid(&self) -> Result<f32> {
        let price = self.bid.best_price();
        Ok(price)
    }

    pub fn worst_bid(&self) -> Result<f32> {
        let price = self.bid.worst_price();
        Ok(price)
    }

    pub fn best_ask(&self) -> Result<()> {
        Ok(())
    }

    pub fn mid_price(&self) -> Result<()> {
        Ok(())
    }

    pub fn bid_ask_spread(&self) -> Result<()> {
        Ok(())
    }

    pub fn market_depth(&self) -> Result<()> {
        Ok(())
    }

    pub fn instrument_id(&self) -> u32 {
        self.instrument_id
    }
}
