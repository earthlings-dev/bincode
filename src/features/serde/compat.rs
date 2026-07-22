//! Bincode v1-compatible serde API surface.
//!
//! This module provides the convenience functions and `Options` builder pattern
//! from bincode v1.3.3, built on top of bincode's current native architecture.

use crate::config::Config;
use crate::config::Configuration;
use crate::config::LittleEndian;
use crate::config::NoLimit;
use crate::config::RejectTrailing;
use crate::config::Varint;
use crate::de::BorrowDecoder;
use crate::de::read::Reader;
use crate::error::DecodeError;
use crate::error::EncodeError;

// ---------------------------------------------------------------------------
// DefaultOptions and options()
// ---------------------------------------------------------------------------

/// The default options type, equivalent to bincode v1's `DefaultOptions`.
///
/// Uses variable integer encoding, little-endian byte order, no byte limit,
/// and rejects trailing bytes in slice deserialization.
pub type DefaultOptions = Configuration<LittleEndian, Varint, NoLimit, RejectTrailing>;

/// Returns the default options for the `Options` builder pattern.
///
/// This matches bincode v1's `bincode::options()`, which used varint encoding
/// and rejected trailing bytes.
///
/// # Example
///
/// ```
/// # #[cfg(feature = "alloc")]
/// # {
/// use bincode::Options;
///
/// let val: u32 = 42;
/// let bytes = bincode::options().with_big_endian().serialize(&val).unwrap();
/// # }
/// ```
pub fn options() -> DefaultOptions {
  crate::config::standard().reject_trailing_bytes()
}

// ---------------------------------------------------------------------------
// Top-level convenience functions (legacy config: fixint + little-endian + allow trailing)
// ---------------------------------------------------------------------------

/// Serialize a value using the legacy configuration (fixint + little-endian).
///
/// This is equivalent to bincode v1's `bincode::serialize()`.
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub fn serialize<T: ::serde::Serialize>(value: &T) -> Result<alloc::vec::Vec<u8>, EncodeError> {
  super::encode_to_vec(value, crate::config::legacy())
}

/// Serialize a value into a writer using the legacy configuration.
///
/// This is equivalent to bincode v1's `bincode::serialize_into()`.
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub fn serialize_into<W: std::io::Write, T: ::serde::Serialize>(mut writer: W, value: &T) -> Result<(), EncodeError> {
  super::encode_into_std_write(value, &mut writer, crate::config::legacy())?;
  Ok(())
}

/// Compute the serialized size of a value using the legacy configuration.
///
/// This is equivalent to bincode v1's `bincode::serialized_size()`.
pub fn serialized_size<T: ::serde::Serialize>(value: &T) -> Result<u64, EncodeError> {
  let mut size_writer = crate::enc::write::SizeWriter::default();
  super::encode_into_writer(value, &mut size_writer, crate::config::legacy())?;
  Ok(size_writer.bytes_written as u64)
}

/// Deserialize a value from a byte slice using the legacy configuration.
///
/// This is equivalent to bincode v1's `bincode::deserialize()`.
/// Trailing bytes are allowed (matching v1 behavior).
pub fn deserialize<'a, T: ::serde::Deserialize<'a>>(bytes: &'a [u8]) -> Result<T, DecodeError> {
  let (val, _) = super::borrow_decode_from_slice(bytes, crate::config::legacy())?;
  Ok(val)
}

/// Deserialize a value from a `std::io::Read` implementor using the legacy configuration.
///
/// This is equivalent to bincode v1's `bincode::deserialize_from()`.
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub fn deserialize_from<R: std::io::Read, T: ::serde::de::DeserializeOwned>(mut reader: R) -> Result<T, DecodeError> {
  super::decode_from_std_read(&mut reader, crate::config::legacy())
}

/// Deserialize a value from a [`Reader`] using the legacy configuration.
///
/// This is the bincode [`Reader`] counterpart to v1's `bincode::deserialize_from_custom()`.
pub fn deserialize_from_reader<R: Reader, T: ::serde::de::DeserializeOwned>(reader: R) -> Result<T, DecodeError> {
  super::decode_from_reader(reader, crate::config::legacy())
}

/// Deserialize a value in place from a byte slice using the legacy configuration.
///
/// Trailing bytes are allowed (matching v1 behavior).
pub fn deserialize_in_place<'a, T: ::serde::Deserialize<'a>>(bytes: &'a [u8], place: &mut T) -> Result<(), DecodeError> {
  let mut serde_decoder = super::de_borrowed::BorrowedSerdeDecoder::from_slice(bytes, crate::config::legacy(), ());
  ::serde::Deserialize::deserialize_in_place(serde_decoder.as_deserializer(), place)
}

// ---------------------------------------------------------------------------
// Options trait
// ---------------------------------------------------------------------------

