#![cfg(all(feature = "serde", feature = "alloc"))]

use bincode::Options;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct TestStruct {
  a: u32,
  b: String,
  c: Vec<u8>,
}

fn sample() -> TestStruct {
  TestStruct {
    a: 42, b: "hello".to_string(), c: vec![1, 2, 3]
  }
}

#[test]
fn roundtrip_serialize_deserialize() {
  let val = sample();
  let bytes = bincode::serialize(&val).unwrap();
  let decoded: TestStruct = bincode::deserialize(&bytes).unwrap();
  assert_eq!(val, decoded);
}

#[test]
fn roundtrip_primitives() {
  let val: u32 = 12345;
  let bytes = bincode::serialize(&val).unwrap();
  let decoded: u32 = bincode::deserialize(&bytes).unwrap();
  assert_eq!(val, decoded);
}

#[test]
fn legacy_config_equivalence() {
  let val = sample();
  let compat_bytes = bincode::serialize(&val).unwrap();
  let direct_bytes = bincode::serde::encode_to_vec(&val, bincode::config::legacy()).unwrap();
  assert_eq!(compat_bytes, direct_bytes);
}

#[test]
fn options_builder_big_endian() {
  let val: u32 = 0x01020304;
  let le_bytes = bincode::options()
    .with_little_endian()
    .with_fixed_int_encoding()
    .serialize(&val)
    .unwrap();
  let be_bytes = bincode::options()
    .with_big_endian()
    .with_fixed_int_encoding()
    .serialize(&val)
    .unwrap();
  // Big endian and little endian should produce different byte sequences
  assert_ne!(le_bytes, be_bytes);

  // Verify big endian produces the expected byte order
  assert_eq!(be_bytes, vec![0x01, 0x02, 0x03, 0x04]);
  assert_eq!(le_bytes, vec![0x04, 0x03, 0x02, 0x01]);
}

#[test]
fn options_serialize_equals_encode_to_vec() {
  let val = sample();
  let options_bytes = bincode::options().allow_trailing_bytes().serialize(&val).unwrap();
  let direct_bytes = bincode::serde::encode_to_vec(&val, bincode::config::standard()).unwrap();
  assert_eq!(options_bytes, direct_bytes);
}

#[test]
fn serialized_size_matches_serialize_len() {
  let val = sample();
  let size = bincode::serialized_size(&val).unwrap();
  let bytes = bincode::serialize(&val).unwrap();
  assert_eq!(size, bytes.len() as u64);
}

#[test]
fn serialized_size_via_options() {
  let val = sample();
  let size = bincode::options().with_big_endian().serialized_size(&val).unwrap();
  let bytes = bincode::options().with_big_endian().serialize(&val).unwrap();
  assert_eq!(size, bytes.len() as u64);
}

#[test]
#[cfg(feature = "std")]
fn serialize_into_deserialize_from() {
  use std::io::Cursor;
  let val = sample();
  let mut buf = Vec::new();
  bincode::serialize_into(&mut buf, &val).unwrap();

  let mut cursor = Cursor::new(&buf);
  let decoded: TestStruct = bincode::deserialize_from(&mut cursor).unwrap();
  assert_eq!(val, decoded);
}

#[test]
fn trailing_bytes_reject() {
  let val: u8 = 42;
  let mut bytes = bincode::options().with_fixed_int_encoding().serialize(&val).unwrap();
  // Append a trailing byte
  bytes.push(0xFF);

  // DefaultOptions rejects trailing bytes
  let result: Result<u8, _> = bincode::options().with_fixed_int_encoding().deserialize(&bytes);
  assert!(result.is_err());
}

#[test]
fn trailing_bytes_allow() {
  let val: u8 = 42;
  let mut bytes = bincode::options().with_fixed_int_encoding().serialize(&val).unwrap();
  bytes.push(0xFF);

  // Allow trailing bytes
  let result: u8 = bincode::options()
    .with_fixed_int_encoding()
    .allow_trailing_bytes()
    .deserialize(&bytes)
    .unwrap();
  assert_eq!(result, val);
}

