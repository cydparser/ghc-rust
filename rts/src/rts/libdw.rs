use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(test)]
use crate::utils::test::{Arbitrary, Gen, HasReferences};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use libc::{clockid_t, pid_t, pthread_cond_t, pthread_key_t, pthread_mutex_t, pthread_t};
use std::ffi::{c_char, c_int, c_uint, c_void};
use std::mem::transmute;
use std::ptr::{null, null_mut};
use std::slice;
#[cfg(feature = "tracing")]
use tracing::instrument;
#[cfg(test)]
mod tests;

pub(crate) const BACKTRACE_CHUNK_SZ: u32 = 256;

#[repr(C, packed)]
///cbindgen:no-export
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
#[derive(Clone)]
struct BacktraceChunk_Owned {
    pub n_frames: StgWord,
    pub frames: [StgPtr; 256usize],
}

#[cfg(test)]
impl Arbitrary for BacktraceChunk_Owned {
    fn arbitrary(g: &mut Gen) -> Self {
        BacktraceChunk_Owned {
            n_frames: Arbitrary::arbitrary(g),
            frames: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct BacktraceChunk_Pointees {
    pub next: BacktraceChunk_,
}

#[cfg(test)]
impl Arbitrary for BacktraceChunk_Pointees {
    fn arbitrary(g: &mut Gen) -> Self {
        BacktraceChunk_Pointees {
            next: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for BacktraceChunk_ {
    type Owned = BacktraceChunk_Owned;
    type Pointees = BacktraceChunk_Pointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            n_frames: owned.n_frames,
            frames: owned.frames,
            next: unsafe { &raw mut (*pointees).next },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            n_frames: self.n_frames,
            frames: self.frames,
        }
    }
}

pub type BacktraceChunk = BacktraceChunk_;

#[repr(C)]
///cbindgen:no-export
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
#[derive(Clone)]
struct Backtrace_Owned {
    pub n_frames: StgWord,
}

#[cfg(test)]
impl Arbitrary for Backtrace_Owned {
    fn arbitrary(g: &mut Gen) -> Self {
        Backtrace_Owned {
            n_frames: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct Backtrace_Pointees {
    pub last: BacktraceChunk,
}

#[cfg(test)]
impl Arbitrary for Backtrace_Pointees {
    fn arbitrary(g: &mut Gen) -> Self {
        Backtrace_Pointees {
            last: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for Backtrace_ {
    type Owned = Backtrace_Owned;
    type Pointees = Backtrace_Pointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            n_frames: owned.n_frames,
            last: unsafe { &raw mut (*pointees).last },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            n_frames: self.n_frames,
        }
    }
}

pub type Backtrace = Backtrace_;

#[repr(C, packed)]
///cbindgen:no-export
pub(crate) struct Location_ {
    pub object_file: *const c_char,
    pub function: *const c_char,
    pub source_file: *const c_char,
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
#[derive(Clone)]
struct Location_Owned {
    pub lineno: StgWord32,
    pub colno: StgWord32,
}

#[cfg(test)]
impl Arbitrary for Location_Owned {
    fn arbitrary(g: &mut Gen) -> Self {
        Location_Owned {
            lineno: Arbitrary::arbitrary(g),
            colno: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct Location_Pointees {
    pub object_file: c_char,
    pub function: c_char,
    pub source_file: c_char,
}

#[cfg(test)]
impl Arbitrary for Location_Pointees {
    fn arbitrary(g: &mut Gen) -> Self {
        Location_Pointees {
            object_file: Arbitrary::arbitrary(g),
            function: Arbitrary::arbitrary(g),
            source_file: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for Location_ {
    type Owned = Location_Owned;
    type Pointees = Location_Pointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            lineno: owned.lineno,
            colno: owned.colno,
            object_file: unsafe { &raw mut (*pointees).object_file },
            function: unsafe { &raw mut (*pointees).function },
            source_file: unsafe { &raw mut (*pointees).source_file },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            lineno: self.lineno,
            colno: self.colno,
        }
    }
}

pub type Location = Location_;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
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

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_backtraceFree"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn backtraceFree(bt: *mut Backtrace) {
    unsafe { sys::backtraceFree(bt as *mut sys::Backtrace) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_libdwGetBacktrace"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn libdwGetBacktrace(session: *mut LibdwSession) -> *mut Backtrace {
    unsafe { transmute(sys::libdwGetBacktrace(session as *mut sys::LibdwSession)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_libdwLookupLocation"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
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

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn libdwPrintBacktrace(
    session: *mut LibdwSession,
    file: *mut FILE,
    bt: *mut Backtrace,
) {
    unsafe {
        sys::libdwPrintBacktrace(
            session as *mut sys::LibdwSession,
            file as *mut sys::FILE,
            bt as *mut sys::Backtrace,
        )
    }
}
