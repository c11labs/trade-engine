pub mod order;
pub mod order_action;
pub mod order_data;
pub mod order_side;
pub mod order_type;
pub mod price_level;

use anyhow::Result;
use price_level::PriceLevel;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct OrderBook {
    instrument_id: u32,
    /* bid: BTreeMap<f32, PriceLevel>,
    ask: BTreeMap<f32, PriceLevel>, */
}

/* impl OrderBook {
    pub fn new(instrument_id: u32) -> Self {
        Self {
            instrument_id,
            bid: BTreeMap::new(),
            ask: BTreeMap::new(),
        }
    }

    pub fn add_price_level(&self, price: f32) -> Result<()> {
        Ok(())
    }

    pub fn bid(&self, shares: u32) -> Result<()> {
        Ok(())
    }

    pub fn ask(&self, shares: u32) -> Result<()> {
        Ok(())
    }

    pub fn best_bid(&self) -> Result<()> {
        Ok(())
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
} */