/// Trait providing bincode v1-compatible `serialize`/`deserialize` methods.
///
/// This trait is blanket-implemented for all types implementing [`Config`].
/// Use [`options()`] to get a `DefaultOptions` value, or
/// [`config::legacy()`](crate::config::legacy) for the v1-default configuration (fixint +
/// little-endian).
///
/// # Trailing bytes
///
/// Slice-based deserialization methods (`deserialize`, `deserialize_seed`,
/// `deserialize_in_place`) check the trailing bytes policy from the config.
/// Use [`.reject_trailing_bytes()`](Configuration::reject_trailing_bytes) or
/// [`.allow_trailing_bytes()`](Configuration::allow_trailing_bytes) to control this.
///
/// # Example
///
/// ```
/// # #[cfg(feature = "alloc")]
/// # {
/// use bincode::Options;
///
/// let val: u32 = 42;
/// let bytes = bincode::options()
///   .with_big_endian()
///   .with_fixed_int_encoding()
///   .serialize(&val)
///   .unwrap();
///
/// let decoded: u32 = bincode::options()
///   .with_big_endian()
///   .with_fixed_int_encoding()
///   .deserialize(&bytes)
///   .unwrap();
///
/// assert_eq!(val, decoded);
/// # }
/// ```
pub trait Options: Config + Sized {
  /// Serialize a value to a `Vec<u8>`.
  #[cfg(feature = "alloc")]
  fn serialize<T: ::serde::Serialize>(&self, value: &T) -> Result<alloc::vec::Vec<u8>, EncodeError> {
    super::encode_to_vec(value, *self)
  }

  /// Serialize a value into a `std::io::Write` implementor.
  #[cfg(feature = "std")]
  fn serialize_into<W: std::io::Write, T: ::serde::Serialize>(&self, mut writer: W, value: &T) -> Result<(), EncodeError> {
    super::encode_into_std_write(value, &mut writer, *self)?;
    Ok(())
  }

  /// Compute the serialized size of a value in bytes.
  fn serialized_size<T: ::serde::Serialize>(&self, value: &T) -> Result<u64, EncodeError> {
    let mut size_writer = crate::enc::write::SizeWriter::default();
    super::encode_into_writer(value, &mut size_writer, *self)?;
    Ok(size_writer.bytes_written as u64)
  }

  /// Deserialize a value from a byte slice.
  ///
  /// If this config rejects trailing bytes, an error is returned when not all
  /// bytes in the slice are consumed.
  fn deserialize<'a, T: ::serde::Deserialize<'a>>(&self, bytes: &'a [u8]) -> Result<T, DecodeError> {
    let (val, bytes_read) = super::borrow_decode_from_slice(bytes, *self)?;
    if self.reject_trailing() && bytes_read != bytes.len() {
      return Err(DecodeError::Other("Slice had bytes remaining after deserialization"));
    }
    Ok(val)
  }

  /// Deserialize a value from a byte slice using a `DeserializeSeed`.
  ///
  /// If this config rejects trailing bytes, an error is returned when not all
  /// bytes in the slice are consumed.
  fn deserialize_seed<'a, T: ::serde::de::DeserializeSeed<'a>>(&self, seed: T, bytes: &'a [u8]) -> Result<T::Value, DecodeError> {
    let (val, bytes_read) = super::seed_decode_from_slice(seed, bytes, *self)?;
    if self.reject_trailing() && bytes_read != bytes.len() {
      return Err(DecodeError::Other("Slice had bytes remaining after deserialization"));
    }
    Ok(val)
  }

  /// Deserialize a value from a `std::io::Read` implementor.
  #[cfg(feature = "std")]
  fn deserialize_from<R: std::io::Read, T: ::serde::de::DeserializeOwned>(&self, mut reader: R) -> Result<T, DecodeError> {
    super::decode_from_std_read(&mut reader, *self)
  }

  /// Deserialize a value from a `std::io::Read` implementor using a `DeserializeSeed`.
  #[cfg(feature = "std")]
  fn deserialize_from_seed<'a, R: std::io::Read, T: ::serde::de::DeserializeSeed<'a>>(
    &self,
    seed: T,
    mut reader: R,
  ) -> Result<T::Value, DecodeError> {
    super::seed_decode_from_std_read(seed, &mut reader, *self)
  }

  /// Deserialize a value from a [`Reader`].
  ///
  /// This is the bincode [`Reader`] counterpart to v1's `deserialize_from_custom()`.
  fn deserialize_from_reader<R: Reader, T: ::serde::de::DeserializeOwned>(&self, reader: R) -> Result<T, DecodeError> {
    super::decode_from_reader(reader, *self)
  }

  /// Deserialize a value from a [`Reader`] using a `DeserializeSeed`.
  fn deserialize_from_reader_seed<'a, R: Reader, T: ::serde::de::DeserializeSeed<'a>>(
    &self,
    seed: T,
    reader: R,
  ) -> Result<T::Value, DecodeError> {
    super::seed_decode_from_reader(seed, reader, *self)
  }

  /// Deserialize a value in place from a byte slice.
  ///
  /// If this config rejects trailing bytes, an error is returned when not all
  /// bytes in the slice are consumed.
  fn deserialize_in_place<'a, T: ::serde::Deserialize<'a>>(&self, bytes: &'a [u8], place: &mut T) -> Result<(), DecodeError> {
    let mut serde_decoder = super::de_borrowed::BorrowedSerdeDecoder::from_slice(bytes, *self, ());
    ::serde::Deserialize::deserialize_in_place(serde_decoder.as_deserializer(), place)?;
    if self.reject_trailing() {
      let bytes_read = bytes.len() - serde_decoder.de.borrow_reader().slice.len();
      if bytes_read != bytes.len() {
        return Err(DecodeError::Other("Slice had bytes remaining after deserialization"));
      }
    }
    Ok(())
  }
}

impl<C: Config> Options for C {}

#[cfg(feature = "alloc")]
extern crate alloc;
