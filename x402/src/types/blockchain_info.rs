use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::PaymentScheme;

/// GET /supported
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub struct SupportedBlockchains<'x> {
    #[serde(borrow)]
    kinds: Cow<'x, [BlockchainKind<'x>]>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub struct BlockchainKind<'x> {
    x402_version: u8,
    scheme: PaymentScheme,
    #[serde(borrow)]
    network: &'x str,
}
