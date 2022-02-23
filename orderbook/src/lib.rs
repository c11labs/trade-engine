pub mod order;
pub mod order_action;
pub mod order_data;
pub mod order_side;
pub mod order_type;
pub mod price;
pub mod price_level;
pub mod order_tree;

use anyhow::{anyhow, Result};
use order_side::OrderSide;
use order::Order;
use order_tree::{Summary, AskTree, OrderTree};
use price::{BidPrice, AskPrice};

#[derive(Debug)]
pub struct OrderBook {
    instrument_id: u32,
    bid: OrderTree,
    ask: AskTree,
}

impl OrderBook {
    pub fn new(instrument_id: u32) -> Self {
        Self {
            instrument_id,
            bid: OrderTree::new(),
            ask: AskTree::new(),
        }
    }

    pub fn bid_price_level_volume(&self, price: f32) -> Result<u32> {
        Ok(self.bid.price_level_volume(price)?)
    }

    pub fn bid_price_level_size(&self, price: f32) -> Result<u32> {
        Ok(self.bid.price_level_size(price)?)
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

    fn m_add_order(&mut self, price: f32, order: Order, side: OrderSide) -> Result<()> {
        match side {
            OrderSide::Bid => {
                self.bid.add_order(price, order)?;
            }
            OrderSide::Ask => {

            }
        }
        Ok(())
    }

    pub fn bid(&mut self, price: f32, order: Order) -> Result<()> {
        self.m_add_order(price, order, OrderSide::Bid)?;
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
