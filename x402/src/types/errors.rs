use serde::{Deserialize, Deserializer, Serializer};

pub type X402Result<T> = Result<T, X402Error>;

/// The x402 protocol defines standard error codes that may be returned by facilitators or resource servers.
/// These error codes help clients understand why a payment failed and take appropriate action.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum X402Error {
    #[error("The mime type is not supported. Only JSON and Binary")]
    UnsupportedX402MimeType,
    ///  `insufficient_funds` error. Client does not have enough tokens to complete the payment
    #[error("The client does not have sufficient tokens to complete the transaction")]
    InsufficientFunds,
    /// `invalid_exact_evm_payload_authorization_valid_after` error. Payment authorization is not yet valid (before validAfter timestamp)
    #[error("Invalid `ValidAfter`")]
    InvalidExactSvmPayloadAuthorizationValidAfter,
    /// `invalid_exact_evm_payload_authorization_valid_before` error. Payment authorization has expired (after validBefore timestamp)
    #[error("Invalid `ValidBefore` value")]
    InvalidExactSvmPayloadAuthorizationValidBefore,
    /// `invalid_exact_evm_payload_authorization_value` error. Payment amount is insufficient for the required payment
    #[error("Invalid autorization value")]
    InvalidExactSvmPayloadAuthorizationValue,
    /// `invalid_exact_evm_payload_signature` error. Payment authorization signature is invalid or improperly signed
    #[error("Invalid payload signature")]
    InvalidExactSvmPayloadSignature,
    ///  `invalid_exact_evm_payload_recipient_mismatch` error. Recipient address does not match payment requirements
    #[error("Recipient mismatch")]
    InvalidExactSvmPayloadRecipientMismatch,
    /// `invalid_network` error. Specified blockchain network is not supported
    #[error("Invalid blockchain network")]
    InvalidNetwork,
    /// `invalid_payload` error. Payment payload is malformed or contains invalid data
    #[error("Invalid payload")]
    InvalidPayload,
    /// `invalid_payment_requirements` error. Payment requirements object is invalid or malformed
    #[error("Invalid payment requirements")]
    InvalidPaymentRequirements,
    /// `invalid_scheme` error. Specified payment scheme is not supported
    #[error("Invalid x402 scheme")]
    InvalidScheme,
    /// `unsupported_scheme` error. Payment scheme is not supported by the facilitator
    #[error("The x402 scheme is not supported")]
    UnsupportedScheme,
    /// `invalid_x402_version` error. Protocol version is not supported
    #[error("Invalid x402 version")]
    InvalidX402Version,
    /// `invalid_transaction_state` error. Blockchain transaction failed or was rejected
    #[error("Invalid transaction state")]
    InvalidTransactionState,
    /// `unexpected_verify_error` error. Unexpected error occurred during payment verification
    #[error("Payment verification error")]
    UnexpectedVerifyError,
    /// `unexpected_settle_error`: Unexpected error occurred during payment settlement
    #[error("`unexpected_settle_error`: Unexpected error occurred during payment settlement")]
    UnexpectedSettleError,
    #[error("Unsupported error")]
    UnsupportedX402Error,
    #[error("The maximum amount required is missing. Unable to build the payment requirements.")]
    MaxAmountIsMissing,
    #[error(
        "The asset required for the transaction is missing. Unable to build the payment requirements."
    )]
    AssetIsMissing,
    #[error("The recipient of the asset is missing. Unable to build the payment requirements.")]
    PayToIsMissing,
    #[error(
        "The resource that requires payment is not set. Unable to build the payment requirements."
    )]
    ResourceIsMissing,
    #[error("A description of the resource is missing. Unable to build the payment requirements.")]
    DescriptionIsMissing,
    #[error("The mime type for the payload is missing. Unable to build the payment requirements.")]
    MimeTypeMissing,
    #[error(
        "The expiry of the transaction request is missing. Unable to build the payment requirements."
    )]
    MaxTimeoutIsMissing,
    #[error(
        "The extra field is missing. At least the feePayer field is required. Unable to build the payment requirements."
    )]
    ExtraIsMissing,
    #[error("The status code received after the `settle` API was called is invalid")]
    UnsupportedX402StatusCodeError,
}

