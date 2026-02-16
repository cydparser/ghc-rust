use crate::ffi::stg::W_;
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(testsuite)]
#[unsafe(no_mangle)]
pub static mut mblocks_allocated: W_ = 0;
