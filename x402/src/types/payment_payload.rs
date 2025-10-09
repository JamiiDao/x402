use serde::{Deserialize, Serialize};

use crate::{PaymentScheme, SchemePayload};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub struct PaymentPayload<'x> {
    /// Protocol version identifier (must be 1)
    x402_version: u8,
    /// Payment scheme identifier (e.g., "exact")
    scheme: PaymentScheme,
    /// Blockchain network identifier (e.g., "base-sepolia", "ethereum-mainnet")
    #[serde(borrow)]
    network: &'x str,
    /// Payment data object
    #[serde(borrow)]
    payload: SchemePayload<'x>,
}
