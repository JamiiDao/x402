use std::borrow::Cow;

use base64ct::{Base64, Encoding};
use serde::{Deserialize, Serialize};

use crate::{
    PaymentScheme, X402SolanaNetworkInfo, X402Version, deserialize_network, serialize_network,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct XPaymentHeader<'x> {
    x402_version: X402Version,
    scheme: PaymentScheme,
    #[serde(deserialize_with = "deserialize_network")]
    #[serde(serialize_with = "serialize_network")]
    network: X402SolanaNetworkInfo,
    #[serde(borrow)]
    payload: XPaymentPayload<'x>,
}

impl<'x> XPaymentHeader<'x> {
    pub fn new(transaction: impl AsRef<[u8]>) -> Self {
        let payload = Base64::encode_string(transaction.as_ref());
        let payload = XPaymentPayload {
            transaction: payload.into(),
        };

        Self {
            payload,
            ..Default::default()
        }
    }

    pub fn change_x402_version(mut self, x402_version: X402Version) -> Self {
        self.x402_version = x402_version;

        self
    }

    pub fn use_mainnet(mut self) -> Self {
        self.network = X402SolanaNetworkInfo::Mainnet;

        self
    }

    pub fn use_testnet(mut self) -> Self {
        self.network = X402SolanaNetworkInfo::Testnet;

        self
    }

    pub fn use_devnet(mut self) -> Self {
        self.network = X402SolanaNetworkInfo::Devnet;

        self
    }

    pub fn use_localnet(mut self) -> Self {
        self.network = X402SolanaNetworkInfo::Localnet;

        self
    }

    pub fn x402_version(&self) -> X402Version {
        self.x402_version
    }

    pub fn network(&self) -> X402SolanaNetworkInfo {
        self.network
    }

    pub fn scheme(&self) -> PaymentScheme {
        self.scheme
    }

    pub fn payload(&'x self) -> &'x XPaymentPayload<'x> {
        &self.payload
    }

    pub fn payload_transaction(&self) -> &str {
        self.payload.transaction.as_ref()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, Clone, Default)]
pub struct XPaymentPayload<'x> {
    transaction: Cow<'x, str>,
}

impl<'x> XPaymentPayload<'x> {
    pub fn transaction(&self) -> &str {
        self.transaction.as_ref()
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Deserialize, Serialize)]
pub struct XPaymentResponse<'x> {
    success: bool,
    transaction: &'x str,
    #[serde(deserialize_with = "deserialize_network")]
    #[serde(serialize_with = "serialize_network")]
    network: X402SolanaNetworkInfo,
    payer: &'x str,
}

impl<'x> XPaymentResponse<'x> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_success(mut self) -> Self {
        self.success = true;

        self
    }

    pub fn set_failed(mut self) -> Self {
        self.success = false;

        self
    }

    pub fn set_mainnet(mut self) -> Self {
        self.network = X402SolanaNetworkInfo::Mainnet;

        self
    }

    pub fn set_testnet(mut self) -> Self {
        self.network = X402SolanaNetworkInfo::Testnet;

        self
    }

    pub fn set_devnet(mut self) -> Self {
        self.network = X402SolanaNetworkInfo::Devnet;

        self
    }

    pub fn set_localnet(mut self) -> Self {
        self.network = X402SolanaNetworkInfo::Localnet;

        self
    }

    pub fn set_transaction_signature(mut self, signature: &'x str) -> Self {
        self.transaction = signature;

        self
    }

    pub fn set_payer(mut self, fee_payer: &'x str) -> Self {
        self.payer = fee_payer;

        self
    }

    pub fn success(&self) -> bool {
        self.success
    }

    pub fn transaction(&self) -> &str {
        self.transaction
    }

    pub fn network(&self) -> X402SolanaNetworkInfo {
        self.network
    }

    pub fn fee_payer(&self) -> &str {
        self.payer
    }
}
