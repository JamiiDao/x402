use core::fmt;

use crate::{BorrowedStr, MimeType, OptionalBorrowedStr, PaymentRequestExtras, PaymentScheme};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct PaymentRequirements<'x> {
    /// Payment scheme identifier (e.g., "exact")
    scheme: PaymentScheme,
    /// Blockchain network identifier (e.g., "base-sepolia", "ethereum-mainnet")
    network: &'x dyn BlockchainNetwork,
    /// Required payment amount in atomic token units
    max_amount_required: &'x dyn Number<'x>,
    /// Token contract address
    asset: &'x dyn BorrowedStr,
    /// Recipient wallet address for the payment
    pay_to: &'x dyn BorrowedStr,
    /// URL of the protected resource
    resource: &'x dyn BorrowedStr,
    /// Human-readable description of the resource
    description: &'x dyn BorrowedStr,
    /// MIME type of the expected response
    mime_type: Option<MimeType>,
    /// JSON schema describing the response format
    output_schema: &'x dyn OptionalBorrowedStr,
    /// Maximum time allowed for payment completion
    max_timeout_seconds: &'x dyn Number<'x>,
    /// Scheme-specific additional information
    extra: PaymentRequestExtras<'x>,
}

impl PaymentRequirements<'_> {
    pub fn to_json(&self) -> jzon::JsonValue {
        jzon::object! {
            scheme: self.scheme.as_str(),
            network: self.network.identifier(),
            maxAmountRequired: self.max_amount_required.as_str(),
            asset: self.asset.as_ref(),
            payTo: self.pay_to.as_ref(),
            resource: self.resource.as_ref(),
            description: self.description.as_ref(),
            mimeType: self.mime_type.map(|mime| mime.as_str()),
            outputSchema: self.output_schema.as_ref(),
            maxTimeoutSeconds: self.max_timeout_seconds.as_str(),
            extra: self.extra.to_json(),
        }
    }
}

pub trait BlockchainNetwork {
    fn identifier(&self) -> &'static str;
}

impl PartialEq for &'_ dyn BlockchainNetwork {
    fn eq(&self, other: &Self) -> bool {
        self.identifier().as_bytes() == other.identifier().as_bytes()
    }
}

impl Eq for &'_ dyn BlockchainNetwork {}

impl PartialOrd for &'_ dyn BlockchainNetwork {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for &'_ dyn BlockchainNetwork {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.identifier()
            .as_bytes()
            .cmp(other.identifier().as_bytes())
    }
}

impl fmt::Debug for &'_ dyn BlockchainNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.identifier())
    }
}

pub trait Number<'x> {
    fn u64(&self) -> u64;

    fn as_str(&'x self) -> &'x str;
}

impl<'x> PartialEq for &'x dyn Number<'x> {
    fn eq(&self, other: &Self) -> bool {
        self.u64() == other.u64()
    }
}

impl<'x> Eq for &'x dyn Number<'x> {}

impl<'x> PartialOrd for &'x dyn Number<'x> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'x> Ord for &'x dyn Number<'x> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.u64().cmp(&other.u64())
    }
}

impl<'x> fmt::Debug for &'x dyn Number<'x> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
