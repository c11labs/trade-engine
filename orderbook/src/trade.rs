use crate::order::MatchedOrder;
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
    pub init_order: MatchedOrder,
    pub matched_orders: Vec<MatchedOrder>,
}

impl MatchedPair {
    pub fn new(init_order: MatchedOrder, matched_orders: Vec<MatchedOrder>) -> Self {
        Self {
            init_order,
            matched_orders,
        }
    }
}
