#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Order {
    #[prost(uint32, tag = "1")]
    pub order_id: u32,
    #[prost(uint32, tag = "2")]
    pub user_id: u32,
    #[prost(double, tag = "3")]
    pub allowance: f64,
    #[prost(string, tag = "4")]
    pub pair: ::prost::alloc::string::String,
    #[prost(double, tag = "5")]
    pub price: f64,
    #[prost(double, tag = "6")]
    pub amount: f64,
    #[prost(enumeration = "OrderSide", tag = "7")]
    pub side: i32,
    #[prost(enumeration = "OrderType", tag = "8")]
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
    StopLimit = 2,
    Oco = 3,
}
