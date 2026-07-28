/// Core provider traits and information structs used to define general blockchain interaction interfaces.
pub mod core;
/// Provider-specific error enumerations for handling transmission, retrieval, or interpretation issues.
pub mod error;
/// Types and definitions for interacting specifically with an Esplora REST API provider backend.
#[cfg(not(target_arch = "wasm32"))]
pub mod esplora;
/// Definitions distinguishing blockchain network states (e.g. mainnet, testnet, regtest) and related configurations.
pub mod network;
/// Submodules and definitions handling direct JSON-RPC interfacing with backing Bitcoin/Elements core nodes.
#[cfg(not(target_arch = "wasm32"))]
pub mod rpc;
/// Abstractions and composite providers intended for general usage in the Simplex SDK.
#[cfg(not(target_arch = "wasm32"))]
pub mod simplex;

pub use core::{ProviderInfo, ProviderTrait};
#[cfg(not(target_arch = "wasm32"))]
pub use esplora::EsploraProvider;
#[cfg(not(target_arch = "wasm32"))]
pub use rpc::elements::ElementsRpc;
#[cfg(not(target_arch = "wasm32"))]
pub use simplex::SimplexProvider;

pub use network::*;

pub use error::ProviderError;
#[cfg(not(target_arch = "wasm32"))]
pub use rpc::error::RpcError;
