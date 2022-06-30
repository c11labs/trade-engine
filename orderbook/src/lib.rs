pub mod order;
pub mod order_side;
pub mod order_tree;
pub mod order_type;
pub mod pair;
pub mod price;
pub mod price_level;
pub mod trade;

use anyhow::{Context, Result};
use order::Order;
use order_side::OrderSide;
use order_tree::OrderTree;
use order_type::OrderType;
use ordered_float::OrderedFloat;
use pair::PriceSizePair;
use price::{AskPrice, BidPrice, IntoInner};
use trade::{MatchedOrder, MatchedPair, Trade};

type PriceSizePairs = Vec<PriceSizePair>;

#[derive(Debug)]
pub struct OrderBook {
    pair: String,
    bid: OrderTree<BidPrice>,
    ask: OrderTree<AskPrice>,
}

impl OrderBook {
    pub fn new(pair: String) -> Self {
        Self {
            pair,
            bid: OrderTree::<BidPrice>::new(),
            ask: OrderTree::<AskPrice>::new(),
        }
    }

    fn m_add_order(&mut self, order: Order, side: OrderSide) -> Result<()> {
        match side {
            OrderSide::Bid => {
                self.bid
                    .add_order(order.price.context("no price provided")?, order)?;
            }
            OrderSide::Ask => {
                self.ask
                    .add_order(order.price.context("no price provided")?, order)?;
            }
        };

        Ok(())
    }

    fn m_match_order(
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
    }

    fn m_bid(&mut self, mut order: Order) -> Result<Option<Trade>> {
        let price_list: Vec<AskPrice> = self.ask.price_list();
        let mut trade: Trade = Trade::new(OrderSide::Bid);
        let mut matched = false;
        match order.r#type {
            OrderType::Limit => {
                for ask_price in price_list.iter() {
                    if order.amount == 0.0 {
                        break;
                    }
                    if OrderedFloat(ask_price.into_inner())
                        > OrderedFloat(order.price.context("no price provided")?)
                    {
                        break;
                    }
                    let (init_order, matched_orders): (MatchedOrder, Vec<MatchedOrder>) =
                        self.m_match_order(ask_price.into_inner(), &mut order, OrderSide::Bid)?;
                    trade.init_type = OrderType::Limit;
                    trade.trades.push(MatchedPair::new(
                        ask_price.into_inner(),
                        init_order,
                        matched_orders,
                    ));
                    matched = true;
                }

                if order.amount > 0.0 {
                    self.m_add_order(order, OrderSide::Bid)?;
                }
            }
            OrderType::Market => {
                for ask_price in price_list.iter() {
                    if order.amount == 0.0 || order.allowance == 0.0 {
                        break;
                    }
                    let (init_order, matched_orders): (MatchedOrder, Vec<MatchedOrder>) =
                        self.m_match_order(ask_price.into_inner(), &mut order, OrderSide::Bid)?;
                    trade.init_type = OrderType::Market;
                    trade.trades.push(MatchedPair::new(
                        ask_price.into_inner(),
                        init_order,
                        matched_orders,
                    ));
                    matched = true;
                }
            }
            OrderType::StopLoss => {}
        };

        if matched {
            return Ok(Some(trade));
        }

        Ok(None)
    }

    fn m_ask(&mut self, mut order: Order) -> Result<Option<Trade>> {
        let price_list: Vec<BidPrice> = self.bid.price_list();
        let mut trade: Trade = Trade::new(OrderSide::Ask);
        let mut matched = false;
        match order.r#type {
            OrderType::Limit => {
                for bid_price in price_list.iter() {
                    if order.amount == 0.0 {
                        break;
                    }
                    if OrderedFloat(bid_price.into_inner())
                        < OrderedFloat(order.price.context("no price provided")?)
                    {
                        break;
                    }
                    let (init_order, matched_orders): (MatchedOrder, Vec<MatchedOrder>) =
                        self.m_match_order(bid_price.into_inner(), &mut order, OrderSide::Ask)?;
                    trade.init_type = OrderType::Limit;
                    trade.trades.push(MatchedPair::new(
                        bid_price.into_inner(),
                        init_order,
                        matched_orders,
                    ));
                    matched = true;
                }

                if order.amount > 0.0 {
                    self.m_add_order(order, OrderSide::Ask)?;
                }
            }
            OrderType::Market => {
                for bid_price in price_list.iter() {
                    if order.amount == 0.0 || order.allowance == 0.0 {
                        break;
                    }
                    let (init_order, matched_orders): (MatchedOrder, Vec<MatchedOrder>) =
                        self.m_match_order(bid_price.into_inner(), &mut order, OrderSide::Ask)?;
                    trade.init_type = OrderType::Market;
                    trade.trades.push(MatchedPair::new(
                        bid_price.into_inner(),
                        init_order,
                        matched_orders,
                    ));
                    matched = true;
                }
            }
            OrderType::StopLoss => {}
        };

        if matched {
            return Ok(Some(trade));
        }

        Ok(None)
    }

    pub fn add(&mut self, order: Order) -> Result<Option<Trade>> {
        match order.side {
            OrderSide::Ask => self.m_ask(order),
            OrderSide::Bid => self.m_bid(order),
        }
    }

    pub fn cancel(&mut self, side: OrderSide, price: f64, order_id: u32) -> Result<()> {
        match side {
            OrderSide::Ask => self.ask.remove_order(price, order_id)?,
            OrderSide::Bid => self.bid.remove_order(price, order_id)?,
        };
        Ok(())
    }

    pub fn modify(
        &mut self,
        mut order: Order,
        new_price: Option<f64>,
        new_amount: Option<f64>,
    ) -> Result<Option<Trade>> {
        if let Ok(()) = self.cancel(
            order.side,
            order.price.context("no price provided")?,
            order.order_id,
        ) {
            if let Some(price) = new_price {
                order.price = Some(price);
            };
            if let Some(amount) = new_amount {
                order.amount = amount;
            };
            return self.add(order);
        };

        Ok(None)
    }

    pub fn size(&self, price: f64, side: OrderSide) -> Result<f64> {
        match side {
            OrderSide::Ask => Ok(self.ask.price_level_size(price)?),
            OrderSide::Bid => Ok(self.bid.price_level_size(price)?),
        }
    }

    pub fn num_order(&self, price: f64, side: OrderSide) -> Result<u32> {
        match side {
            OrderSide::Ask => Ok(self.ask.price_level_num_order(price)?),
            OrderSide::Bid => Ok(self.bid.price_level_num_order(price)?),
        }
    }

    pub fn price_list(&self, side: OrderSide) -> Vec<f64> {
        match side {
            OrderSide::Ask => self
                .ask
                .price_list()
                .iter_mut()
                .map(|x| x.into_inner())
                .collect(),
            OrderSide::Bid => self
                .bid
                .price_list()
                .iter_mut()
                .map(|x| x.into_inner())
                .collect(),
        }
    }

    pub fn price_and_size(&self) -> (PriceSizePairs, PriceSizePairs) {
        (self.ask.price_and_size(), self.bid.price_and_size())
    }

    pub fn pair(&self) -> &str {
        &self.pair
    }
}
