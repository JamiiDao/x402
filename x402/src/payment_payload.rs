use crate::{PaymentScheme, SchemePayload};

pub struct PaymentPayload {
    /// Protocol version identifier (must be 1)
    x402Version: u8,
    /// Payment scheme identifier (e.g., "exact")
    scheme: PaymentScheme,
    /// Blockchain network identifier (e.g., "base-sepolia", "ethereum-mainnet")
    network: String,
    /// Payment data object
    payload: SchemePayload,
}
