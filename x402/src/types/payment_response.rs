use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::{PaymentRequirements, X402Version};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentRequirementsResponse<'x> {
    /// Protocol version identifier
    x402_version: X402Version,
    /// Human-readable error message explaining why payment is required
    #[serde(borrow)]
    error: &'x str,
    /// Array of payment requirement objects defining acceptable payment methods
    #[serde(borrow)]
    accepts: Cow<'x, [PaymentRequirements<'x>]>,
}

impl<'x> PaymentRequirementsResponse<'x> {
    pub const ERROR: &'static str = "X-PAYMENT header is required";

    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_payment_requirement(&mut self, value: PaymentRequirements<'x>) -> &mut Self {
        self.accepts.to_mut().push(value);

        self
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn x402_version(&self) -> X402Version {
        self.x402_version
    }

    pub fn error(&self) -> &str {
        self.error
    }

    pub fn accepts(&self) -> &[PaymentRequirements<'_>] {
        self.accepts.as_ref()
    }
}

impl<'x> Default for PaymentRequirementsResponse<'x> {
    fn default() -> Self {
        Self {
            x402_version: X402Version::V1,
            error: Self::ERROR,
            accepts: Cow::Borrowed(&[]),
        }
    }
}
