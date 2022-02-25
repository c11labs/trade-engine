pub mod order;
pub mod order_action;
pub mod order_side;
pub mod order_tree;
pub mod order_type;
pub mod price;
pub mod price_level;

use anyhow::{anyhow, Result};
use order::Order;
use order_side::OrderSide;
use order_tree::OrderTree;
use price::{AskPrice, BidPrice};

#[derive(Debug)]
pub struct OrderBook {
    instrument_id: u32,
    bid: OrderTree<BidPrice>,
    ask: OrderTree<AskPrice>,
}

impl OrderBook {
    pub fn new(instrument_id: u32) -> Self {
        Self {
            instrument_id,
            bid: OrderTree::<BidPrice>::new(),
            ask: OrderTree::<AskPrice>::new(),
        }
    }

    pub fn bid_price_level_volume(&self, price: f32) -> Result<u32> {
        Ok(self.bid.price_level_volume(price)?)
    }

    pub fn bid_price_level_size(&self, price: f32) -> Result<u32> {
        Ok(self.bid.price_level_size(price)?)
    }

    pub fn ask_price_level_volume(&self, price: f32) -> Result<u32> {
        Ok(self.ask.price_level_volume(price)?)
    }

    pub fn ask_price_level_size(&self, price: f32) -> Result<u32> {
        Ok(self.ask.price_level_size(price)?)
    }

    fn m_add_order(&mut self, price: f32, order: Order, side: OrderSide) -> Result<()> {
        match side {
            OrderSide::Bid => {
                self.bid.add_order(price, order)?;
            }
            OrderSide::Ask => {
                self.ask.add_order(price, order)?;
            }
        }
        Ok(())
    }

    pub fn bid(&mut self, price: f32, mut order: Order) -> Result<()> {
        if let Err(_err) = self.match_order(price, &mut order, OrderSide::Bid) {
            
        };
        self.m_add_order(price, order, OrderSide::Bid)?;
        Ok(())
    }

    pub fn ask(&mut self, price: f32, mut order: Order) -> Result<()> {
        if let Err(_err) = self.match_order(price, &mut order, OrderSide::Ask) {

        };
        self.m_add_order(price, order, OrderSide::Ask)?;
        Ok(())
    }

    fn match_order(&mut self, price: f32, order: &mut Order, side: OrderSide) -> Result<()> {
        match side {
            OrderSide::Bid => {
                self.ask.match_order(price, order)?;
            }
            OrderSide::Ask => {
                self.bid.match_order(price, order)?;
            }
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
