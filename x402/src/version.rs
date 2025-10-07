use crate::X402Error;

pub enum X402Version {
    V1,
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
