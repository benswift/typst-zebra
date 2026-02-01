use crate::{Bitmap, Error};
use datamatrix::{DataMatrixBuilder, SymbolList, data::DataEncodingError};
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Options {
    #[serde(default)]
    encodation_type: Option<EncodationType>,
    #[serde(default)]
    symbols: Symbol,
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
enum EncodationType {
    Ascii,
    C40,
    Text,
    X12,
    Edifact,
    Base256,
}

impl From<EncodationType> for datamatrix::EncodationType {
    fn from(value: EncodationType) -> Self {
        match value {
            EncodationType::Ascii => Self::Ascii,
            EncodationType::C40 => Self::C40,
            EncodationType::Text => Self::Text,
            EncodationType::X12 => Self::X12,
            EncodationType::Edifact => Self::Edifact,
            EncodationType::Base256 => Self::Base256,
        }
    }
}

#[derive(Debug, Default, Deserialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
enum Symbol {
    #[default]
    Square,
    Rect,
    #[serde(rename = "rect-8")]
    Rect8,
    #[serde(rename = "rect-12")]
    Rect12,
    #[serde(rename = "rect-16")]
    Rect16,
    RectExt,
    #[serde(rename = "rect-ext-8")]
    RectExt8,
    #[serde(rename = "rect-ext-12")]
    RectExt12,
    #[serde(rename = "rect-ext-16")]
    RectExt16,
    #[serde(rename = "rect-ext-20")]
    RectExt20,
    #[serde(rename = "rect-ext-24")]
    RectExt24,
    #[serde(rename = "rect-ext-26")]
    RectExt26,
}

impl From<Symbol> for SymbolList {
    fn from(value: Symbol) -> Self {
        let rect = || SymbolList::default().enforce_rectangular();
        let rect_ = |height| rect().enforce_height_in(height..=height);
        let rect_ext = || SymbolList::all().enforce_rectangular();
        let rect_ext_ = |height| rect_ext().enforce_height_in(height..=height);
        match value {
            Symbol::Square => SymbolList::default().enforce_square(),
            Symbol::Rect => rect(),
            Symbol::Rect8 => rect_(8),
            Symbol::Rect12 => rect_(12),
            Symbol::Rect16 => rect_(16),
            Symbol::RectExt => rect_ext(),
            Symbol::RectExt8 => rect_ext_(8),
            Symbol::RectExt12 => rect_ext_(12),
            Symbol::RectExt16 => rect_ext_(16),
            Symbol::RectExt20 => rect_ext_(20),
            Symbol::RectExt24 => rect_ext_(24),
            Symbol::RectExt26 => rect_ext_(26),
        }
    }
}

impl Options {
    pub fn encode(&self, data: &[u8], is_utf8: bool) -> Result<Bitmap, Error> {
        let datamatrix = DataMatrixBuilder::new()
            .with_encodation_types(
                self.encodation_type
                    .map_or_else(datamatrix::EncodationType::all, |x| {
                        datamatrix::EncodationType::from(x).into()
                    }),
            )
            .with_symbol_list(self.symbols)
            .encode_eci(data, is_utf8.then_some(ECI_UTF8))?;
        Ok(datamatrix.bitmap())
    }
}

const ECI_UTF8: u32 = 26;

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
        assert!(Options::default().encode(b"0", false).is_ok());
    }
}
