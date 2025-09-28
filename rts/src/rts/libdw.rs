use crate::prelude::*;
use crate::stg::types::{StgPtr, StgWord, StgWord32};

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

#[cfg(feature = "sys")]
impl From<BacktraceChunk_> for sys::BacktraceChunk_ {
    fn from(x: BacktraceChunk_) -> Self {
        unsafe { transmute(x) }
    }
}

/// - GHC_PLACES: {libraries}
pub type BacktraceChunk = BacktraceChunk_;

/// cbindgen:no-export
#[repr(C)]
pub struct Backtrace_ {
    n_frames: StgWord,
    last: *mut BacktraceChunk,
}

#[cfg(feature = "sys")]
impl From<Backtrace_> for sys::Backtrace_ {
    fn from(x: Backtrace_) -> Self {
        unsafe { transmute(x) }
    }
}

/// - GHC_PLACES: {libraries}
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

#[cfg(feature = "sys")]
impl From<Location_> for sys::Location_ {
    fn from(x: Location_) -> Self {
        unsafe { transmute(x) }
    }
}

/// - GHC_PLACES: {libraries}
pub type Location = Location_;

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct LibdwSession_ {
    _unused: [u8; 0],
}

#[cfg(feature = "sys")]
impl From<LibdwSession_> for sys::LibdwSession_ {
    fn from(x: LibdwSession_) -> Self {
        unsafe { transmute(x) }
    }
}

/// - GHC_PLACES: {libraries}
pub type LibdwSession = LibdwSession_;

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_backtraceFree"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn backtraceFree(bt: *mut Backtrace) {
    unsafe { sys::backtraceFree(bt as *mut sys::Backtrace) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_libdwGetBacktrace"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn libdwGetBacktrace(session: *mut LibdwSession) -> *mut Backtrace {
    unsafe { sys::libdwGetBacktrace(session as *mut sys::LibdwSession) as *mut Backtrace }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_libdwLookupLocation"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn libdwLookupLocation(
    session: *mut LibdwSession,
    loc: *mut Location,
    pc: StgPtr,
) -> c_int {
    unsafe {
        sys::libdwLookupLocation(
            session as *mut sys::LibdwSession,
            loc as *mut sys::Location,
            pc,
        )
    }
}
