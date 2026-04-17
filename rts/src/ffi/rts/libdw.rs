pub use crate::libdw::{
    Backtrace, Backtrace_, BacktraceChunk, BacktraceChunk_, Location, Location_, backtraceFree,
    libdwGetBacktrace, libdwLookupLocation,
};
use crate::prelude::*;

pub(crate) const BACKTRACE_CHUNK_SZ: u32 = 256;

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct LibdwSession_ {
    _unused: [u8; 0],
}
