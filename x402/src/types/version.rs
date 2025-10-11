use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::X402Error;

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default, Serialize, Deserialize,
)]
#[non_exhaustive]
pub enum X402Version {
    #[default]
    V1 = 1,
}

impl TryFrom<u8> for X402Version {
    type Error = X402Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::V1),
            _ => Err(X402Error::InvalidX402Version),
        }
    }
}

pub fn serialize_x402_version<S>(version: &X402Version, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_u8(*version as u8)
}

pub fn deserialize_x402_version<'de, D>(deserializer: D) -> Result<X402Version, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    s.as_str()
        .parse::<u8>()
        .map_err(serde::de::Error::custom)?
        .try_into()
        .map_err(serde::de::Error::custom)
}
