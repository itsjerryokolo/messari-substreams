// @generated
// <https://github.com/Uniswap/v2-core/blob/master/contracts/UniswapV2Factory.sol>
// <https://github.com/Uniswap/v2-core/blob/master/contracts/UniswapV2Pair.sol>

/// event PairCreated(address indexed token0, address indexed token1, address pair, uint);
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PairCreatedEvent {
    /// bytes: Hash
    #[prost(bytes="vec", tag="1")]
    pub tx_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="2")]
    pub log_index: u32,
    /// bytes: Address
    #[prost(bytes="vec", tag="3")]
    pub token0: ::prost::alloc::vec::Vec<u8>,
    /// bytes: Address
    #[prost(bytes="vec", tag="4")]
    pub token1: ::prost::alloc::vec::Vec<u8>,
    /// bytes: Address
    #[prost(bytes="vec", tag="5")]
    pub pair: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PairCreatedEvents {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<PairCreatedEvent>,
}
/// event Mint(address indexed sender, uint amount0, uint amount1);
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MintEvent {
    /// bytes: Hash
    #[prost(bytes="vec", tag="1")]
    pub tx_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="2")]
    pub log_index: u32,
    /// bytes: Address
    #[prost(bytes="vec", tag="3")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    /// string: BigInt
    #[prost(string, tag="4")]
    pub amount0: ::prost::alloc::string::String,
    /// string: BigInt
    #[prost(string, tag="5")]
    pub amount1: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MintEvents {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<MintEvent>,
}
/// event Burn(address indexed sender, uint amount0, uint amount1, address indexed to);
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BurnEvent {
    /// bytes: Hash
    #[prost(bytes="vec", tag="1")]
    pub tx_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="2")]
    pub log_index: u32,
    /// bytes: Address
    #[prost(bytes="vec", tag="3")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    /// string: BigInt
    #[prost(string, tag="4")]
    pub amount0: ::prost::alloc::string::String,
    /// string: BigInt
    #[prost(string, tag="5")]
    pub amount1: ::prost::alloc::string::String,
    /// bytes: Address
    #[prost(bytes="vec", tag="6")]
    pub to: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BurnEvents {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<BurnEvent>,
}
/// event Swap(address indexed sender, uint amount0In, uint amount1In, uint amount0Out, uint amount1Out, address indexed to);
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapEvent {
    /// bytes: Hash
    #[prost(bytes="vec", tag="1")]
    pub tx_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="2")]
    pub log_index: u32,
    /// bytes: Address
    #[prost(bytes="vec", tag="3")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    /// string: BigInt
    #[prost(string, tag="4")]
    pub amount0_in: ::prost::alloc::string::String,
    /// string: BigInt
    #[prost(string, tag="5")]
    pub amount1_in: ::prost::alloc::string::String,
    /// string: BigInt
    #[prost(string, tag="6")]
    pub amount0_out: ::prost::alloc::string::String,
    /// string: BigInt
    #[prost(string, tag="7")]
    pub amount1_out: ::prost::alloc::string::String,
    /// bytes: Address
    #[prost(bytes="vec", tag="8")]
    pub to: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapEvents {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<SwapEvent>,
}
/// event Sync(uint112 reserve0, uint112 reserve1);
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncEvent {
    /// bytes: Hash
    #[prost(bytes="vec", tag="1")]
    pub tx_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="2")]
    pub log_index: u32,
    /// string: BigInt
    #[prost(string, tag="3")]
    pub reserve0: ::prost::alloc::string::String,
    /// string: BigInt
    #[prost(string, tag="4")]
    pub reserve1: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncEvents {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<SyncEvent>,
}
// @@protoc_insertion_point(module)
