use serde::{Deserialize, Serialize};

use crate::{
    PaymentScheme, SchemePayload, X402SolanaNetworkInfo, deserialize_network, serialize_network,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub struct PaymentPayload<'x> {
    /// Protocol version identifier (must be 1)
    x402_version: u8,
    /// Payment scheme identifier (e.g., "exact")
    scheme: PaymentScheme,
    /// Blockchain network identifier (solana-devnet, solana-mainnet, solana-testnet, solana-localnet)
    #[serde(deserialize_with = "deserialize_network")]
    #[serde(serialize_with = "serialize_network")]
    network: X402SolanaNetworkInfo,
    /// Payment data object
    #[serde(borrow)]
    payload: SchemePayload<'x>,
}
