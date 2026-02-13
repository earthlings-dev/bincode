#![cfg(all(feature = "derive", feature = "std"))]

use bincode::{Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
pub enum TypeOfFile {
    Unknown = -1,
}

#[test]
fn roundtrip_negative_discriminant_enum() {
    let original = TypeOfFile::Unknown;
    let config = bincode::config::standard();
    let encoded = bincode::encode_to_vec(&original, config).unwrap();
    let (decoded, _): (TypeOfFile, _) = bincode::decode_from_slice(&encoded, config).unwrap();
    assert_eq!(original, decoded);
}
