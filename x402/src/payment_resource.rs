use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::PaymentRequirements;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub struct ResourceInfo<'x> {
    /// The resource URL or identifier being monetized
    #[serde(borrow)]
    resource: &'x str,
    /// Resource type (currently "http" for HTTP endpoints)
    #[serde(borrow)]
    r#type: Option<&'x str>,
    /// Protocol version supported by the resource
    x402_version: u8,
    /// Array of PaymentRequirements objects specifying payment methods
    #[serde(borrow)]
    accepts: Cow<'x, [PaymentRequirements<'x>]>,
    /// Unix timestamp of when the resource was last updated
    last_updated: u64,
    /// Additional metadata (category, provider, etc.)
    #[serde(borrow)]
    metadata: Option<ResourceInfoMetadata<'x>>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Deserialize, Serialize)]
pub struct ResourceInfoMetadata<'x> {
    #[serde(borrow)]
    category: &'x str,
    #[serde(borrow)]
    provider: &'x str,
}
