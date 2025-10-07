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
