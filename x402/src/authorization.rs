use crate::BorrowedStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Authorization<'x> {
    /// Payer's wallet address
    from: &'x dyn BorrowedStr,
    /// Recipient's wallet address
    to: &'x dyn BorrowedStr,
    /// Payment amount in atomic units
    value: &'x dyn BorrowedStr,
    /// Unix timestamp when authorization becomes valid
    valid_after: &'x dyn BorrowedStr,
    /// Unix timestamp when authorization expires
    valid_before: &'x dyn BorrowedStr,
    /// EIP-3009 Nonce. Each authorization includes a unique 32-byte nonce to prevent replay attacks
    nonce: &'x dyn BorrowedStr,
}

/// The protocol supports integration with authentication systems (e.g., Sign-In with Ethereum - SIWE)
/// to enable authenticated pricing models where verified users receive discounted rates or special access terms.
pub struct Authentication {}
