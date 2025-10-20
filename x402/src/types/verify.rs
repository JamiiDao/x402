use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::{PaymentRequirements, XPaymentPayload};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyPayload<'x> {
    #[serde(borrow)]
    pub payment_payload: XPaymentPayload<'x>,
    #[serde(borrow)]
    pub payment_requirements: PaymentRequirements<'x>,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyPayloadResponse<'x> {
    pub is_valid: bool,
    pub invalid_reason: Option<Cow<'x, str>>,
    pub payer: &'x str,
}
