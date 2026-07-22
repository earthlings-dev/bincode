#![cfg(test)]

use ::rand::RngExt;
use bincode::Options;

mod membership;
mod misc;
mod rand;
mod sway;

/// Test that all three v3 API surfaces (native encode/decode, serde encode/decode,
/// and v1 compat serialize/deserialize) produce identical output for a given value
/// and config.
pub fn test_same_with_config<T, C>(t: &T, config: C)
where
    T: bincode::Encode
        + bincode::Decode<()>
        + serde::Serialize
        + serde::de::DeserializeOwned
        + core::fmt::Debug
        + PartialEq,
    C: bincode::config::Config + Copy,
{
    // v1 compat API (Options trait — serialize/deserialize)
    let v1_encoded = config.serialize(t).unwrap();

    println!("Encoded {t:?} as {v1_encoded:?}");

    // v2 native API (encode_to_vec)
    let v2_encoded = bincode::encode_to_vec(t, config).unwrap();
    assert_eq!(
        v1_encoded,
        v2_encoded,
        "{t:?} encodes differently between v1 compat and native API\nbincode config {:?}",
        core::any::type_name::<C>(),
    );

    // v2 serde API (serde::encode_to_vec)
    let v2_serde_encoded = bincode::serde::encode_to_vec(t, config).unwrap();
    assert_eq!(
        v1_encoded, v2_serde_encoded,
        "{t:?} encodes differently between v1 compat and serde API"
    );

    // Deserialize via v1 compat API
    let v1_decoded: T = config.deserialize(&v1_encoded).unwrap();
    assert_eq!(&v1_decoded, t);

    // Deserialize via v2 native API
    let v2_decoded: T = bincode::decode_from_slice(&v1_encoded, config).unwrap().0;
    assert_eq!(&v2_decoded, t);

    // Deserialize via v2 serde API
    let v2_serde_decoded: T = bincode::serde::decode_from_slice(&v1_encoded, config)
        .unwrap()
        .0;
    assert_eq!(&v2_serde_decoded, t);
}

pub fn test_same<T>(t: T)
where
    T: bincode::Encode
        + bincode::Decode<()>
        + serde::Serialize
        + serde::de::DeserializeOwned
        + core::fmt::Debug
        + PartialEq,
{
    // legacy() = fixint + little-endian (matches v1 default internal config)
    test_same_with_config(&t, bincode::config::legacy());

    // Check a bunch of different configs:
    test_same_with_config(
        &t,
        bincode::config::legacy()
            .with_big_endian()
            .with_variable_int_encoding(),
    );
    test_same_with_config(
        &t,
        bincode::config::legacy()
            .with_little_endian()
            .with_variable_int_encoding(),
    );
    test_same_with_config(
        &t,
        bincode::config::legacy()
            .with_big_endian()
            .with_fixed_int_encoding(),
    );
    test_same_with_config(
        &t,
        bincode::config::legacy()
            .with_little_endian()
            .with_fixed_int_encoding(),
    );
}

pub fn gen_string(rng: &mut impl RngExt) -> String {
    let len = rng.random_range(0..100usize);
    let mut result = String::with_capacity(len * 4);
    for _ in 0..len {
        result.push(rng.random_range('\0'..char::MAX));
    }
    result
}
