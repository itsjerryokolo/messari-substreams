// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc20Tokens {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<Erc20Token>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc20Token {
    /// bytes: Address
    #[prost(bytes="vec", tag="1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub decimals: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    /// bytes: Hash
    #[prost(bytes="vec", tag="1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="2")]
    pub nonce: u64,
    #[prost(uint64, tag="3")]
    pub block_number: u64,
    #[prost(uint64, tag="4")]
    pub block_timestamp: u64,
    /// string: BigInt, in wei
    #[prost(string, tag="5")]
    pub gas_price: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub gas_limit: u64,
}
// @@protoc_insertion_point(module)
