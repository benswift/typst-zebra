use crate::{Bitmap, Error};
use qrcode::{Color, QrCode, types::QrError};

#[derive(Debug, Default)]
pub struct Options {}

impl Options {
    pub fn encode(&self, data: &[u8]) -> Result<Bitmap, Error> {
        let qrcode = QrCode::new(data)?;
        Ok(Bitmap::new(
            qrcode.to_colors().iter().map(|c| *c != Color::Light),
            qrcode.width(),
        ))
    }
}

impl From<QrError> for Error {
    fn from(value: QrError) -> Self {
        Self(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_0() {
        assert!(Options::default().encode(b"0").is_ok());
    }
}
