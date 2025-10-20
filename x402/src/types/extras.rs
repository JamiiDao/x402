use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentRequestExtras<'x> {
    #[serde(borrow)]
    name: Option<&'x str>,
    #[serde(borrow)]
    version: Option<&'x str>,
    #[serde(borrow)]
    fee_payer: &'x str,
    token_extensions_mint: bool,
    decimals: u8,
    #[serde(borrow)]
    authority: Option<&'x str>,
}

impl<'x> PaymentRequestExtras<'x> {
    pub fn new(fee_payer: &'x str) -> Self {
        Self {
            fee_payer,
            token_extensions_mint: true,
            ..Default::default()
        }
    }

    pub fn set_name(mut self, name: &'x str) -> Self {
        self.name.replace(name);

        self
    }

    pub fn set_version(mut self, version: &'x str) -> Self {
        self.version.replace(version);

        self
    }

    pub fn set_token_extensions_mint(mut self) -> Self {
        self.token_extensions_mint = true;

        self
    }

    pub fn set_legacy_token_mint(mut self) -> Self {
        self.token_extensions_mint = false;

        self
    }

    pub fn set_authority(mut self, authority: &'x str) -> Self {
        self.authority.replace(authority);

        self
    }

    pub fn set_decimals(mut self, decimals: u8) -> Self {
        self.decimals = decimals;

        self
    }

    pub fn fee_payer(&self) -> &str {
        self.fee_payer
    }

    pub fn name(&self) -> Option<&str> {
        self.name
    }

    pub fn version(&self) -> Option<&str> {
        self.version
    }

    pub fn token_extensions_mint(&self) -> bool {
        self.token_extensions_mint
    }

    pub fn decimals(&self) -> u8 {
        self.decimals
    }

    pub fn authority(&self) -> Option<&str> {
        self.authority
    }
}
