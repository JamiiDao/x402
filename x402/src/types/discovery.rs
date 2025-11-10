use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::PaymentRequirements;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub struct ResourceInfo<'x> {
    /// The resource URL or identifier being monetized
    #[serde(borrow)]
    pub resource: &'x str,
    /// Resource type (currently "http" for HTTP endpoints)
    #[serde(borrow)]
    pub r#type: Option<&'x str>,
    /// Protocol version supported by the resource
    pub x402_version: u8,
    /// Array of PaymentRequirements objects specifying payment methods
    #[serde(borrow)]
    pub accepts: Cow<'x, [PaymentRequirements<'x>]>,
    /// Unix timestamp of when the resource was last updated
    pub last_updated: u64,
    /// Additional metadata (category, provider, etc.)
    #[serde(borrow)]
    pub metadata: Option<ResourceInfoMetadata<'x>>,
    #[serde(borrow)]
    pub header_image: Option<Cow<'x, str>>,
    #[serde(borrow)]
    pub title: Option<Cow<'x, str>>,
    #[serde(borrow)]
    pub description: Option<Cow<'x, str>>,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceInfoMetadata<'x> {
    #[serde(borrow)]
    pub category: &'x str,
    #[serde(borrow)]
    pub provider: &'x str,
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_card: Option<&'x str>,
}
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub struct DiscoveryRequest<'x> {
    /// Filter by resource type (e.g., "http")
    #[serde(borrow)]
    pub r#type: &'x str,
    /// Maximum number of results to return (1-100); defaults to `20`
    pub limit: u64,
    /// Number of results to skip for pagination; defaults to `0`
    pub offset: Option<u64>,
}

/// GET /discovery/resources
/// ## Example usage API
/// ### Discover financial data APIs
/// GET /discovery/resources?type=http&limit=10
///
/// ### Search for specific provider
/// GET /discovery/resources?metadata[provider]=Coinbase
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub struct DiscoveryPayload<'x> {
    pub x402_version: u8,
    #[serde(borrow)]
    pub items: Cow<'x, [ResourceInfo<'x>]>,
    pub pagination: PayloadPagination,
}

#[derive(
    Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Serialize, Deserialize,
)]
pub struct PayloadPagination {
    pub limit: Option<u64>,
    pub offset: u64,
    pub total: Option<u64>,
}
