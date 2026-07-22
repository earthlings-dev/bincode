use bincode::config::{
    AllowTrailing, BigEndian, Config, Endianness, Fixint, IntEncoding, Limit, LittleEndian,
    NativeEndian, NoLimit, RejectTrailing, Varint,
};

fn assert_marker_traits<T: Copy + Clone + core::fmt::Debug>() {}

fn assert_debug<T: core::fmt::Debug>(value: T) -> String {
    format!("{value:?}")
}

#[test]
fn configuration_markers_are_debug_unit_structs() {
    assert_marker_traits::<BigEndian>();
    assert_marker_traits::<LittleEndian>();
    assert_marker_traits::<NativeEndian>();
    assert_marker_traits::<Fixint>();
    assert_marker_traits::<Varint>();
    assert_marker_traits::<NoLimit>();
    assert_marker_traits::<Limit<16>>();
    assert_marker_traits::<AllowTrailing>();
    assert_marker_traits::<RejectTrailing>();

    let _ = BigEndian;
    let _ = LittleEndian;
    let _ = NativeEndian;
    let _ = Fixint;
    let _ = Varint;
    let _ = NoLimit;
    let _ = Limit::<16>;
    let _ = AllowTrailing;
    let _ = RejectTrailing;
}

#[test]
fn concrete_configurations_are_debug_without_changing_behavior() {
    let standard = bincode::config::standard();
    assert!(assert_debug(standard).starts_with("Configuration"));
    assert!(standard.endianness() == Endianness::Little);
    assert!(standard.int_encoding() == IntEncoding::Variable);
    assert!(standard.limit().is_none());
    assert!(!standard.reject_trailing());

    let legacy = bincode::config::legacy();
    assert!(assert_debug(legacy).starts_with("Configuration"));
    assert!(legacy.endianness() == Endianness::Little);
    assert!(legacy.int_encoding() == IntEncoding::Fixed);

    let big_endian = standard.with_big_endian();
    assert!(assert_debug(big_endian).starts_with("Configuration"));
    assert!(big_endian.endianness() == Endianness::Big);

    let little_endian = big_endian.with_little_endian();
    assert!(assert_debug(little_endian).starts_with("Configuration"));
    assert!(little_endian.endianness() == Endianness::Little);

    let native_endian = little_endian.with_native_endian();
    assert!(assert_debug(native_endian).starts_with("Configuration"));

    #[cfg(target_endian = "little")]
    assert!(native_endian.endianness() == Endianness::Little);
    #[cfg(target_endian = "big")]
    assert!(native_endian.endianness() == Endianness::Big);

    let fixed = native_endian.with_fixed_int_encoding();
    assert!(assert_debug(fixed).starts_with("Configuration"));
    assert!(fixed.int_encoding() == IntEncoding::Fixed);

    let variable = fixed.with_variable_int_encoding();
    assert!(assert_debug(variable).starts_with("Configuration"));
    assert!(variable.int_encoding() == IntEncoding::Variable);

    let limited = variable.with_limit::<16>();
    assert!(assert_debug(limited).starts_with("Configuration"));
    assert!(limited.limit() == Some(16));

    let unlimited = limited.with_no_limit();
    assert!(assert_debug(unlimited).starts_with("Configuration"));
    assert!(unlimited.limit().is_none());

    let rejecting = unlimited.reject_trailing_bytes();
    assert!(assert_debug(rejecting).starts_with("Configuration"));
    assert!(rejecting.reject_trailing());

    let allowing = rejecting.allow_trailing_bytes();
    assert!(assert_debug(allowing).starts_with("Configuration"));
    assert!(!allowing.reject_trailing());
}
