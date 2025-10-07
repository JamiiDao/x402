use crate::BorrowedStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct SettlementResponse<'x> {
    /// Indicates whether the payment settlement was successful
    success: bool,
    /// Error reason if settlement failed (omitted if successful)
    error_reason: Option<&'x dyn BorrowedStr>,
    /// Blockchain transaction hash (empty string if settlement failed)
    transaction: Option<&'x dyn BorrowedStr>,
    /// Blockchain network identifier
    network: &'x dyn BorrowedStr,
    /// Address of the payer's wallet
    payer: &'x dyn BorrowedStr,
}
