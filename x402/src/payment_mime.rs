use crate::X402Error;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum MimeType {
    Json,
    Binary,
}

impl MimeType {
    pub const JSON_MIME: &str = "application/json";
    pub const BINARY_MIME: &str = "application/octet-stream";

    pub fn as_str(&self) -> &'static str {
        (*self).into()
    }
}

impl From<MimeType> for &'static str {
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
