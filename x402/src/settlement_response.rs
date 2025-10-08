use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub struct SettlementResponse<'x> {
    /// Indicates whether the payment settlement was successful
    success: bool,
    /// Error reason if settlement failed (omitted if successful)
    #[serde(borrow)]
    error_reason: Option<&'x str>,
    /// Blockchain transaction hash (empty string if settlement failed)
    #[serde(borrow)]
    transaction: Option<&'x str>,
    /// Blockchain network identifier
    #[serde(borrow)]
    network: &'x str,
    /// Address of the payer's wallet
    #[serde(borrow)]
    payer: &'x str,
}
