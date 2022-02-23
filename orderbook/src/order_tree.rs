use crate::price_level::PriceLevel;
use crate::order::Order;
use crate::price::{AskPrice, BidPrice};
use std::collections::BTreeMap;
use anyhow::{Context, Result};

pub trait Summary {
    fn best_price(&self) -> f32;
    fn worst_price(&self) -> f32;
}

#[derive(Debug)]
pub struct OrderTree {
    tree: BTreeMap<BidPrice, PriceLevel>,
    best_price: BidPrice,
    worst_price: BidPrice,
}

impl Summary for OrderTree {
    fn best_price(&self) -> f32 {
        self.best_price.into_inner()
    }

    fn worst_price(&self) -> f32 {
        self.worst_price.into_inner()
    }
}

impl OrderTree {
    pub fn new() -> Self {
        Self {
            tree: BTreeMap::new(),
            best_price: BidPrice(f32::NEG_INFINITY),
            worst_price: BidPrice(f32::INFINITY),
        }
    }

    pub fn add(&mut self, price: f32) -> Result<()> {
        let limit = PriceLevel::new(price);
        let price = BidPrice(price);
        self.tree.insert(price, limit);

        if self.best_price > price {
            self.best_price = price;
        }

        if self.worst_price < price {
            self.worst_price = price;
        }

        Ok(())
    }

    pub fn add_order(&mut self, price: f32, order: Order) -> Result<()> {
        match self.tree.get_mut(&BidPrice(price)) {
            Some(price_level) => {
                price_level.add(order)?;
            }
            None => {
                self.add(price)?;
                let price_level = self.tree.get_mut(&BidPrice(price)).context("can not add new order")?;
                price_level.add(order)?;
            }
        }
        Ok(())
    }

    pub fn contains_price(&self, price: f32) -> bool {
        self.tree.contains_key(&BidPrice(price))
    }

    pub fn price_list(&self) -> Vec<BidPrice> {
        self.tree.keys().cloned().collect()
    }

    pub fn price_level_size(&self, price: f32) -> Result<u32> {
        let price_level = self.tree.get(&BidPrice(price)).context("key not found")?;
        Ok(price_level.size())
    }

    pub fn price_level_volume(&self, price: f32) -> Result<u32> {
        let price_level = self.tree.get(&BidPrice(price)).context("key not found")?;
        Ok(price_level.volume())
    }
}

#[derive(Debug)]
pub struct AskTree {
    tree: BTreeMap<AskPrice, PriceLevel>,
    pub best_price: AskPrice,
    pub worst_price: AskPrice,
}

impl Summary for AskTree {
    fn best_price(&self) -> f32 {
        self.best_price.into_inner()
    }

    fn worst_price(&self) -> f32 {
        self.worst_price.into_inner()
    }
}

impl AskTree {
    pub fn new() -> Self {
        Self {
            tree: BTreeMap::new(),
            best_price: AskPrice(f32::INFINITY),
            worst_price: AskPrice(f32::NEG_INFINITY),
        }
    }

    pub fn add(&mut self, price: f32) -> Result<()> {
        let limit = PriceLevel::new(price);
        let price = AskPrice(price);
        self.tree.insert(price, limit);

        if self.best_price > price {
            self.best_price = price;
        }

        if self.worst_price < price {
            self.worst_price = price;
        }

        Ok(())
    }

    pub fn contains_price(&self, price: f32) -> bool {
        self.tree.contains_key(&AskPrice(price))
    }

    pub fn price_list(&self) -> Vec<AskPrice> {
        self.tree.keys().cloned().collect()
    }
}
