use core::{fmt, time::Duration};
use std::borrow::Cow;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    MimeType, PaymentRequestExtras, PaymentScheme, X402Error, X402Result, deserialize_mime,
    serialize_mime,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentRequirements<'x> {
    /// Payment scheme identifier (e.g., "exact")
    scheme: PaymentScheme,
    /// Blockchain network identifier (e.g., "base-sepolia", "ethereum-mainnet")
    #[serde(serialize_with = "serialize_network")]
    #[serde(deserialize_with = "deserialize_network")]
    network: X402SolanaNetworkInfo,
    /// Required payment amount in atomic token units
    max_amount_required: u64,
    /// Token contract address
    #[serde(borrow)]
    asset: &'x str,
    /// Recipient wallet address for the payment
    #[serde(borrow)]
    pay_to: &'x str,
    /// URL of the protected resource
    #[serde(borrow)]
    resource: &'x str,
    /// Human-readable description of the resource
    #[serde(borrow)]
    description: &'x str,
    /// MIME type of the expected response
    #[serde(serialize_with = "serialize_mime")]
    #[serde(deserialize_with = "deserialize_mime")]
    mime_type: Option<MimeType>,
    /// Optional JSON schema describing the response format
    #[serde(borrow)]
    output_schema: Option<Cow<'x, str>>,
    /// Maximum time allowed for payment completion
    max_timeout_seconds: u64,
    /// Scheme-specific additional information
    #[serde(borrow)]
    extra: PaymentRequestExtras<'x>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PaymentRequirementsBuilder<'x> {
    /// Payment scheme identifier (e.g., "exact")
    scheme: PaymentScheme,
    /// Blockchain network identifier (e.g., "base-sepolia", "ethereum-mainnet")
    network: X402SolanaNetworkInfo,
    /// Required payment amount in atomic token units
    max_amount_required: Option<u64>,
    /// Token contract address
    asset: Option<&'x str>,
    /// Recipient wallet address for the payment
    pay_to: Option<&'x str>,
    /// URL of the protected resource
    resource: Option<&'x str>,
    /// Human-readable description of the resource
    description: Option<&'x str>,
    /// MIME type of the expected response
    mime_type: Option<MimeType>,
    /// Optional JSON schema describing the response format
    output_schema: Option<Cow<'x, str>>,
    /// Maximum time allowed for payment completion
    max_timeout_seconds: Option<u64>,
    /// Scheme-specific additional information
    extra: Option<PaymentRequestExtras<'x>>,
}

impl<'x> PaymentRequirementsBuilder<'x> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn use_mainnet(&mut self) -> &mut Self {
        self.network = X402SolanaNetworkInfo::Mainnet;

        self
    }

    pub fn use_testnet(&mut self) -> &mut Self {
        self.network = X402SolanaNetworkInfo::Testnet;

        self
    }

    pub fn use_devnet(&mut self) -> &mut Self {
        self.network = X402SolanaNetworkInfo::Devnet;

        self
    }

    pub fn use_localhost(&mut self) -> &mut Self {
        self.network = X402SolanaNetworkInfo::Localnet;

        self
    }

    pub fn set_amount(&mut self, amount: u64) -> &mut Self {
        self.max_amount_required.replace(amount);

        self
    }

    pub fn set_asset(&mut self, asset: &'x str) -> &mut Self {
        self.asset.replace(asset);

        self
    }

    pub fn set_recipient(&mut self, recipient: &'x str) -> &mut Self {
        self.pay_to.replace(recipient);

        self
    }

    pub fn set_resource(&mut self, resource: &'x str) -> &mut Self {
        self.resource.replace(resource);

        self
    }

    pub fn set_description(&mut self, description: &'x str) -> &mut Self {
        self.description.replace(description);

        self
    }

    pub fn set_mime_as_json(&mut self) -> &mut Self {
        self.mime_type.replace(MimeType::Json);

        self
    }

    pub fn set_mime_as_binary(&mut self) -> &mut Self {
        self.mime_type.replace(MimeType::Binary);

        self
    }

    pub fn set_output_schema(&mut self, json_schema: jzon::JsonValue) -> &mut Self {
        self.output_schema.replace(json_schema.to_string().into());

        self
    }

    pub fn set_max_timeout_seconds(&mut self, duration: Duration) -> &mut Self {
        self.max_timeout_seconds.replace(duration.as_secs());

        self
    }

    pub fn set_extra(&mut self, extra: PaymentRequestExtras<'x>) -> &mut Self {
        self.extra.replace(extra);

        self
    }

    pub fn build(self) -> X402Result<PaymentRequirements<'x>> {
        Ok(PaymentRequirements {
            scheme: PaymentScheme::Exact,
            network: self.network,
            max_amount_required: self
                .max_amount_required
                .ok_or(X402Error::MaxAmountIsMissing)?,
            asset: self.asset.ok_or(X402Error::AssetIsMissing)?,
            pay_to: self.pay_to.ok_or(X402Error::PayToIsMissing)?,
            resource: self.resource.ok_or(X402Error::ResourceIsMissing)?,
            description: self.description.ok_or(X402Error::DescriptionIsMissing)?,
            mime_type: self.mime_type,
            output_schema: self.output_schema,
            max_timeout_seconds: self
                .max_timeout_seconds
                .ok_or(X402Error::MaxTimeoutIsMissing)?,
            extra: self.extra.ok_or(X402Error::ExtraIsMissing)?,
        })
    }
}

