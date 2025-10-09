use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentRequestExtras<'x> {
    #[serde(borrow)]
    name: Option<&'x str>,
    #[serde(borrow)]
    version: Option<&'x str>,
    #[serde(borrow)]
    fee_payer: &'x str,
}

impl<'x> PaymentRequestExtras<'x> {
    pub fn new(fee_payer: &'x str) -> Self {
        Self {
            fee_payer,
            ..Default::default()
        }
    }

    pub fn set_name(&mut self, name: &'x str) -> &mut Self {
        self.name.replace(name);

        self
    }

    pub fn set_version(&mut self, version: &'x str) -> &mut Self {
        self.version.replace(version);

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
}
