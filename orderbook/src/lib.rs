pub mod order;
pub mod order_action;
pub mod order_data;
pub mod order_side;
pub mod order_type;
pub mod price;
pub mod price_level;
pub mod price_tree;

use anyhow::{anyhow, Result};
use order_side::OrderSide;
use price_tree::{Summary, AskTree, BidTree};
use price::{BidPrice, AskPrice};

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
            OrderSide::Ask => {
                self.ask.add(price)?;
            }
        }
        Ok(())
    }

    pub fn bid(&mut self, price: f32, shares: u32) -> Result<()> {
        if self.bid.contains_price(price) {
            return Err(anyhow!("already contain key"));
        } else {
            self.add_price(price, OrderSide::Bid)?;
        }
        Ok(())
    }

    pub fn ask(&mut self, price: f32, shares: u32) -> Result<()> {
        if self.ask.contains_price(price) {
            return Err(anyhow!("already contain key"));
        } else {
            self.add_price(price, OrderSide::Ask)?;
        }
        Ok(())
    }

    pub fn bid_price_list(&self) -> Vec<BidPrice> {
        self.bid.price_list()
    }
    
    pub fn ask_price_list(&self) -> Vec<AskPrice> {
        self.ask.price_list()
    }

    pub fn best_bid(&self) -> f32 {
        self.bid.best_price()
    }

    pub fn worst_bid(&self) -> f32 {
        self.bid.worst_price()
    }

    pub fn best_ask(&self) -> f32 {
        self.ask.best_price()
    }

    pub fn worst_ask(&self) -> f32 {
        self.ask.worst_price()
    }

    pub fn mid_price(&self) -> f32 {
        (self.best_ask() + self.best_bid()) / 2.0
    }

    pub fn bid_ask_spread(&self) -> f32 {
        self.best_ask() - self.best_bid()
    }

    pub fn market_depth(&self) -> f32 {
        self.worst_ask() - self.worst_bid()
    }

    pub fn instrument_id(&self) -> u32 {
        self.instrument_id
    }
}
