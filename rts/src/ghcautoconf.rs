use crate::prelude::*;

pub(crate) const RTS_LINKER_USE_MMAP: i32 = 1;

#[ffi(ghc_lib)]
pub const SIZEOF_LONG: c_int = 8;

#[ffi(ghc_lib)]
pub const SIZEOF_VOID_P: c_int = 8;
