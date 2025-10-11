use serde::{Deserialize, Serialize};

use crate::{
    PaymentScheme, X402SolanaNetworkInfo, X402Version, deserialize_network,
    deserialize_x402_version, serialize_network, serialize_x402_version,
};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub struct SupportedSchemes {
    kinds: Vec<SchemeKind>,
}

impl SupportedSchemes {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_kind(&mut self, supported_scheme: SchemeKind) -> &mut Self {
        self.kinds.push(supported_scheme);

        self
    }

    pub fn add_kinds(&mut self, supported_scheme: &[SchemeKind]) -> &mut Self {
        supported_scheme.iter().for_each(|supported_scheme| {
            self.kinds.push(*supported_scheme);
        });

        self
    }

    pub fn kinds(&self) -> &[SchemeKind] {
        self.kinds.as_slice()
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub struct SchemeKind {
    #[serde(deserialize_with = "deserialize_x402_version")]
    #[serde(serialize_with = "serialize_x402_version")]
    pub x402_version: X402Version,
    pub scheme: PaymentScheme,
    #[serde(deserialize_with = "deserialize_network")]
    #[serde(serialize_with = "serialize_network")]
    pub network: X402SolanaNetworkInfo,
}
