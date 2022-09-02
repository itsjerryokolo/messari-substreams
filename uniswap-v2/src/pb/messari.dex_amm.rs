// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pool {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// bytes: Address
    #[prost(bytes="vec", tag="2")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="3")]
    pub input_tokens: ::prost::alloc::vec::Vec<super::common::Erc20Token>,
    /// Metrics
    ///
    /// string: BigDecimal
    #[prost(string, tag="100")]
    pub total_value_locked: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolToken {
    /// bytes: Address
    #[prost(bytes="vec", tag="1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    /// Balance of input token in native amounts
    ///
    /// string: BigInt
    #[prost(string, tag="2")]
    pub balance: ::prost::alloc::string::String,
    /// Weights of input token in the liquidity pool in percentage values. For example, 0.5/0.5 for Uniswap pools, 0.482/0.518 for a Curve pool, 0.1/0.1/0.8 for a Balancer pool
    #[prost(double, tag="3")]
    pub weight: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pools {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<Pool>,
}
// @@protoc_insertion_point(module)