impl TryFrom<&str> for X402Error {
    type Error = X402Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let outcome = match value {
            "insufficient_funds" => Self::InsufficientFunds,
            "invalid_exact_evm_payload_authorization_valid_after" => {
                Self::InvalidExactSvmPayloadAuthorizationValidAfter
            }
            "invalid_exact_evm_payload_authorization_valid_before" => {
                Self::InvalidExactSvmPayloadAuthorizationValidBefore
            }
            "invalid_exact_evm_payload_authorization_value" => {
                Self::InvalidExactSvmPayloadAuthorizationValue
            }
            "invalid_exact_evm_payload_signature" => Self::InvalidExactSvmPayloadSignature,
            "invalid_exact_evm_payload_recipient_mismatch" => {
                Self::InvalidExactSvmPayloadRecipientMismatch
            }
            "invalid_network" => Self::InvalidNetwork,
            "invalid_payload" => Self::InvalidPayload,
            "invalid_payment_requirements" => Self::InvalidPaymentRequirements,
            "invalid_scheme" => Self::InvalidScheme,
            "unsupported_scheme" => Self::UnsupportedScheme,
            "invalid_x402_version" => Self::InvalidX402Version,
            "invalid_transaction_state" => Self::InvalidTransactionState,
            "unexpected_verify_error" => Self::UnexpectedVerifyError,
            "unexpected_settle_error" => Self::UnexpectedSettleError,
            _ => return Err(Self::UnsupportedX402Error),
        };

        Ok(outcome)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub enum X402PaymentErrorStatusCode {
    /// Payment needed to access resource
    #[default]
    PaymentRequired,
    /// Malformed payment payload or requirements
    InvalidPayment,
    /// Payment verification or settlement failed
    PaymentFailed,
    /// Internal server error during payment processing
    ServerError,
}

impl X402PaymentErrorStatusCode {
    pub fn status_code(&self) -> u16 {
        match self {
            Self::PaymentRequired => 402,
            Self::InvalidPayment => 400,
            Self::PaymentFailed => 402,
            Self::ServerError => 500,
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Self::PaymentRequired => "Payment Required",
            Self::InvalidPayment => "Invalid Payment",
            Self::PaymentFailed => "Payment Failed",
            Self::ServerError => "Server Error",
        }
    }

    pub fn info(&self) -> &str {
        match self {
            Self::PaymentRequired => "Payment needed to access resource",
            Self::InvalidPayment => "Malformed payment payload or requirements",
            Self::PaymentFailed => "Payment verification or settlement failed",
            Self::ServerError => "Internal server error during payment processing",
        }
    }
}

impl TryFrom<&str> for X402PaymentErrorStatusCode {
    type Error = X402Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let error_status = match value {
            m if m.contains(Self::PaymentRequired.description()) => Self::PaymentRequired,
            m if m.contains(Self::InvalidPayment.description()) => Self::InvalidPayment,
            m if m.contains(Self::PaymentFailed.description()) => Self::PaymentFailed,
            m if m.contains(Self::ServerError.description()) => Self::ServerError,
            _ => return Err(X402Error::UnsupportedX402StatusCodeError),
        };

        Ok(error_status)
    }
}

pub fn serialize_error_status_code<S>(
    error_status_code: &X402PaymentErrorStatusCode,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_u16(error_status_code.status_code())
}

pub fn deserialize_error_status_code<'de, D>(
    deserializer: D,
) -> Result<X402PaymentErrorStatusCode, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(deserializer)?
        .as_str()
        .try_into()
        .map_err(serde::de::Error::custom)
}
