#![cfg(test)]

use ::rand::RngExt;
use bincode::Options;

mod membership;
mod misc;
mod rand;
mod sway;

/// Test that all three API surfaces (native encode/decode, serde encode/decode,
/// and v1-compatible serialize/deserialize) produce identical output for a given value
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
    // v1-compatible API (Options trait — serialize/deserialize)
    let compat_encoded = config.serialize(t).unwrap();

    println!("Encoded {t:?} as {compat_encoded:?}");

    // Native API (encode_to_vec)
    let native_encoded = bincode::encode_to_vec(t, config).unwrap();
    assert_eq!(
        compat_encoded,
        native_encoded,
        "{t:?} encodes differently between the v1-compatible and native APIs\nbincode config {:?}",
        core::any::type_name::<C>(),
    );

    // Serde API (serde::encode_to_vec)
    let serde_encoded = bincode::serde::encode_to_vec(t, config).unwrap();
    assert_eq!(
        compat_encoded, serde_encoded,
        "{t:?} encodes differently between the v1-compatible and serde APIs"
    );

    // Deserialize via the v1-compatible API
    let compat_decoded: T = config.deserialize(&compat_encoded).unwrap();
    assert_eq!(&compat_decoded, t);

    // Deserialize via the native API
    let native_decoded: T = bincode::decode_from_slice(&compat_encoded, config)
        .unwrap()
        .0;
    assert_eq!(&native_decoded, t);

    // Deserialize via the serde API
    let serde_decoded: T = bincode::serde::decode_from_slice(&compat_encoded, config)
        .unwrap()
        .0;
    assert_eq!(&serde_decoded, t);
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
