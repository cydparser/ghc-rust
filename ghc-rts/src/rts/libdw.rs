use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use libc::{clockid_t, pid_t, pthread_cond_t, pthread_key_t, pthread_mutex_t, pthread_t};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use std::mem::transmute;
#[cfg(feature = "tracing")]
use tracing::instrument;
#[cfg(test)]
mod tests;

pub(crate) const BACKTRACE_CHUNK_SZ: u32 = 256;

#[repr(C, packed)]
pub(crate) struct BacktraceChunk_ {
    pub n_frames: StgWord,
    pub next: *mut BacktraceChunk_,
    pub frames: [StgPtr; 256usize],
}

#[cfg(feature = "sys")]
impl From<BacktraceChunk_> for sys::BacktraceChunk_ {
    fn from(x: BacktraceChunk_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for BacktraceChunk_ {
    fn arbitrary(g: &mut Gen) -> Self {
        BacktraceChunk_ {
            n_frames: Arbitrary::arbitrary(g),
            next: Arbitrary::arbitrary(g),
            frames: Arbitrary::arbitrary(g),
        }
    }
}

pub type BacktraceChunk = BacktraceChunk_;

#[repr(C)]
pub(crate) struct Backtrace_ {
    pub n_frames: StgWord,
    pub last: *mut BacktraceChunk,
}

#[cfg(feature = "sys")]
impl From<Backtrace_> for sys::Backtrace_ {
    fn from(x: Backtrace_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for Backtrace_ {
    fn arbitrary(g: &mut Gen) -> Self {
        Backtrace_ {
            n_frames: Arbitrary::arbitrary(g),
            last: Arbitrary::arbitrary(g),
        }
    }
}

pub type Backtrace = Backtrace_;

#[repr(C, packed)]
pub(crate) struct Location_ {
    pub object_file: *const ::core::ffi::c_char,
    pub function: *const ::core::ffi::c_char,
    pub source_file: *const ::core::ffi::c_char,
    pub lineno: StgWord32,
    pub colno: StgWord32,
}

#[cfg(feature = "sys")]
impl From<Location_> for sys::Location_ {
    fn from(x: Location_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for Location_ {
    fn arbitrary(g: &mut Gen) -> Self {
        Location_ {
            object_file: Arbitrary::arbitrary(g),
            function: Arbitrary::arbitrary(g),
            source_file: Arbitrary::arbitrary(g),
            lineno: Arbitrary::arbitrary(g),
            colno: Arbitrary::arbitrary(g),
        }
    }
}

pub type Location = Location_;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct LibdwSession_ {
    _unused: [u8; 0],
}

#[cfg(feature = "sys")]
impl From<LibdwSession_> for sys::LibdwSession_ {
    fn from(x: LibdwSession_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for LibdwSession_ {
    fn arbitrary(g: &mut Gen) -> Self {
        LibdwSession_ {
            _unused: Arbitrary::arbitrary(g),
        }
    }
}

pub type LibdwSession = LibdwSession_;

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn backtraceFree(bt: *mut Backtrace) {
    unsafe { sys::backtraceFree(bt) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn libdwGetBacktrace(session: *mut LibdwSession) -> *mut Backtrace {
    unsafe { transmute(sys::libdwGetBacktrace(session)) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn libdwLookupLocation(
    session: *mut LibdwSession,
    loc: *mut Location,
    pc: StgPtr,
) -> ::core::ffi::c_int {
    unsafe { transmute(sys::libdwLookupLocation(session, loc, pc)) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn libdwPrintBacktrace(
    session: *mut LibdwSession,
    file: *mut FILE,
    bt: *mut Backtrace,
) {
    unsafe { sys::libdwPrintBacktrace(session, file, bt) }
}
