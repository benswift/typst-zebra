use crate::{Bitmap, Error};
use datamatrix::{DataMatrixBuilder, SymbolList, data::DataEncodingError};

#[derive(Debug, Default)]
pub struct Options {}

impl Options {
    pub fn encode(&self, data: &[u8]) -> Result<Bitmap, Error> {
        let datamatrix = DataMatrixBuilder::new()
            .with_symbol_list(SymbolList::default().enforce_square())
            .encode_eci(data, None)?;
        Ok(datamatrix.bitmap())
    }
}

impl From<DataEncodingError> for Error {
    fn from(value: DataEncodingError) -> Self {
        let value = match value {
            DataEncodingError::TooMuchOrIllegalData => "too much or illegal data",
            DataEncodingError::SymbolListEmpty => "symbol list empty",
        };
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