#[test]
fn top_level_deserialize_allows_trailing() {
  // Top-level functions use legacy config which allows trailing
  let val: u8 = 42;
  let mut bytes = bincode::serialize(&val).unwrap();
  bytes.push(0xFF);

  let result: u8 = bincode::deserialize(&bytes).unwrap();
  assert_eq!(result, val);
}

#[test]
fn native_endian_works() {
  let val: u32 = 0x01020304;
  let native_bytes = bincode::config::standard()
    .with_native_endian()
    .with_fixed_int_encoding()
    .serialize(&val)
    .unwrap();

  // Native endian should match one of little or big endian
  let le_bytes = bincode::config::standard()
    .with_little_endian()
    .with_fixed_int_encoding()
    .serialize(&val)
    .unwrap();
  let be_bytes = bincode::config::standard()
    .with_big_endian()
    .with_fixed_int_encoding()
    .serialize(&val)
    .unwrap();

  assert!(native_bytes == le_bytes || native_bytes == be_bytes);
}

#[test]
fn deserialize_seed() {
  use core::marker::PhantomData;
  // PhantomData<T> as DeserializeSeed<'de> is equivalent to T::deserialize
  let val: u32 = 42;
  let bytes = bincode::options().with_fixed_int_encoding().serialize(&val).unwrap();

  let result: u32 = bincode::options()
    .with_fixed_int_encoding()
    .deserialize_seed(PhantomData::<u32>, &bytes)
    .unwrap();
  assert_eq!(result, val);
}

#[test]
fn deserialize_from_reader() {
  let val = sample();
  let bytes = bincode::serialize(&val).unwrap();
  let reader = bincode::de::read::SliceReader::new(&bytes);
  let decoded: TestStruct = bincode::deserialize_from_reader(reader).unwrap();
  assert_eq!(val, decoded);
}

#[test]
fn deserialize_from_reader_via_options() {
  let val = sample();
  let bytes = bincode::options().with_big_endian().serialize(&val).unwrap();
  let reader = bincode::de::read::SliceReader::new(&bytes);
  let decoded: TestStruct = bincode::options().with_big_endian().deserialize_from_reader(reader).unwrap();
  assert_eq!(val, decoded);
}

#[test]
fn deserialize_in_place_basic() {
  let val: u32 = 42;
  let bytes = bincode::serialize(&val).unwrap();

  let mut place: u32 = 0;
  bincode::deserialize_in_place(&bytes, &mut place).unwrap();
  assert_eq!(place, val);
}

#[test]
fn deserialize_in_place_struct() {
  let val = sample();
  let bytes = bincode::serialize(&val).unwrap();

  let mut place = TestStruct {
    a: 0, b: String::new(), c: Vec::new()
  };
  bincode::deserialize_in_place(&bytes, &mut place).unwrap();
  assert_eq!(place, val);
}

#[test]
fn deserialize_in_place_via_options_rejects_trailing() {
  let val: u8 = 42;
  let mut bytes = bincode::options().with_fixed_int_encoding().serialize(&val).unwrap();
  bytes.push(0xFF);

  let mut place: u8 = 0;
  let result = bincode::options()
    .with_fixed_int_encoding()
    .deserialize_in_place(&bytes, &mut place);
  assert!(result.is_err());
}

#[test]
fn options_with_limit() {
  let val = sample();
  let bytes = bincode::options().serialize(&val).unwrap();
  // Should fail to decode with a very small limit
  let result: Result<TestStruct, _> = bincode::options().with_limit::<2>().deserialize(&bytes);
  assert!(result.is_err());
}

#[test]
fn default_options_is_standard_plus_reject_trailing() {
  let val = sample();
  let options_bytes = bincode::options().allow_trailing_bytes().serialize(&val).unwrap();
  let standard_bytes = bincode::serde::encode_to_vec(&val, bincode::config::standard()).unwrap();
  assert_eq!(options_bytes, standard_bytes);
}

#[test]
fn borrowed_deserialization() {
  let text = "hello world";
  let bytes = bincode::serialize(&text).unwrap();
  let decoded: &str = bincode::config::legacy().deserialize(&bytes).unwrap();
  assert_eq!(decoded, text);
}
