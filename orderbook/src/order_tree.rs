use crate::order::Order;
use crate::price::{AskPrice, BidPrice, IntoInner};
use crate::price_level::PriceLevel;
use anyhow::{Context, Result};
use std::collections::BTreeMap;

impl From<f32> for BidPrice {
    fn from(price: f32) -> Self {
        Self(price)
    }
}

impl From<f32> for AskPrice {
    fn from(price: f32) -> Self {
        Self(price)
    }
}

pub trait Summary {
    fn best_price(&self) -> f32;
    fn worst_price(&self) -> f32;
}

#[derive(Debug)]
pub struct OrderTree<T> {
    tree: BTreeMap<T, PriceLevel>,
    best_price: T,
    worst_price: T,
}

impl OrderTree<BidPrice> {
    pub fn new() -> Self {
        Self {
            tree: BTreeMap::new(),
            best_price: BidPrice(f32::NEG_INFINITY),
            worst_price: BidPrice(f32::INFINITY),
        }
    }
}

impl OrderTree<AskPrice> {
    pub fn new() -> Self {
        Self {
            tree: BTreeMap::new(),
            best_price: AskPrice(f32::INFINITY),
            worst_price: AskPrice(f32::NEG_INFINITY),
        }
    }
}

impl<T: IntoInner + PartialEq + PartialOrd + Ord + Clone + Copy + From<f32>> OrderTree<T> {
    pub fn add(&mut self, price: f32) -> Result<()> {
        let limit = PriceLevel::new(price);
        let price = T::from(price);
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
        match self.tree.get_mut(&T::from(price)) {
            Some(price_level) => {
                price_level.add(order)?;
            }
            None => {
                self.add(price)?;
                let price_level = self
                    .tree
                    .get_mut(&T::from(price))
                    .context("can not add new order")?;
                price_level.add(order)?;
            }
        }
        Ok(())
    }

    pub fn contains_price(&self, price: f32) -> bool {
        self.tree.contains_key(&T::from(price))
    }

    pub fn price_list(&self) -> Vec<T> {
        self.tree.keys().cloned().collect()
    }

    pub fn price_level_size(&self, price: f32) -> Result<u32> {
        let price_level = self.tree.get(&T::from(price)).context("key not found")?;
        Ok(price_level.size())
    }

    pub fn price_level_volume(&self, price: f32) -> Result<u32> {
        let price_level = self.tree.get(&T::from(price)).context("key not found")?;
        Ok(price_level.volume())
    }

    pub fn best_price(&self) -> f32 {
        self.best_price.into_inner()
    }

    pub fn worst_price(&self) -> f32 {
        self.worst_price.into_inner()
    }
}
