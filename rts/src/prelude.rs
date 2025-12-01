#![expect(unused_imports)]

pub use crate::utils::bindgen::__IncompleteArrayField;
pub(crate) use crate::utils::macros::sys;
#[cfg(test)]
pub use crate::utils::test::*;
pub use ghc_macros::{ffi, instrument};
#[cfg(feature = "sys")]
pub use ghc_rts_sys as sys;
#[cfg(test)]
pub use quickcheck_macros::quickcheck;
pub use std::ffi::{CStr, CString, c_char, c_int, c_uint, c_void};
pub use std::mem::{offset_of, transmute};
pub use std::ptr::{null, null_mut};

pub fn before_exit<T: AsRef<str>>(msg: T) {
    // TODO(rust) flush any logging
    eprintln!("Exiting due to {}", msg.as_ref());
}
