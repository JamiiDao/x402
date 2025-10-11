use serde::{Deserialize, Serialize};

/// The "exact" scheme uses for solana
/// ```
#[derive(Debug, PartialEq, Eq, Default, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub enum PaymentScheme {
    #[default]
    Exact,
}

impl PaymentScheme {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Exact => "exact",
        }
    }
}
