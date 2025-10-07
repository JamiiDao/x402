use crate::ResourceInfo;

pub struct DiscoveryRequest {
    /// Filter by resource type (e.g., "http")
    r#type: String,
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
pub struct DiscoveryPayload {
    x402_version: u8,
    items: Vec<ResourceInfo>,
    pagination: PayloadPagination,
}

pub struct PayloadPagination {
    limit: u64,
    offset: u64,
    total: u64,
}
