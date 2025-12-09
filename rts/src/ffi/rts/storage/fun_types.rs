use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(compiler, ghc_lib)]
pub const ARG_GEN: u32 = 0;

#[ffi(compiler, ghc_lib)]
pub const ARG_GEN_BIG: u32 = 1;

#[ffi(compiler)]
pub const ARG_BCO: u32 = 2;

#[ffi(compiler)]
pub const ARG_NONE: u32 = 3;

#[ffi(compiler)]
pub const ARG_N: u32 = 4;

#[ffi(compiler)]
pub const ARG_P: u32 = 5;

#[ffi(compiler)]
pub const ARG_F: u32 = 6;

#[ffi(compiler)]
pub const ARG_D: u32 = 7;

#[ffi(compiler)]
pub const ARG_L: u32 = 8;

#[ffi(compiler)]
pub const ARG_V16: u32 = 9;

#[ffi(compiler)]
pub const ARG_V32: u32 = 10;

#[ffi(compiler)]
pub const ARG_V64: u32 = 11;

#[ffi(compiler)]
pub const ARG_NN: u32 = 12;

#[ffi(compiler)]
pub const ARG_NP: u32 = 13;

#[ffi(compiler)]
pub const ARG_PN: u32 = 14;

#[ffi(compiler)]
pub const ARG_PP: u32 = 15;

#[ffi(compiler)]
pub const ARG_NNN: u32 = 16;

#[ffi(compiler)]
pub const ARG_NNP: u32 = 17;

#[ffi(compiler)]
pub const ARG_NPN: u32 = 18;

#[ffi(compiler)]
pub const ARG_NPP: u32 = 19;

#[ffi(compiler)]
pub const ARG_PNN: u32 = 20;

#[ffi(compiler)]
pub const ARG_PNP: u32 = 21;

#[ffi(compiler)]
pub const ARG_PPN: u32 = 22;

#[ffi(compiler)]
pub const ARG_PPP: u32 = 23;

#[ffi(compiler)]
pub const ARG_PPPP: u32 = 24;

#[ffi(compiler)]
pub const ARG_PPPPP: u32 = 25;

#[ffi(compiler)]
pub const ARG_PPPPPP: u32 = 26;

#[ffi(compiler)]
pub const ARG_PPPPPPP: u32 = 27;

#[ffi(compiler)]
pub const ARG_PPPPPPPP: u32 = 28;
