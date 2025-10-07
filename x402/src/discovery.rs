use std::borrow::Cow;

use crate::{BorrowedStr, ResourceInfo};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct DiscoveryRequest<'x> {
    /// Filter by resource type (e.g., "http")
    r#type: &'x dyn BorrowedStr,
    /// Maximum number of results to return (1-100); defaults to `20`
    limit: u64,
    /// Number of results to skip for pagination; defaults to `0`
    offset: Option<u64>,
}

/// GET /discovery/resources
/// ## Example usage API
/// ### Discover financial data APIs
/// GET /discovery/resources?type=http&limit=10
///
/// ### Search for specific provider
/// GET /discovery/resources?metadata[provider]=Coinbase
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct DiscoveryPayload<'x> {
    x402_version: u8,
    items: Cow<'x, [ResourceInfo<'x>]>,
    pagination: PayloadPagination,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct PayloadPagination {
    limit: u64,
    offset: u64,
    total: u64,
}
