use crate::{MimeType, PaymentRequestExtras, PaymentScheme};

pub struct PaymentRequirements {
    /// Payment scheme identifier (e.g., "exact")
    scheme: PaymentScheme,
    /// Blockchain network identifier (e.g., "base-sepolia", "ethereum-mainnet")
    network: String,
    /// Required payment amount in atomic token units
    max_amount_required: String,
    /// Token contract address
    asset: String,
    /// Recipient wallet address for the payment
    pay_to: String,
    /// URL of the protected resource
    resource: String,
    /// Human-readable description of the resource
    description: String,
    /// MIME type of the expected response
    mime_type: Option<MimeType>,
    /// JSON schema describing the response format
    output_schema: Option<String>,
    /// Maximum time allowed for payment completion
    max_timeout_seconds: u64,
    /// Scheme-specific additional information
    extra: PaymentRequestExtras,
}
