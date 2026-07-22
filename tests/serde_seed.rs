#![cfg(feature = "serde")]

use bincode::{Options, config::Config, de::read::Reader};
use serde::{Deserialize, de::DeserializeSeed};

#[derive(Copy, Clone)]
struct AddToDecoded(u32);

impl<'de> DeserializeSeed<'de> for AddToDecoded {
    type Value = u32;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(u32::deserialize(deserializer)? + self.0)
    }
}

fn encoded_value<C: Config>(config: C) -> ([u8; 8], usize) {
    let mut bytes = [0; 8];
    let len = bincode::serde::encode_into_slice(42_u32, &mut bytes, config).unwrap();
    (bytes, len)
}

#[test]
fn seeded_slice_and_reader_apis_apply_the_seed() {
    let config = bincode::config::standard();
    let (mut bytes, len) = encoded_value(config);
    bytes[len] = 0xA5;
    let input = &bytes[..=len];

    let (direct_slice, consumed) =
        bincode::serde::seed_decode_from_slice(AddToDecoded(8), input, config).unwrap();
    assert_eq!(direct_slice, 50);
    assert_eq!(consumed, len);

    let options_slice = config
        .allow_trailing_bytes()
        .deserialize_seed(AddToDecoded(8), input)
        .unwrap();
    assert_eq!(options_slice, direct_slice);

    let mut reader = bincode::de::read::SliceReader::new(input);
    let direct_reader =
        bincode::serde::seed_decode_from_reader(AddToDecoded(8), &mut reader, config).unwrap();
    assert_eq!(direct_reader, direct_slice);
    let mut sentinel = [0];
    reader.read(&mut sentinel).unwrap();
    assert_eq!(sentinel, [0xA5]);

    let mut reader = bincode::de::read::SliceReader::new(input);
    let options_reader = config
        .deserialize_from_reader_seed(AddToDecoded(8), &mut reader)
        .unwrap();
    assert_eq!(options_reader, direct_slice);
    reader.read(&mut sentinel).unwrap();
    assert_eq!(sentinel, [0xA5]);
}

#[test]
fn seeded_slice_options_preserve_trailing_byte_policy() {
    let config = bincode::config::standard();
    let (mut bytes, len) = encoded_value(config);
    bytes[len] = 0xA5;
    let input = &bytes[..=len];

    let rejected = config
        .reject_trailing_bytes()
        .deserialize_seed(AddToDecoded(0), input);
    assert!(matches!(
        rejected,
        Err(bincode::error::DecodeError::Other(_))
    ));

    let accepted = config
        .allow_trailing_bytes()
        .deserialize_seed(AddToDecoded(0), input)
        .unwrap();
    assert_eq!(accepted, 42);
}

#[test]
fn seeded_reader_reports_truncated_input() {
    let config = bincode::config::standard().with_fixed_int_encoding();
    let mut reader = bincode::de::read::SliceReader::new(&[1, 2]);
    let error =
        bincode::serde::seed_decode_from_reader(AddToDecoded(0), &mut reader, config).unwrap_err();
    assert!(matches!(
        error,
        bincode::error::DecodeError::UnexpectedEnd { .. }
    ));

    let mut reader = bincode::de::read::SliceReader::new(&[1, 2]);
    let error = config
        .deserialize_from_reader_seed(AddToDecoded(0), &mut reader)
        .unwrap_err();
    assert!(matches!(
        error,
        bincode::error::DecodeError::UnexpectedEnd { .. }
    ));
}

#[cfg(feature = "std")]
#[test]
fn seeded_std_read_apis_apply_the_seed_and_preserve_position() {
    use std::io::Cursor;

    let config = bincode::config::standard();
    let (mut bytes, len) = encoded_value(config);
    bytes[len] = 0xA5;
    let input = &bytes[..=len];

    let mut cursor = Cursor::new(input);
    let direct =
        bincode::serde::seed_decode_from_std_read(AddToDecoded(8), &mut cursor, config).unwrap();
    assert_eq!(direct, 50);
    assert_eq!(cursor.position(), len as u64);
    let mut sentinel = [0];
    std::io::Read::read_exact(&mut cursor, &mut sentinel).unwrap();
    assert_eq!(sentinel, [0xA5]);

    let mut cursor = Cursor::new(input);
    let via_options = config
        .deserialize_from_seed(AddToDecoded(8), &mut cursor)
        .unwrap();
    assert_eq!(via_options, direct);
    assert_eq!(cursor.position(), len as u64);
    std::io::Read::read_exact(&mut cursor, &mut sentinel).unwrap();
    assert_eq!(sentinel, [0xA5]);
}

#[cfg(feature = "std")]
#[test]
fn seeded_std_read_reports_truncated_input() {
    use std::io::Cursor;

    let config = bincode::config::standard().with_fixed_int_encoding();
    let mut cursor = Cursor::new([1, 2]);
    let error = bincode::serde::seed_decode_from_std_read(AddToDecoded(0), &mut cursor, config)
        .unwrap_err();
    match error {
        bincode::error::DecodeError::Io { inner, .. } => {
            assert_eq!(inner.kind(), std::io::ErrorKind::UnexpectedEof);
        }
        other => panic!("expected an I/O error, got {other:?}"),
    }

    let mut cursor = Cursor::new([1, 2]);
    let error = config
        .deserialize_from_seed(AddToDecoded(0), &mut cursor)
        .unwrap_err();
    match error {
        bincode::error::DecodeError::Io { inner, .. } => {
            assert_eq!(inner.kind(), std::io::ErrorKind::UnexpectedEof);
        }
        other => panic!("expected an I/O error, got {other:?}"),
    }
}
