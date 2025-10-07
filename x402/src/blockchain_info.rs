use crate::PaymentScheme;

/// GET /supported
pub struct SupportedBlockchains {
    kinds: Vec<BlockchainKind>,
}

pub struct BlockchainKind {
    x402Version: u8,
    scheme: PaymentScheme,
    network: String,
}
