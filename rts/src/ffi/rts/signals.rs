use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(ghc_lib, libraries)]
pub const STG_SIG_DFL: i32 = -1;

#[ffi(ghc_lib, libraries)]
pub const STG_SIG_IGN: i32 = -2;

#[ffi(libraries)]
pub const STG_SIG_ERR: i32 = -3;

#[ffi(ghc_lib, libraries)]
pub const STG_SIG_HAN: i32 = -4;

#[ffi(ghc_lib, libraries)]
pub const STG_SIG_RST: i32 = -5;
