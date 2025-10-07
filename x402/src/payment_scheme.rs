use crate::Authorization;

/// The "exact" scheme uses EIP-3009 (Transfer with Authorization) to enable secure,
/// gasless transfers of specific amounts of ERC-20 tokens.
/// ```js
/// const authorizationTypes = {
///   TransferWithAuthorization: [
///     { name: "from", type: "address" },
///     { name: "to", type: "address" },
///     { name: "value", type: "uint256" },
///     { name: "validAfter", type: "uint256" },
///     { name: "validBefore", type: "uint256" },
///     { name: "nonce", type: "bytes32" },
///   ],
/// };
/// ```
pub enum PaymentScheme {
    Exact,
}

pub struct SchemePayload {
    /// EIP-712 signature for authorization
    signature: String,
    /// EIP-3009 authorization parameters
    authorization: Authorization,
}
