use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub struct Authorization<'x> {
    /// Payer's wallet address
    #[serde(borrow)]
    from: &'x str,
    /// Recipient's wallet address
    #[serde(borrow)]
    to: &'x str,
    /// Payment amount in atomic units
    #[serde(borrow)]
    value: &'x str,
    /// Unix timestamp when authorization becomes valid
    #[serde(borrow)]
    valid_after: &'x str,
    /// Unix timestamp when authorization expires
    #[serde(borrow)]
    valid_before: &'x str,
    /// EIP-3009 Nonce. Each authorization includes a unique 32-byte nonce to prevent replay attacks
    #[serde(borrow)]
    nonce: &'x str,
}

/// The protocol supports integration with authentication systems (e.g., Sign-In with Ethereum - SIWE)
/// to enable authenticated pricing models where verified users receive discounted rates or special access terms.
pub struct Authentication {}
