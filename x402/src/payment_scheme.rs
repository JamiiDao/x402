use crate::{Authorization, BorrowedStr};

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
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum PaymentScheme {
    Exact,
}

impl PaymentScheme {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Exact => "exact",
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct SchemePayload<'x> {
    /// EIP-712 signature for authorization
    signature: &'x dyn BorrowedStr,
    /// EIP-3009 authorization parameters
    authorization: Authorization<'x>,
}
