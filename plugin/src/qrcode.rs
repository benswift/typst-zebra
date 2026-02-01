use crate::{Bitmap, Error};
use qrcode::{Color, QrCode, bits::Bits, types::QrError};
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Options {
    #[serde(default)]
    version: Version,
    #[serde(default)]
    mode: Option<Mode>,
    #[serde(default)]
    ec_level: EcLevel,
}

#[derive(Debug, Default, Deserialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
enum Version {
    #[default]
    Normal,
    Micro,
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum Mode {
    Numeric,
    Alphanumeric,
    Byte,
}

#[derive(Debug, Default, Deserialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
enum EcLevel {
    L,
    #[default]
    M,
    Q,
    H,
}

impl From<EcLevel> for qrcode::EcLevel {
    fn from(value: EcLevel) -> Self {
        match value {
            EcLevel::L => Self::L,
            EcLevel::M => Self::M,
            EcLevel::Q => Self::Q,
            EcLevel::H => Self::H,
        }
    }
}

impl Options {
    pub fn encode(&self, data: &[u8], is_utf8: bool) -> Result<Bitmap, Error> {
        let ec_level = self.ec_level.into();
        let try_encode = |version| -> Result<QrCode, QrError> {
            let mut bits = Bits::new(version);
            if is_utf8 {
                bits.push_eci_designator(ECI_UTF8)?;
            }
            match self.mode {
                None => bits.push_optimal_data(data),
                Some(Mode::Numeric) => bits.push_numeric_data(data),
                Some(Mode::Alphanumeric) => bits.push_alphanumeric_data(data),
                Some(Mode::Byte) => bits.push_byte_data(data),
            }?;
            bits.push_terminator(ec_level)?;
            QrCode::with_bits(bits, ec_level)
        };

        let max_version = match self.version {
            Version::Normal => 40,
            Version::Micro => 4,
        };
        let mut qrcode: Result<QrCode, QrError> = Err(QrError::InvalidVersion);
        for version in 1..=max_version {
            let version = match self.version {
                Version::Normal => qrcode::Version::Normal(version),
                Version::Micro => qrcode::Version::Micro(version),
            };
            qrcode = try_encode(version);
            match qrcode {
                Err(
                    QrError::DataTooLong
                    | QrError::UnsupportedCharacterSet
                    | QrError::InvalidVersion,
                ) => (),
                _ => break,
            }
        }
        let qrcode = qrcode?;

        Ok(Bitmap::new(
            qrcode.to_colors().iter().map(|c| *c != Color::Light),
            qrcode.width(),
        ))
    }
}

const ECI_UTF8: u32 = 26;

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
        assert!(Options::default().encode(b"0", false).is_ok());
    }
}
