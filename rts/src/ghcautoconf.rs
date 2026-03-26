use crate::prelude::*;

pub(crate) const RTS_LINKER_USE_MMAP: c_int = 1 as c_int;

#[ffi(ghc_lib)]
pub const SIZEOF_LONG: c_int = 8 as c_int;

#[ffi(ghc_lib)]
pub const SIZEOF_VOID_P: c_int = 8 as c_int;
