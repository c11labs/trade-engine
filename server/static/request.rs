#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Add {
    #[prost(message, optional, tag = "1")]
    pub order: ::core::option::Option<super::order::Order>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cancel {
    #[prost(enumeration = "super::order::OrderSide", tag = "1")]
    pub side: i32,
    #[prost(float, tag = "2")]
    pub price: f32,
    #[prost(uint32, tag = "3")]
    pub order_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Modify {
    #[prost(message, optional, tag = "1")]
    pub order: ::core::option::Option<super::order::Order>,
    #[prost(float, tag = "2")]
    pub new_price: f32,
    #[prost(float, tag = "3")]
    pub new_amount: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(oneof = "request::Command", tags = "1, 2, 3")]
    pub command: ::core::option::Option<request::Command>,
}
/// Nested message and enum types in `Request`.
pub mod request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Command {
        #[prost(message, tag = "1")]
        Add(super::Add),
        #[prost(message, tag = "2")]
        Cancel(super::Cancel),
        #[prost(message, tag = "3")]
        Modify(super::Modify),
    }
}
