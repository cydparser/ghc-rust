use crate::prelude::*;

#[ffi(compiler)]
pub const HOST_ARCH: [c_char; 8] = unsafe { transmute::<[u8; 8], [c_char; 8]>(*b"aarch64\0") };

#[ffi(compiler)]
pub const HOST_OS: [c_char; 7] = unsafe { transmute::<[u8; 7], [c_char; 7]>(*b"darwin\0") };

pub(crate) const HOST_VENDOR: [c_char; 6] =
    unsafe { transmute::<[u8; 6], [c_char; 6]>(*b"apple\0") };
