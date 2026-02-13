#![no_main]
use libfuzzer_sys::fuzz_target;

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::ffi::CString;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::num::{NonZeroI128, NonZeroI32, NonZeroU128, NonZeroU32};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use bincode::Options;

#[derive(
    bincode::Decode,
    bincode::Encode,
    PartialEq,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    Eq,
    PartialOrd,
    Ord,
)]
enum AllTypes {
    BTreeMap(BTreeMap<u8, AllTypes>),
    BTreeSet(BTreeSet<AllTypes>),
    VecDeque(VecDeque<AllTypes>),
    Vec(Vec<u8>),
    String(String),
    Box(Box<u8>),
    BoxSlice(Box<[u8]>),
    CString(CString),
    SystemTime(SystemTime),
    Duration(Duration),
    PathBuf(PathBuf),
    IpAddr(IpAddr),
    Ipv4Addr(Ipv4Addr),
    Ipv6Addr(Ipv6Addr),
    SocketAddr(SocketAddr),
    SocketAddrV4(SocketAddrV4),
    SocketAddrV6(SocketAddrV6),
    NonZeroU32(NonZeroU32),
    NonZeroI32(NonZeroI32),
    NonZeroU128(NonZeroU128),
    NonZeroI128(NonZeroI128),
    I128(i128),
    I8(i8),
    U128(u128),
    U8(u8),
}

fuzz_target!(|data: &[u8]| {
    let config = bincode::config::legacy().with_limit::<1024>();

    // v1 compat API (Options::deserialize uses serde internally)
    let compat_result: Result<AllTypes, _> = config.deserialize(data);
    // v2 native API (decode_from_slice uses bincode::Decode)
    let native_result: Result<(AllTypes, _), _> = bincode::decode_from_slice(data, config);

    match (&compat_result, &native_result) {
        // Either hitting the limit is fine
        (Err(bincode::error::DecodeError::LimitExceeded), _)
        | (_, Err(bincode::error::DecodeError::LimitExceeded)) => {}
        // Both succeed — values must match
        (Ok(compat_val), Ok((native_val, _))) if compat_val != native_val => {
            println!("Bytes:        {:?}", data);
            println!("v1 compat:    {:?}", compat_val);
            println!("v2 native:    {:?}", native_val);
            panic!("v1 compat and v2 native decoded different values");
        }
        // One succeeds, one fails — mismatch
        (Ok(_), Err(_)) | (Err(_), Ok(_)) => {
            println!("Bytes:        {:?}", data);
            println!("v1 compat:    {:?}", compat_result);
            println!("v2 native:    {:?}", native_result);
            panic!("one API succeeded while the other failed");
        }
        // Both succeed with equal values, or both fail — fine
        _ => {}
    }
});
