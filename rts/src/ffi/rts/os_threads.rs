use crate::ffi::hs_ffi::HsStablePtr;
use crate::ffi::stg::types::StgWord64;
use crate::prelude::*;
use libc::{pthread_cond_t, pthread_mutex_t, pthread_t};

#[cfg(test)]
mod tests;

/// - GHC_PLACES: {testsuite}
#[repr(C)]
pub struct Condition {
    pub cond: pthread_cond_t,
}

#[cfg(feature = "sys")]
impl From<Condition> for sys::Condition {
    fn from(x: Condition) -> Self {
        unsafe { transmute(x) }
    }
}

/// - GHC_PLACES: {testsuite}
pub type Mutex = pthread_mutex_t;

/// - GHC_PLACES: {testsuite}
pub type OSThreadId = pthread_t;

/// - GHC_PLACES: {utils}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn shutdownThread() -> ! {
    #[cfg(feature = "sys")]
    unsafe {
        sys::shutdownThread()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("shutdownThread")
}

/// - GHC_PLACES: {testsuite}
pub type OSThreadProc = Option<unsafe extern "C" fn(arg1: *mut c_void) -> *mut c_void>;
#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn createOSThread(
    tid: *mut OSThreadId,
    name: *const c_char,
    startProc: OSThreadProc,
    param: *mut c_void,
) -> c_int {
    #[cfg(feature = "sys")]
    unsafe {
        sys::createOSThread(tid as *mut sys::OSThreadId, name, startProc, param)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("createOSThread")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn initCondition(pCond: *mut Condition) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::initCondition(pCond as *mut sys::Condition)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("initCondition")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn broadcastCondition(pCond: *mut Condition) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::broadcastCondition(pCond as *mut sys::Condition)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("broadcastCondition")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn waitCondition(pCond: *mut Condition, pMut: *mut Mutex) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::waitCondition(pCond as *mut sys::Condition, pMut as *mut sys::Mutex)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("waitCondition")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn initMutex(pMut: *mut Mutex) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::initMutex(pMut as *mut sys::Mutex)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("initMutex")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn forkOS_createThread(entry: HsStablePtr) -> c_int {
    #[cfg(feature = "sys")]
    unsafe {
        sys::forkOS_createThread(entry)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("forkOS_createThread")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getNumberOfProcessors() -> u32 {
    #[cfg(feature = "sys")]
    unsafe {
        sys::getNumberOfProcessors()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("getNumberOfProcessors")
}

pub(crate) type KernelThreadId = StgWord64;
