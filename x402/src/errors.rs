pub type X402Result<T> = Result<T, X402Error>;

/// The x402 protocol defines standard error codes that may be returned by facilitators or resource servers.
/// These error codes help clients understand why a payment failed and take appropriate action.
pub enum X402Error {
    UnsupportedX402MimeType,
    ///  `insufficient_funds` error. Client does not have enough tokens to complete the payment
    InsufficientFunds,
    /// `invalid_exact_evm_payload_authorization_valid_after` error. Payment authorization is not yet valid (before validAfter timestamp)
    InvalidExactSvmPayloadAuthorizationValidAfter,
    /// `invalid_exact_evm_payload_authorization_valid_before` error. Payment authorization has expired (after validBefore timestamp)
    InvalidExactSvmPayloadAuthorizationValidBefore,
    /// `invalid_exact_evm_payload_authorization_value` error. Payment amount is insufficient for the required payment
    InvalidExactSvmPayloadAuthorizationValue,
    /// `invalid_exact_evm_payload_signature` error. Payment authorization signature is invalid or improperly signed
    InvalidExactSvmPayloadSignature,
    ///  `invalid_exact_evm_payload_recipient_mismatch` error. Recipient address does not match payment requirements
    InvalidExactSvmPayloadRecipientMismatch,
    /// `invalid_network` error. Specified blockchain network is not supported
    InvalidNetwork,
    /// `invalid_payload` error. Payment payload is malformed or contains invalid data
    InvalidPayload,
    /// `invalid_payment_requirements` error. Payment requirements object is invalid or malformed
    InvalidPaymentRequirements,
    /// `invalid_scheme` error. Specified payment scheme is not supported
    InvalidScheme,
    /// `unsupported_scheme` error. Payment scheme is not supported by the facilitator
    UnsupportedScheme,
    /// `invalid_x402_version` error. Protocol version is not supported
    InvalidX402Version,
    /// `invalid_transaction_state` error. Blockchain transaction failed or was rejected
    InvalidTransactionState,
    /// `unexpected_verify_error` error. Unexpected error occurred during payment verification
    UnexpectedVerifyError,
    /// `unexpected_settle_error`: Unexpected error occurred during payment settlement
    UnexpectedSettleError,
    UnsupportedX402Error,
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
