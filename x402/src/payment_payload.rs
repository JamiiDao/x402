use crate::{BorrowedStr, PaymentScheme, SchemePayload};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct PaymentPayload<'x> {
    /// Protocol version identifier (must be 1)
    x402_version: u8,
    /// Payment scheme identifier (e.g., "exact")
    scheme: PaymentScheme,
    /// Blockchain network identifier (e.g., "base-sepolia", "ethereum-mainnet")
    network: &'x dyn BorrowedStr,
    /// Payment data object
    payload: SchemePayload<'x>,
}
