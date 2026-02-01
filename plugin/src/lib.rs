mod datamatrix;
mod qrcode;

use ::datamatrix::placement::PathSegment;
use serde::de::DeserializeOwned;
use wasm_minimal_protocol::{initiate_protocol, wasm_func};

type Bitmap = ::datamatrix::placement::Bitmap<bool>;

initiate_protocol!();

#[wasm_func]
pub fn encode(
    data: &[u8],
    data_type: &[u8],
    barcode: &[u8],
    options: &[u8],
) -> Result<Vec<u8>, String> {
    let is_utf8 = match data_type {
        b"bytes" => false,
        b"str" => !data.is_ascii(),
        _ => return Err("invalid data_type".to_string()),
    };
    let bitmap = match barcode {
        // aztec
        b"datamatrix" => cbor_read::<datamatrix::Options>(options)?.encode(data, is_utf8),
        // pdf417
        b"qrcode" => cbor_read::<qrcode::Options>(options)?.encode(data, is_utf8),
        _ => return Err("invalid barcode".to_string()),
    }?;
    Ok(bitmap_to_path(&bitmap))
}

fn cbor_read<T: DeserializeOwned>(options: &[u8]) -> Result<T, String> {
    ciborium::de::from_reader(options).map_err(|e| e.to_string())
}

fn bitmap_to_path(bitmap: &Bitmap) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.push_i16(bitmap.width().try_into().unwrap());
    bytes.push_i16(bitmap.height().try_into().unwrap());
    for part in bitmap.path() {
        match part {
            PathSegment::Move(dx, dy) => {
                bytes.push(0);
                bytes.push_i16(dx);
                bytes.push_i16(dy);
            }
            PathSegment::Horizontal(dx) => {
                bytes.push(1);
                bytes.push_i16(dx);
            }
            PathSegment::Vertical(dy) => {
                bytes.push(2);
                bytes.push_i16(dy);
            }
            PathSegment::Close => bytes.push(3),
        }
    }
    bytes
}

trait Bytes {
    fn push_i16(&mut self, n: i16);
}

impl Bytes for Vec<u8> {
    fn push_i16(&mut self, n: i16) {
        self.extend(n.to_le_bytes());
    }
}

#[derive(Debug)]
struct Error(String);

impl From<Error> for String {
    fn from(value: Error) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert!(encode(b"0", b"str", b"datamatrix", &[0xA0]).is_ok());
    }

    #[test]
    fn test_errors() {
        assert_eq!(
            encode(b"0", b"unknown", b"", b"").unwrap_err(),
            "invalid data_type"
        );
        assert_eq!(
            encode(b"0", b"str", b"unknown", b"").unwrap_err(),
            "invalid barcode"
        );
    }
}
