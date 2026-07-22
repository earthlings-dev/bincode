#![cfg(feature = "derive")]

use bincode::Decode;
use bincode::Encode;

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
pub struct A;
#[derive(Debug, PartialEq, Encode, Decode, Clone)]
pub struct B<T>
where
  T: Clone + Encode + Decode<()>,
{
  pub t: T,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct MyStruct<T>
where
  T: Clone + Encode + Decode<()>,
{
  pub a: A,
  pub b: B<T>,
}

#[test]
fn roundtrip_nested_generics() {
  let original = MyStruct {
    a: A,
    b: B {
      t: 42u32
    },
  };
  let config = bincode::config::standard();
  let encoded = bincode::encode_to_vec(&original, config).unwrap();
  let (decoded, _): (MyStruct<u32>, _) = bincode::decode_from_slice(&encoded, config).unwrap();
  assert_eq!(original, decoded);
}
