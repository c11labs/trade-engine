#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trade {
    #[prost(enumeration = "super::order::OrderSide", tag = "1")]
    pub init_side: i32,
    #[prost(enumeration = "super::order::OrderType", tag = "2")]
    pub init_type: i32,
    #[prost(message, repeated, tag = "3")]
    pub trades: ::prost::alloc::vec::Vec<MatchedPair>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchedPair {
    #[prost(double, tag = "1")]
    pub price: f64,
    #[prost(message, optional, tag = "2")]
    pub init_order: ::core::option::Option<MatchedOrder>,
    #[prost(message, repeated, tag = "3")]
    pub matched_orders: ::prost::alloc::vec::Vec<MatchedOrder>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchedOrder {
    #[prost(uint32, tag = "1")]
    pub order_id: u32,
    #[prost(uint32, tag = "2")]
    pub user_id: u32,
    #[prost(double, tag = "3")]
    pub amount: f64,
}
