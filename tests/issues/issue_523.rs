#![cfg(all(feature = "derive", feature = "std"))]

extern crate std;

use bincode::{Decode, Encode};
use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq, Encode, Decode)]
pub struct Foo<'a>(Cow<'a, str>);

#[test]
fn roundtrip_cow_str() {
    let original = Foo(Cow::Borrowed("hello"));
    let config = bincode::config::standard();
    let encoded = bincode::encode_to_vec(&original, config).unwrap();
    let (decoded, _): (Foo, _) = bincode::decode_from_slice(&encoded, config).unwrap();
    assert_eq!(original, decoded);
}
