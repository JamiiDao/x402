use std::borrow::Cow;

use crate::{BorrowedStr, PaymentScheme};

/// GET /supported
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct SupportedBlockchains<'x> {
    kinds: Cow<'x, [BlockchainKind<'x>]>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct BlockchainKind<'x> {
    x402_version: u8,
    scheme: PaymentScheme,
    network: &'x dyn BorrowedStr,
}
