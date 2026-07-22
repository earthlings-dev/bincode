#[cfg(feature = "alloc")]
mod impl_alloc;
#[cfg(feature = "alloc")]
pub use self::impl_alloc::*;

#[cfg(feature = "std")]
mod impl_std;
#[cfg(feature = "std")]
pub use self::impl_std::*;

#[cfg(feature = "derive")]
mod derive;
#[cfg(feature = "derive")]
pub use self::derive::*;

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub mod serde;

// Re-export v1-compatible serde API at the crate root.
#[cfg(feature = "serde")]
pub use self::serde::compat::DefaultOptions;
#[cfg(feature = "serde")]
pub use self::serde::compat::Options;
#[cfg(feature = "serde")]
pub use self::serde::compat::deserialize;
#[cfg(all(feature = "serde", feature = "std"))]
pub use self::serde::compat::deserialize_from;
#[cfg(feature = "serde")]
pub use self::serde::compat::deserialize_from_reader;
#[cfg(feature = "serde")]
pub use self::serde::compat::deserialize_in_place;
#[cfg(feature = "serde")]
pub use self::serde::compat::options;
#[cfg(all(feature = "serde", feature = "alloc"))]
pub use self::serde::compat::serialize;
#[cfg(all(feature = "serde", feature = "std"))]
pub use self::serde::compat::serialize_into;
#[cfg(all(feature = "serde", feature = "alloc"))]
pub use self::serde::compat::serialized_size;
