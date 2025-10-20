use serde::{Deserialize, Deserializer, Serializer};

use crate::X402Error;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum MimeType {
    #[default]
    Json,
    Binary,
}

pub fn serialize_mime<S>(mime: &Option<MimeType>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match mime {
        Some(m) => serializer.serialize_str(m.as_str()),
        None => serializer.serialize_none(),
    }
}

pub fn deserialize_mime<'de, D>(deserializer: D) -> Result<Option<MimeType>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        Some(s) => s
            .as_str()
            .try_into()
            .map(Option::Some)
            .map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}

impl MimeType {
    pub const JSON_MIME: &str = "application/json";
    pub const BINARY_MIME: &str = "application/octet-stream";

    pub fn as_str(&self) -> &str {
        (*self).into()
    }
}

impl From<MimeType> for &str {
    fn from(value: MimeType) -> Self {
        match value {
            MimeType::Json => MimeType::JSON_MIME,
            MimeType::Binary => MimeType::BINARY_MIME,
        }
    }
}

impl TryFrom<&str> for MimeType {
    type Error = X402Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let outcome = match value {
            Self::JSON_MIME => Self::Json,
            Self::BINARY_MIME => Self::Binary,
            _ => return Err(X402Error::UnsupportedX402MimeType),
        };

        Ok(outcome)
    }
}
