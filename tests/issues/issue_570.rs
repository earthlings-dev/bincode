#![cfg(feature = "derive")]

#[derive(Debug, PartialEq, bincode::Encode, bincode::Decode)]
pub struct Eg<D, E> {
    data: (D, E),
}

#[test]
fn roundtrip_multi_generic() {
    let original = Eg { data: (42u32, 7u8) };
    let config = bincode::config::standard();
    let encoded = bincode::encode_to_vec(&original, config).unwrap();
    let (decoded, _): (Eg<u32, u8>, _) = bincode::decode_from_slice(&encoded, config).unwrap();
    assert_eq!(original, decoded);
}
