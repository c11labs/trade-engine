use crate::future_order::FutureOrder;
use crate::future_pair::FuturePriceSizePair;
use crate::future_price::{IntoInner, LongPrice, ShortPrice};
use crate::future_price_level::FuturePriceLevel;
// use crate::trade::MatchedOrder;
use anyhow::{bail, Context, Result};
use std::collections::BTreeMap;

impl From<f64> for LongPrice {
    fn from(price: f64) -> Self {
        Self(price)
    }
}

impl From<f64> for ShortPrice {
    fn from(price: f64) -> Self {
        Self(price)
    }
}

#[derive(Debug)]
pub struct FutureOrderTree<T> {
    tree: BTreeMap<T, FuturePriceLevel>,
    price_list: Vec<T>,
}

impl FutureOrderTree<LongPrice> {
    pub fn new() -> Self {
        Self {
            tree: BTreeMap::new(),
            price_list: Vec::new(),
        }
    }
}

impl FutureOrderTree<ShortPrice> {
    pub fn new() -> Self {
        Self {
            tree: BTreeMap::new(),
            price_list: Vec::new(),
        }
    }
}

impl<T: IntoInner + PartialEq + PartialOrd + Ord + Clone + Copy + From<f64>> FutureOrderTree<T> {
    fn m_add_price_level(&mut self, price: f64) -> Result<()> {
        let limit = FuturePriceLevel::new(price);
        let price = T::from(price);
        self.tree.insert(price, limit);

        Ok(())
    }

    pub fn add_order(&mut self, price: f64, order: FutureOrder) -> Result<()> {
        match self.tree.get_mut(&T::from(price)) {
            Some(price_level) => {
                price_level.add(order)?;
            }
            None => {
                self.m_add_price_level(price)?;
                self.price_list = self.tree.keys().cloned().collect();
                let price_level = self
                    .tree
                    .get_mut(&T::from(price))
                    .context("can not add new order")?;
                price_level.add(order)?;
            }
        }
        Ok(())
    }

    pub fn remove_order(&mut self, price: f64, order_id: u32) -> Result<()> {
        match self.tree.get_mut(&T::from(price)) {
            Some(price_level) => {
                price_level.remove(order_id)?;

                if price_level.is_empty() {
                    self.tree.remove(&T::from(price));
                    self.price_list = self.tree.keys().cloned().collect();
                }

                Ok(())
            }
            None => bail!("price not found"),
        }
    }

    /* pub fn match_order(
        &mut self,
        price: f64,
        order: &mut Order,
    ) -> Result<(MatchedOrder, Vec<MatchedOrder>)> {
        let price_level = self
            .tree
            .get_mut(&T::from(price))
            .context("price not found")?;
        let (init_order, matched_orders): (MatchedOrder, Vec<MatchedOrder>) =
            price_level.match_order(order)?;

        if price_level.is_empty() {
            self.tree.remove(&T::from(price));
            self.price_list = self.tree.keys().cloned().collect();
        }

        Ok((init_order, matched_orders))
    } */

    pub fn contains_price(&self, price: f64) -> bool {
        self.tree.contains_key(&T::from(price))
    }

    pub fn price_list(&self) -> Vec<T> {
        self.price_list.clone()
    }

    pub fn price_level_num_order(&self, price: f64) -> Result<u32> {
        let price_level = self.tree.get(&T::from(price)).context("price not found")?;
        Ok(price_level.num_order())
    }

    pub fn price_level_size(&self, price: f64) -> Result<f64> {
        let price_level = self.tree.get(&T::from(price)).context("price not found")?;
        Ok(price_level.size())
    }

    pub fn price_and_size(&self) -> Vec<FuturePriceSizePair> {
        let mut pairs: Vec<FuturePriceSizePair> = Vec::new();
        for (price, price_level) in self.tree.iter() {
            let price = price.into_inner();
            let size = price_level.size();

            pairs.push(FuturePriceSizePair { price, size });
        }

        pairs
    }
}
