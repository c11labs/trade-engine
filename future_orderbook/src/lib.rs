pub mod future_margin;
pub mod future_order;
pub mod future_order_side;
pub mod future_order_tree;
pub mod future_order_type;
pub mod future_pair;
pub mod future_price;
pub mod future_price_level;
pub mod future_trade;

use anyhow::{Context, Result};
use ordered_float::OrderedFloat;

use future_order::FutureOrder;
use future_order_side::FutureOrderSide;
use future_order_tree::FutureOrderTree;
use future_order_type::FutureOrderType;
use future_pair::FuturePriceSizePair;
use future_price::{IntoInner, LongPrice, ShortPrice};
// use trade::{MatchedOrder, MatchedPair, Trade};

fn max(a: f64, b: f64) -> f64 {
    if a > b {
        a
    } else {
        b
    }
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}

type FuturePriceSizePairs = Vec<FuturePriceSizePair>;

#[derive(Debug)]
pub struct FutureOrderBook {
    pair: String,
    long: FutureOrderTree<LongPrice>,
    short: FutureOrderTree<ShortPrice>,
    premium_indices: Vec<f64>,
}

impl FutureOrderBook {
    pub fn new(pair: String) -> Self {
        Self {
            pair,
            long: FutureOrderTree::<LongPrice>::new(),
            short: FutureOrderTree::<ShortPrice>::new(),
            premium_indices: vec![],
        }
    }

    fn m_add_order(&mut self, order: FutureOrder, side: FutureOrderSide) -> Result<()> {
        match side {
            FutureOrderSide::Long => {
                self.long
                    .add_order(order.price.context("no price provided")?, order)?;
            }
            FutureOrderSide::Short => {
                self.short
                    .add_order(order.price.context("no price provided")?, order)?;
            }
        };

        Ok(())
    }

    /* fn m_match_order(
        &mut self,
        price: f64,
        order: &mut Order,
        side: OrderSide,
    ) -> Result<(MatchedOrder, Vec<MatchedOrder>)> {
        let (init_order, matched_orders): (MatchedOrder, Vec<MatchedOrder>) = match side {
            OrderSide::Bid => self.ask.match_order(price, order)?,
            OrderSide::Ask => self.bid.match_order(price, order)?,
        };

        Ok((init_order, matched_orders))
    } */

    fn m_long(&mut self, mut order: FutureOrder) -> Result<()> {
        self.m_add_order(order, FutureOrderSide::Long)?;
        Ok(())
    }

    fn m_short(&mut self, mut order: FutureOrder) -> Result<()> {
        self.m_add_order(order, FutureOrderSide::Short)?;
        Ok(())
    }

    pub fn add(&mut self, order: FutureOrder) -> Result<()> {
        match order.side {
            FutureOrderSide::Long => self.m_long(order),
            FutureOrderSide::Short => self.m_short(order),
        }
    }

    pub fn cancel(&mut self, side: FutureOrderSide, price: f64, order_id: u32) -> Result<()> {
        match side {
            FutureOrderSide::Long => self.long.remove_order(price, order_id)?,
            FutureOrderSide::Short => self.short.remove_order(price, order_id)?,
        };
        Ok(())
    }

    pub fn size(&self, price: f64, side: FutureOrderSide) -> Result<f64> {
        match side {
            FutureOrderSide::Long => Ok(self.long.price_level_size(price)?),
            FutureOrderSide::Short => Ok(self.short.price_level_size(price)?),
        }
    }

    pub fn num_order(&self, price: f64, side: FutureOrderSide) -> Result<u32> {
        match side {
            FutureOrderSide::Long => Ok(self.long.price_level_num_order(price)?),
            FutureOrderSide::Short => Ok(self.short.price_level_num_order(price)?),
        }
    }

    pub fn price_list(&self, side: FutureOrderSide) -> Vec<f64> {
        match side {
            FutureOrderSide::Long => self
                .long
                .price_list()
                .iter_mut()
                .map(|x| x.into_inner())
                .collect(),
            FutureOrderSide::Short => self
                .short
                .price_list()
                .iter_mut()
                .map(|x| x.into_inner())
                .collect(),
        }
    }

    pub fn price_and_size(&self) -> (FuturePriceSizePairs, FuturePriceSizePairs) {
        (self.long.price_and_size(), self.short.price_and_size())
    }

    pub fn pair(&self) -> &str {
        &self.pair
    }

    pub fn impact_price_series(&self, side: FutureOrderSide) -> f64 {
        let impact_margin_notional = 25000.0;
        let price_list: Vec<f64> = self.price_list(side);

        let mut tmp_quote: f64 = 0.0;
        let mut accum_quote_notional_quantity: Vec<f64> = vec![];

        let mut tmp_quant: f64 = 0.0;
        let mut accum_base_quantity: Vec<f64> = vec![];
        for price in &price_list {
            let amount = self.size(*price, side).unwrap();
            tmp_quote += price * amount;
            accum_quote_notional_quantity.push(tmp_quote);

            tmp_quant += amount;
            accum_base_quantity.push(tmp_quant);
        }

        let price = price_list[price_list.len() - 2]; // price at level x - 1
        let base_quant = accum_base_quantity[accum_base_quantity.len() - 2]; // accumulated base quantity at level x - 1
        let notional_quant = accum_quote_notional_quantity[accum_quote_notional_quantity.len() - 2]; // accumulated quote notional quantity at level x - 1

        impact_margin_notional / ((impact_margin_notional - notional_quant) / price + base_quant)
    }

    pub fn calc_premium_index(&mut self) {
        let price_index = 11500.66_f64;
        let impact_bid_price = self.impact_price_series(FutureOrderSide::Long);
        let impact_ask_price = self.impact_price_series(FutureOrderSide::Short);
        let premium_index = (max(0.0_f64, impact_bid_price - price_index)
            - max(0.0_f64, price_index - impact_ask_price))
            / price_index;
        self.premium_indices.push(premium_index);
    }

    pub fn avg_premium_index(&self) -> f64 {
        let mut index = 1.0_f64;
        let mut accum_index = 0.0_f64;
        let mut accum = 0.0_f64;
        for premium_index in &self.premium_indices {
            accum += index as f64 * premium_index;
            accum_index += index;
            index += 1.0;
        }
        accum / accum_index
    }

    pub fn funding_rate(&self) -> f64 {
        let interest_rate = 0.01;
        let avg_premium_index = self.avg_premium_index();
        avg_premium_index + clamp(interest_rate - avg_premium_index, -0.05, 0.05)
    }
}
