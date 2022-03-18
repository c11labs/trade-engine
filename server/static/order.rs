#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Order {
    #[prost(uint32, tag = "1")]
    pub order_id: u32,
    #[prost(uint32, tag = "2")]
    pub user_id: u32,
    #[prost(string, tag = "3")]
    pub pair: ::prost::alloc::string::String,
    #[prost(float, tag = "4")]
    pub price: f32,
    #[prost(float, tag = "5")]
    pub amount: f32,
    #[prost(enumeration = "OrderSide", tag = "6")]
    pub side: i32,
    #[prost(enumeration = "OrderType", tag = "7")]
    pub r#type: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderSide {
    Ask = 0,
    Bid = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderType {
    Limit = 0,
    Market = 1,
}
