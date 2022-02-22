use ordered_float::OrderedFloat;
use crate::price_level::PriceLevel;
use crate::price::{AskPrice, BidPrice};
use std::collections::BTreeMap;
use anyhow::Result;

pub trait Summary {
    fn best_price(&self) -> f32;
    fn worst_price(&self) -> f32;
}

#[derive(Debug)]
pub struct BidTree<'a> {
    tree: BTreeMap<BidPrice, PriceLevel<'a>>,
    best_price: BidPrice,
    worst_price: BidPrice,
}

impl<'a> Summary for BidTree<'a> {
    fn best_price(&self) -> f32 {
        self.best_price.into_inner()
    }

    fn worst_price(&self) -> f32 {
        self.worst_price.into_inner()
    }
}

impl<'a> BidTree<'a> {
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

    pub fn contains_price(&self, price: f32) -> bool {
        self.tree.contains_key(&BidPrice(price))
    }

    pub fn price_list(&self) -> Vec<BidPrice> {
        self.tree.keys().cloned().collect()
    }
}

#[derive(Debug)]
pub struct AskTree<'a> {
    tree: BTreeMap<AskPrice, PriceLevel<'a>>,
    pub best_price: AskPrice,
    pub worst_price: AskPrice,
}

impl<'a> Summary for AskTree<'a> {
    fn best_price(&self) -> f32 {
        self.best_price.into_inner()
    }

    fn worst_price(&self) -> f32 {
        self.worst_price.into_inner()
    }
}

impl<'a> AskTree<'a> {
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
