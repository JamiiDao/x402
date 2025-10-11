use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::{
    PaymentRequirements, PaymentRequirementsResponse, X402PaymentErrorStatusCode,
    X402SolanaNetworkInfo, X402Version, deserialize_network, serialize_network,
};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub struct SettlementResponse<'x, T: Clone> {
    /// Indicates whether the payment settlement was successful
    success: bool,
    /// Blockchain transaction hash (empty string if settlement failed)
    #[serde(borrow)]
    transaction: Option<&'x str>,
    /// Blockchain network identifier (solana-devnet, solana-mainnet, solana-testnet, solana-localnet)
    #[serde(deserialize_with = "deserialize_network")]
    #[serde(serialize_with = "serialize_network")]
    network: X402SolanaNetworkInfo,
    /// Address of the payer's wallet
    #[serde(borrow)]
    payer: &'x str,
    data: T,
    timestamp: &'x str, // Example "2024-01-15T10:30:00Z"
}

impl<'x, T> SettlementResponse<'x, T>
where
    T: Serialize + Deserialize<'x> + Clone + Default,
{
    pub fn new(success: bool) -> Self {
        Self {
            success,
            ..Default::default()
        }
    }

    pub fn set_transaction_signature(&mut self, transaction_signature: &'x str) -> &mut Self {
        self.transaction.replace(transaction_signature);

        self
    }

    pub fn set_data(&mut self, data: T) -> &mut Self {
        self.data = data;

        self
    }

    // TODO use rust time formats
    pub fn set_timestamp(&mut self, timestamp: &'x str) -> &mut Self {
        self.timestamp = timestamp;

        self
    }

    pub fn set_payer(&mut self, fee_payer: &'x str) -> &mut Self {
        self.payer = fee_payer;

        self
    }

    pub fn set_mainnet(&mut self) -> &mut Self {
        self.network = X402SolanaNetworkInfo::Mainnet;

        self
    }

    pub fn set_testnet(&mut self) -> &mut Self {
        self.network = X402SolanaNetworkInfo::Testnet;

        self
    }

    pub fn set_devnet(&mut self) -> &mut Self {
        self.network = X402SolanaNetworkInfo::Devnet;

        self
    }

    pub fn set_localnet(&mut self) -> &mut Self {
        self.network = X402SolanaNetworkInfo::Localnet;

        self
    }

    /// Indicates whether the payment settlement was successful
    pub fn success(&self) -> bool {
        self.success
    }

    /// Blockchain transaction hash (empty string if settlement failed)
    pub fn transaction(&self) -> Option<&str> {
        self.transaction
    }

    /// Blockchain network identifier (solana-devnet, solana-mainnet, solana-testnet, solana-localnet)
    pub fn network(&self) -> X402SolanaNetworkInfo {
        self.network
    }

    /// Address of the payer's wallet
    pub fn payer(&self) -> &str {
        self.payer
    }
    pub fn timestamp(&self) -> &str {
        self.timestamp
    }

    pub fn data(&self) -> &T {
        &self.data
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct SettlementResponseError<'x> {
    payload: PaymentRequirementsResponse<'x>,
    status_code: X402PaymentErrorStatusCode,
}

impl<'x> SettlementResponseError<'x> {
    pub fn new(payment_requirements: PaymentRequirementsResponse<'x>) -> Self {
        Self {
            payload: payment_requirements,
            status_code: X402PaymentErrorStatusCode::PaymentRequired,
        }
    }

    pub fn inner(&'x self) -> &'x PaymentRequirementsResponse<'x> {
        &self.payload
    }

    pub fn take(self) -> PaymentRequirementsResponse<'x> {
        self.payload
    }

    pub fn set_error_reason(mut self, error: &'x str) -> Self {
        self.payload.set_error_reason(error);

        self
    }

    pub fn set_status_code(mut self, status_code: X402PaymentErrorStatusCode) -> Self {
        self.status_code = status_code;

        self
    }

    pub fn x402_version(&self) -> X402Version {
        self.payload.x402_version()
    }

    pub fn accepts(&self) -> &[PaymentRequirements<'_>] {
        self.payload.accepts()
    }

    pub fn status_code(&self) -> u16 {
        self.status_code.status_code()
    }

    pub fn status_code_description(&self) -> &str {
        self.status_code.description()
    }

    pub fn error_header(&'x self) -> Cow<'x, str> {
        Cow::Borrowed("")
            + Cow::Owned(self.status_code().to_string())
            + " "
            + self.status_code_description()
    }

    /// Error reason if settlement failed (omitted if successful)
    pub fn error_reason(&self) -> &str {
        self.payload.error_reason()
    }
}