impl<'x> Default for PaymentRequirementsBuilder<'x> {
    fn default() -> Self {
        Self {
            scheme: PaymentScheme::Exact,
            network: X402SolanaNetworkInfo::default(),
            max_amount_required: Option::default(),
            asset: Option::default(),
            pay_to: Option::default(),
            resource: Option::default(),
            description: Option::default(),
            mime_type: Option::default(),
            output_schema: Option::None,
            max_timeout_seconds: Option::default(),
            extra: Option::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Clone, Copy)]
pub enum X402SolanaNetworkInfo {
    Mainnet,
    Testnet,
    #[default]
    Devnet,
    Localnet,
}

impl X402SolanaNetworkInfo {
    pub const MAINNET: &str = "mainnet";
    pub const TESTNET: &str = "testnet";
    pub const DEVNET: &str = "devnet";
    pub const LOCALNET: &str = "localnet";

    pub const MAINNET_CHAIN: &str = "solana:mainnet";
    pub const TESTNET_CHAIN: &str = "solana:testnet";
    pub const DEVNET_CHAIN: &str = "solana:devnet";
    pub const LOCALNET_CHAIN: &str = "solana:localnet";

    pub const MAINNET_X402: &str = "solana-mainnet";
    pub const TESTNET_X402: &str = "solana-testnet";
    pub const DEVNET_X402: &str = "solana-devnet";
    pub const LOCALNET_X402: &str = "solana-localnet";
}

impl BlockchainNetwork for X402SolanaNetworkInfo {
    fn identifier(&self) -> &str {
        match self {
            Self::Mainnet => Self::MAINNET,
            Self::Testnet => Self::TESTNET,
            Self::Devnet => Self::DEVNET,
            Self::Localnet => Self::LOCALNET,
        }
    }

    fn chain(&self) -> &str {
        match self {
            Self::Mainnet => Self::MAINNET_CHAIN,
            Self::Testnet => Self::TESTNET_CHAIN,
            Self::Devnet => Self::DEVNET_CHAIN,
            Self::Localnet => Self::LOCALNET_CHAIN,
        }
    }

    fn x402_identifier(&self) -> &str {
        match self {
            Self::Mainnet => Self::MAINNET_X402,
            Self::Testnet => Self::TESTNET_X402,
            Self::Devnet => Self::DEVNET_X402,
            Self::Localnet => Self::LOCALNET_X402,
        }
    }
}

impl TryFrom<&str> for X402SolanaNetworkInfo {
    type Error = X402Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let network = match value {
            Self::MAINNET_X402 => Self::Mainnet,
            Self::MAINNET_CHAIN => Self::Mainnet,
            Self::TESTNET_X402 => Self::Testnet,
            Self::TESTNET_CHAIN => Self::Testnet,
            Self::DEVNET_X402 => Self::Devnet,
            Self::DEVNET_CHAIN => Self::Devnet,
            Self::LOCALNET_X402 => Self::Localnet,
            Self::LOCALNET_CHAIN => Self::Localnet,
            _ => return Err(X402Error::InvalidNetwork),
        };

        Ok(network)
    }
}

fn serialize_network<S>(network: &X402SolanaNetworkInfo, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(network.x402_identifier())
}

fn deserialize_network<'de, D>(deserializer: D) -> Result<X402SolanaNetworkInfo, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    s.as_str().try_into().map_err(serde::de::Error::custom)
}

pub trait BlockchainNetwork {
    fn identifier(&self) -> &str;

    fn chain(&self) -> &str;

    fn x402_identifier(&self) -> &str;
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
