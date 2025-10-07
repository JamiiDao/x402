use crate::{PaymentRequirements, X402Version};

pub struct PaymentRequirementsResponse {
    /// Protocol version identifier
    x402_version: X402Version,
    /// Human-readable error message explaining why payment is required
    error: &'static str,
    /// Array of payment requirement objects defining acceptable payment methods
    accepts: Vec<PaymentRequirements>,
}

impl PaymentRequirementsResponse {
    pub const ERROR: &str = "X-PAYMENT header is required";
    pub fn new() -> Self {
        Self {
            x402_version: X402Version::V1,
            error: Self::ERROR,
            accepts: Vec::default(),
        }
    }
}
