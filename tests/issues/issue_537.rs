#![cfg(all(feature = "derive", feature = "std"))]

use bincode::{Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
struct Foo<Bar = ()> {
    x: Bar,
}

#[test]
fn roundtrip_default_type_param() {
    let original = Foo { x: () };
    let config = bincode::config::standard();
    let encoded = bincode::encode_to_vec(&original, config).unwrap();
    let (decoded, _): (Foo, _) = bincode::decode_from_slice(&encoded, config).unwrap();
    assert_eq!(original, decoded);
}
