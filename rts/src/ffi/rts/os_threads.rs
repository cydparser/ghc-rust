use crate::ffi::hs_ffi::HsStablePtr;
use crate::ffi::stg::types::StgWord64;
use crate::prelude::*;

#[cfg(test)]
mod tests;

// TODO(rust): Use Rust's cross-platform, safe threading API.

#[ffi(compiler, testsuite)]
pub type Condition = TODO_;

#[ffi(testsuite)]
pub type Mutex = TODO_;

#[ffi(testsuite)]
pub type OSThreadId = TODO_;

/// cbindgen:no-export
#[repr(C)]
pub struct TODO_ {
    _unused: [u8; 0],
}

#[ffi(utils)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn shutdownThread() -> ! {
    before_exit("shutdownThread");
    sys! {
        shutdownThread()
    }
}

#[ffi(testsuite)]
pub type OSThreadProc = Option<unsafe extern "C" fn(arg1: *mut c_void) -> *mut c_void>;

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn createOSThread(
    tid: *mut OSThreadId,
    name: *const c_char,
    startProc: OSThreadProc,
    param: *mut c_void,
) -> c_int {
    sys! {
        createOSThread(tid as * mut sys::OSThreadId, name, startProc, param)
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn initCondition(pCond: *mut Condition) {
    sys! {
        initCondition(pCond as * mut sys::Condition)
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn broadcastCondition(pCond: *mut Condition) {
    sys! {
        broadcastCondition(pCond as * mut sys::Condition)
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn waitCondition(pCond: *mut Condition, pMut: *mut Mutex) {
    sys! {
        waitCondition(pCond as * mut sys::Condition, pMut as * mut sys::Mutex)
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn initMutex(pMut: *mut Mutex) {
    sys! {
        initMutex(pMut as * mut sys::Mutex)
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn forkOS_createThread(entry: HsStablePtr) -> c_int {
    sys! {
        forkOS_createThread(entry)
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getNumberOfProcessors() -> u32 {
    sys! {
        getNumberOfProcessors()
    }
}

pub(crate) type KernelThreadId = StgWord64;
