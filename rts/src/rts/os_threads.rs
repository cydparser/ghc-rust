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

#[repr(C)]
pub struct Condition {
    pub cond: pthread_cond_t,
    pub timeout_clk: clockid_t,
}

#[cfg(feature = "sys")]
impl From<Condition> for sys::Condition {
    fn from(x: Condition) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for Condition {
    fn arbitrary(g: &mut Gen) -> Self {
        Condition {
            cond: Arbitrary::arbitrary(g),
            timeout_clk: Arbitrary::arbitrary(g),
        }
    }
}

pub type Mutex = pthread_mutex_t;

pub type OSThreadId = pthread_t;

pub(crate) type ThreadLocalKey = pthread_key_t;

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn osThreadId() -> OSThreadId {
    unsafe { sys::osThreadId() }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn shutdownThread() -> ! {
    unsafe { sys::shutdownThread() }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn yieldThread() {
    unsafe { sys::yieldThread() }
}

pub type OSThreadProc = Option<unsafe extern "C" fn(arg1: *mut c_void) -> *mut c_void>;
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_createOSThread"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn createOSThread(
    tid: *mut OSThreadId,
    name: *const c_char,
    startProc: OSThreadProc,
    param: *mut c_void,
) -> c_int {
    unsafe { sys::createOSThread(tid, name, startProc, param) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn createAttachedOSThread(
    tid: *mut OSThreadId,
    name: *const c_char,
    startProc: OSThreadProc,
    param: *mut c_void,
) -> c_int {
    unsafe { sys::createAttachedOSThread(tid, name, startProc, param) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn osThreadIsAlive(id: OSThreadId) -> bool {
    unsafe { transmute(sys::osThreadIsAlive(id)) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn interruptOSThread(id: OSThreadId) {
    unsafe { sys::interruptOSThread(id) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn joinOSThread(id: OSThreadId) {
    unsafe { sys::joinOSThread(id) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_initCondition"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn initCondition(pCond: *mut Condition) {
    unsafe { sys::initCondition(pCond as *mut sys::Condition) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn closeCondition(pCond: *mut Condition) {
    unsafe { sys::closeCondition(pCond as *mut sys::Condition) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_broadcastCondition"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn broadcastCondition(pCond: *mut Condition) {
    unsafe { sys::broadcastCondition(pCond as *mut sys::Condition) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn signalCondition(pCond: *mut Condition) {
    unsafe { sys::signalCondition(pCond as *mut sys::Condition) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_waitCondition"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn waitCondition(pCond: *mut Condition, pMut: *mut Mutex) {
    unsafe { sys::waitCondition(pCond as *mut sys::Condition, pMut) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn timedWaitCondition(
    pCond: *mut Condition,
    pMut: *mut Mutex,
    timeout: Time,
) -> bool {
    unsafe {
        transmute(sys::timedWaitCondition(
            pCond as *mut sys::Condition,
            pMut,
            timeout,
        ))
    }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_initMutex"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn initMutex(pMut: *mut Mutex) {
    unsafe { sys::initMutex(pMut) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn closeMutex(pMut: *mut Mutex) {
    unsafe { sys::closeMutex(pMut) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn newThreadLocalKey(key: *mut ThreadLocalKey) {
    unsafe { sys::newThreadLocalKey(key) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn getThreadLocalVar(key: *mut ThreadLocalKey) -> *mut c_void {
    unsafe { sys::getThreadLocalVar(key) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn setThreadLocalVar(key: *mut ThreadLocalKey, value: *mut c_void) {
    unsafe { sys::setThreadLocalVar(key, value) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn freeThreadLocalKey(key: *mut ThreadLocalKey) {
    unsafe { sys::freeThreadLocalKey(key) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn setThreadAffinity(n: u32, m: u32) {
    unsafe { sys::setThreadAffinity(n, m) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn setThreadNode(node: u32) {
    unsafe { sys::setThreadNode(node) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn releaseThreadNode() {
    unsafe { sys::releaseThreadNode() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_forkOS_createThread"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn forkOS_createThread(entry: HsStablePtr) -> c_int {
    unsafe { sys::forkOS_createThread(entry) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn freeThreadingResources() {
    unsafe { sys::freeThreadingResources() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_getNumberOfProcessors"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getNumberOfProcessors() -> u32 {
    unsafe { sys::getNumberOfProcessors() }
}

pub(crate) type KernelThreadId = StgWord64;

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn kernelThreadId() -> KernelThreadId {
    unsafe { sys::kernelThreadId() }
}
