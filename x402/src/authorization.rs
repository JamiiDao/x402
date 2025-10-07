pub struct Authorization {
    /// Payer's wallet address
    from: String,
    /// Recipient's wallet address
    to: String,
    /// Payment amount in atomic units
    value: String,
    /// Unix timestamp when authorization becomes valid
    valid_after: String,
    /// Unix timestamp when authorization expires
    valid_before: String,
    /// EIP-3009 Nonce. Each authorization includes a unique 32-byte nonce to prevent replay attacks
    nonce: String,
}

/// The protocol supports integration with authentication systems (e.g., Sign-In with Ethereum - SIWE)
/// to enable authenticated pricing models where verified users receive discounted rates or special access terms.
pub struct Authentication {}
