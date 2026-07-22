#![cfg(feature = "derive")]

/// HID-IO Packet Buffer Struct
///
/// # Remarks
/// Used to store HID-IO data chunks. Will be chunked into individual packets on transmission.
#[repr(C)]
#[derive(PartialEq, Eq, Clone, Debug, bincode::Encode)]
pub struct HidIoPacketBuffer<const H: usize> {
    /// Type of packet (Continued is automatically set if needed)
    pub ptype: u32,
    /// Packet Id
    pub id: u32,
    /// Packet length for serialization (in bytes)
    pub max_len: u32,
    /// Payload data, chunking is done automatically by serializer
    pub data: [u8; H],
    /// Set False if buffer is not complete, True if it is
    pub done: bool,
}

/// Reduced version of HidIoCommandId with non-contiguous discriminants.
/// Tests that bincode handles `#[repr(u32)]` enums with gaps in discriminant values.
#[repr(u32)]
#[derive(PartialEq, Eq, Clone, Copy, Debug, bincode::Encode)]
pub enum HidIoCommandId {
    SupportedIds = 0x00,
    GetInfo = 0x01,
    GetProperties = 0x10,
    OpenUrl = 0x30,
    HidKeyboard = 0x40,
    Unused = 0xFFFF,
}

#[test]
fn encode_const_generic_struct() {
    let buf = HidIoPacketBuffer::<4> {
        ptype: 1,
        id: 2,
        max_len: 4,
        data: [0xAA, 0xBB, 0xCC, 0xDD],
        done: true,
    };
    let encoded = bincode::encode_to_vec(&buf, bincode::config::standard()).unwrap();
    assert!(!encoded.is_empty());
}

#[test]
fn encode_repr_u32_enum() {
    let config = bincode::config::standard();
    for cmd in [
        HidIoCommandId::SupportedIds,
        HidIoCommandId::GetInfo,
        HidIoCommandId::GetProperties,
        HidIoCommandId::OpenUrl,
        HidIoCommandId::HidKeyboard,
        HidIoCommandId::Unused,
    ] {
        let encoded = bincode::encode_to_vec(cmd, config).unwrap();
        assert!(!encoded.is_empty());
    }
}
