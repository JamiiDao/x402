use std::borrow::Cow;

use crate::{PaymentRequirements, X402Version};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PaymentRequirementsResponse<'x> {
    /// Protocol version identifier
    x402_version: X402Version,
    /// Human-readable error message explaining why payment is required
    error: &'static str,
    /// Array of payment requirement objects defining acceptable payment methods
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
