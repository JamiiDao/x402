use crate::X402Error;

pub enum MimeType {
    Json,
    Binary,
}

impl TryFrom<&str> for MimeType {
    type Error = X402Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let outcome = match value {
            "application/json" => Self::Json,
            "application/octet-stream" => Self::Binary,
            _ => return Err(X402Error::UnsupportedX402MimeType),
        };

        Ok(outcome)
    }
}
