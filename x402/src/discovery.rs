use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::ResourceInfo;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub struct DiscoveryRequest<'x> {
    /// Filter by resource type (e.g., "http")
    #[serde(borrow)]
    r#type: &'x str,
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
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub struct DiscoveryPayload<'x> {
    x402_version: u8,
    #[serde(borrow)]
    items: Cow<'x, [ResourceInfo<'x>]>,
    pagination: PayloadPagination,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct PayloadPagination {
    limit: u64,
    offset: u64,
    total: u64,
}
