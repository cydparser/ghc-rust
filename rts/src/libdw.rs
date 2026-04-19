use crate::ffi::stg::types::{StgPtr, StgWord, StgWord32};
use crate::prelude::*;

#[cfg(test)]
mod tests;

const BACKTRACE_CHUNK_SZ: u32 = 256;

/// cbindgen:no-export
#[repr(C, packed)]
pub struct BacktraceChunk_ {
    pub(crate) n_frames: StgWord,
    pub(crate) next: *mut BacktraceChunk_,
    pub(crate) frames: [StgPtr; 256],
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
    pub(crate) object_file: *const c_char,
    pub(crate) function: *const c_char,
    pub(crate) source_file: *const c_char,
    pub(crate) lineno: StgWord32,
    pub(crate) colno: StgWord32,
}

#[ffi(compiler, ghc_lib)]
pub type Location = Location_;

#[ffi(ghc_lib)]
pub type LibdwSession = LibdwSession_;

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct LibdwSession_ {
    _unused: [u8; 0],
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn backtraceFree(mut bt: *mut Backtrace) {}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn libdwGetBacktrace(mut session: *mut LibdwSession) -> *mut Backtrace {
    return null_mut::<Backtrace>();
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn libdwLookupLocation(
    mut session: *mut LibdwSession,
    mut loc: *mut Location,
    mut pc: StgPtr,
) -> c_int {
    return 1;
}
