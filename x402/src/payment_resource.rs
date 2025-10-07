use crate::PaymentRequirements;

pub struct ResourceInfo {
    /// The resource URL or identifier being monetized
    resource: String,
    /// Resource type (currently "http" for HTTP endpoints)
    r#type: Option<String>,
    /// Protocol version supported by the resource
    x402_version: u8,
    /// Array of PaymentRequirements objects specifying payment methods
    accepts: Vec<PaymentRequirements>,
    /// Unix timestamp of when the resource was last updated
    last_updated: u64,
    /// Additional metadata (category, provider, etc.)
    metadata: Option<ResourceInfoMetadata>,
}

pub struct ResourceInfoMetadata {
    category: String,
    provider: String,
}
