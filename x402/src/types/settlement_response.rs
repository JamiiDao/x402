use serde::{Deserialize, Serialize};

use crate::{X402SolanaNetworkInfo, deserialize_network, serialize_network};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub struct SettlementResponse<'x> {
    /// Indicates whether the payment settlement was successful
    success: bool,
    /// Error reason if settlement failed (omitted if successful)
    #[serde(borrow)]
    error_reason: Option<&'x str>,
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
}

impl<'x> SettlementResponse<'x> {
    pub fn new(success: bool) -> Self {
        Self {
            success,
            ..Default::default()
        }
    }

    pub fn set_error_reason(&mut self, error: &'x str) -> &mut Self {
        self.error_reason.replace(error);

        self
    }

    pub fn set_transaction_signature(&mut self, transaction_signature: &'x str) -> &mut Self {
        self.transaction.replace(transaction_signature);

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

    /// Error reason if settlement failed (omitted if successful)
    pub fn error_reason(&self) -> Option<&str> {
        self.error_reason
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
}
