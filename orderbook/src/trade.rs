use crate::order_side::OrderSide;
use crate::order_type::OrderType;

#[derive(Debug, Clone)]
pub struct Trade {
    pub init_side: OrderSide,
    pub init_type: OrderType,
    pub trades: Vec<MatchedPair>,
}

impl Trade {
    pub fn new(init_side: OrderSide) -> Self {
        Self {
            init_side,
            init_type: OrderType::Limit,
            trades: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MatchedPair {
    pub price: f64,
    pub init_order: MatchedOrder,
    pub matched_orders: Vec<MatchedOrder>,
}

impl MatchedPair {
    pub fn new(price: f64, init_order: MatchedOrder, matched_orders: Vec<MatchedOrder>) -> Self {
        Self {
            price,
            init_order,
            matched_orders,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MatchedOrder {
    pub order_id: u32,
    pub user_id: u32,
    pub amount: f64,
}

impl MatchedOrder {
    pub fn new(order_id: u32, user_id: u32, amount: f64) -> Self {
        Self {
            order_id,
            user_id,
            amount,
        }
    }
}
