use core::error::Error as _;

use bincode::error::DecodeError;
use bincode::error::EncodeError;

fn assert_core_error<T: core::error::Error>() {}

#[test]
fn public_errors_implement_core_error() {
  assert_core_error::<EncodeError>();
  assert_core_error::<DecodeError>();
}

#[test]
fn core_error_sources_are_preserved() {
  let cell = core::cell::RefCell::new(());
  let borrowed = cell.borrow_mut();
  let inner = cell.try_borrow().unwrap_err();
  let error = EncodeError::RefCellAlreadyBorrowed {
    inner,
    type_name: "()",
  };
  assert!(error.source().unwrap().downcast_ref::<core::cell::BorrowError>().is_some());
  drop(borrowed);

  let bytes = core::hint::black_box([0xFF]);
  let inner = core::str::from_utf8(&bytes).unwrap_err();
  let error = DecodeError::Utf8 {
    inner,
  };
  assert!(error.source().unwrap().downcast_ref::<core::str::Utf8Error>().is_some());

  assert!(EncodeError::UnexpectedEnd.source().is_none());
  assert!(EncodeError::Other("opaque").source().is_none());
  assert!(DecodeError::LimitExceeded.source().is_none());
  assert!(DecodeError::Other("opaque").source().is_none());

  #[cfg(feature = "alloc")]
  {
    assert!(EncodeError::OtherString("opaque".into()).source().is_none());
    assert!(DecodeError::OtherString("opaque".into()).source().is_none());
  }

  #[cfg(feature = "serde")]
  {
    let error = EncodeError::Serde(bincode::serde::EncodeError::SequenceMustHaveLength);
    assert!(error.source().is_none());

    let error = DecodeError::Serde(bincode::serde::DecodeError::AnyNotSupported);
    assert!(error.source().is_none());
  }
}

#[cfg(feature = "std")]
#[test]
fn std_error_sources_are_preserved_through_core_error() {
  let error = EncodeError::Io {
    inner: std::io::Error::other("io failure"), index: 0
  };
  assert!(error.source().unwrap().downcast_ref::<std::io::Error>().is_some());

  let before_epoch = std::time::UNIX_EPOCH
    .duration_since(std::time::UNIX_EPOCH + std::time::Duration::from_secs(1))
    .unwrap_err();
  let error = EncodeError::InvalidSystemTime {
    inner: before_epoch, time: Box::new(std::time::UNIX_EPOCH)
  };
  assert!(error.source().unwrap().downcast_ref::<std::time::SystemTimeError>().is_some());

  let error = EncodeError::LockFailed {
    type_name: "std::sync::Mutex<()>"
  };
  assert!(error.source().is_none());

  let error = DecodeError::Io {
    inner: std::io::Error::other("opaque decode I/O failure"), additional: 1
  };
  assert!(error.source().is_none());
}
