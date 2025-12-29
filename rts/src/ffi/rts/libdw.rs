use crate::ffi::stg::types::{StgPtr, StgWord, StgWord32};
use crate::prelude::*;

#[cfg(test)]
mod tests;

pub(crate) const BACKTRACE_CHUNK_SZ: u32 = 256;

/// cbindgen:no-export
#[repr(C, packed)]
pub struct BacktraceChunk_ {
    n_frames: StgWord,
    next: *mut BacktraceChunk_,
    frames: [StgPtr; 256usize],
}

#[ffi(ghc_lib)]
pub type BacktraceChunk = BacktraceChunk_;

#[ffi(compiler)]
#[repr(C)]
pub struct Backtrace_ {
    pub n_frames: StgWord,
    pub last: *mut BacktraceChunk,
}

#[ffi(ghc_lib)]
pub type Backtrace = Backtrace_;

/// cbindgen:no-export
#[repr(C, packed)]
pub struct Location_ {
    object_file: *const c_char,
    function: *const c_char,
    source_file: *const c_char,
    lineno: StgWord32,
    colno: StgWord32,
}

#[ffi(compiler, ghc_lib)]
pub type Location = Location_;

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct LibdwSession_ {
    _unused: [u8; 0],
}

pub(crate) type LibdwSession = LibdwSession_;

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn backtraceFree(bt: *mut Backtrace) {
    sys! {
        backtraceFree(bt as * mut sys::Backtrace)
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn libdwGetBacktrace(session: *mut LibdwSession) -> *mut Backtrace {
    sys! {
        libdwGetBacktrace(session as * mut sys::LibdwSession).cast()
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn libdwLookupLocation(
    session: *mut LibdwSession,
    loc: *mut Location,
    pc: StgPtr,
) -> c_int {
    sys! {
        libdwLookupLocation(session as * mut sys::LibdwSession, loc as * mut
        sys::Location, pc)
    }
}
