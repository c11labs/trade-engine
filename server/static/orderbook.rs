#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderBook {
    #[prost(message, repeated, tag = "1")]
    pub ask_pairs: ::prost::alloc::vec::Vec<PriceSizePair>,
    #[prost(message, repeated, tag = "2")]
    pub bid_pairs: ::prost::alloc::vec::Vec<PriceSizePair>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceSizePair {
    #[prost(float, tag = "1")]
    pub price: f32,
    #[prost(float, tag = "2")]
    pub size: f32,
}
