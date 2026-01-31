mod datamatrix;
mod qrcode;

use ::datamatrix::placement::PathSegment;
use wasm_minimal_protocol::{initiate_protocol, wasm_func};

type Bitmap = ::datamatrix::placement::Bitmap<bool>;

initiate_protocol!();

#[wasm_func]
pub fn encode(data: &[u8], barcode: &[u8]) -> Result<Vec<u8>, String> {
    let bitmap = match barcode {
        // aztec
        b"datamatrix" => datamatrix::Options::default().encode(data),
        // pdf417
        b"qrcode" => qrcode::Options::default().encode(data),
        _ => return Err("invalid barcode".to_string()),
    }?;
    Ok(bitmap_to_path(&bitmap))
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
        assert!(encode(b"0", b"datamatrix").is_ok());
    }

    #[test]
    fn test_errors() {
        assert_eq!(encode(b"0", b"unknown").unwrap_err(), "invalid barcode");
    }
}
