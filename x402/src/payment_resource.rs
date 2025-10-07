use std::borrow::Cow;

use crate::{BorrowedStr, PaymentRequirements};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct ResourceInfo<'x> {
    /// The resource URL or identifier being monetized
    resource: &'x dyn BorrowedStr,
    /// Resource type (currently "http" for HTTP endpoints)
    r#type: Option<&'x dyn BorrowedStr>,
    /// Protocol version supported by the resource
    x402_version: u8,
    /// Array of PaymentRequirements objects specifying payment methods
    accepts: Cow<'x, [PaymentRequirements<'x>]>,
    /// Unix timestamp of when the resource was last updated
    last_updated: u64,
    /// Additional metadata (category, provider, etc.)
    metadata: Option<ResourceInfoMetadata<'x>>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct ResourceInfoMetadata<'x> {
    category: &'x dyn BorrowedStr,
    provider: &'x dyn BorrowedStr,
}
