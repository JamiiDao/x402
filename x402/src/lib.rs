pub struct PaymentRequirementsResponse {
    /// Protocol version identifier
    x402_version: X402Version,
    /// Human-readable error message explaining why payment is required
    error: String,
    /// Array of payment requirement objects defining acceptable payment methods
    accepts: Vec<PaymentRequirements>,
}

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

pub struct PaymentRequestExtras {
    name: String,
    version: String,
    feePayer: String,
}

/// The "exact" scheme uses EIP-3009 (Transfer with Authorization) to enable secure,
/// gasless transfers of specific amounts of ERC-20 tokens.
/// ```js
/// const authorizationTypes = {
///   TransferWithAuthorization: [
///     { name: "from", type: "address" },
///     { name: "to", type: "address" },
///     { name: "value", type: "uint256" },
///     { name: "validAfter", type: "uint256" },
///     { name: "validBefore", type: "uint256" },
///     { name: "nonce", type: "bytes32" },
///   ],
/// };
/// ```
pub enum PaymentScheme {
    Exact,
}

pub enum MimeType {
    Json,
    Binary,
}

impl TryFrom<&str> for MimeType {
    type Error = X402Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let outcome = match value {
            "application/json" => Self::Json,
            "application/octet-stream" => Self::Binary,
            _ => return Err(X402Error::UnsupportedX402MimeType),
        };

        Ok(outcome)
    }
}

pub enum X402Version {
    V1,
}

impl TryFrom<u8> for X402Version {
    type Error = X402Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::V1),
            _ => Err(X402Error::InvalidX402Version),
        }
    }
}

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

pub struct PaymentPayload {
    /// Protocol version identifier (must be 1)
    x402Version: u8,
    /// Payment scheme identifier (e.g., "exact")
    scheme: PaymentScheme,
    /// Blockchain network identifier (e.g., "base-sepolia", "ethereum-mainnet")
    network: String,
    /// Payment data object
    payload: SchemePayload,
}

pub struct SchemePayload {
    /// EIP-712 signature for authorization
    signature: String,
    /// EIP-3009 authorization parameters
    authorization: Authorization,
}

pub struct Authorization {
    /// Payer's wallet address
    from: String,
    /// Recipient's wallet address
    to: String,
    /// Payment amount in atomic units
    value: String,
    /// Unix timestamp when authorization becomes valid
    valid_after: String,
    /// Unix timestamp when authorization expires
    valid_before: String,
    /// EIP-3009 Nonce. Each authorization includes a unique 32-byte nonce to prevent replay attacks
    nonce: String,
}

pub struct SettlementResponse {
    /// Indicates whether the payment settlement was successful
    success: bool,
    /// Error reason if settlement failed (omitted if successful)
    error_reason: Option<String>,
    /// Blockchain transaction hash (empty string if settlement failed)
    transaction: Option<String>,
    /// Blockchain network identifier
    network: String,
    /// Address of the payer's wallet
    payer: String,
}

/// GET /supported
pub struct SupportedBlockchains {
    kinds: Vec<BlockchainKind>,
}

pub struct BlockchainKind {
    x402Version: u8,
    scheme: PaymentScheme,
    network: String,
}

pub struct DiscoveryRequest {
    /// Filter by resource type (e.g., "http")
    r#type: String,
    /// Maximum number of results to return (1-100); defaults to `20`
    limit: u64,
    /// Number of results to skip for pagination; defaults to `0`
    offset: Option<u64>,
}

/// GET /discovery/resources
/// ## Example usage API
/// ### Discover financial data APIs
/// GET /discovery/resources?type=http&limit=10
///
/// ### Search for specific provider
/// GET /discovery/resources?metadata[provider]=Coinbase
pub struct DiscoveryPayload {
    x402_version: u8,
    items: Vec<ResourceInfo>,
    pagination: PayloadPagination,
}

pub struct PayloadPagination {
    limit: u64,
    offset: u64,
    total: u64,
}

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

/// The protocol supports integration with authentication systems (e.g., Sign-In with Ethereum - SIWE)
/// to enable authenticated pricing models where verified users receive discounted rates or special access terms.
pub struct Authentication {}
