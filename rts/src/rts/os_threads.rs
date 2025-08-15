use crate::hs_ffi::HsStablePtr;
use crate::prelude::*;
use crate::rts::time::Time;
use crate::stg::types::StgWord64;
use libc::{pthread_cond_t, pthread_key_t, pthread_mutex_t, pthread_t};

#[cfg(test)]
mod tests;

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

pub type Mutex = pthread_mutex_t;

pub type OSThreadId = pthread_t;

pub(crate) type ThreadLocalKey = pthread_key_t;

#[instrument]
pub(crate) unsafe fn osThreadId() -> OSThreadId {
    unsafe { sys::osThreadId() }
}

#[instrument]
pub(crate) unsafe fn shutdownThread() -> ! {
    unsafe { sys::shutdownThread() }
}

#[instrument]
pub(crate) unsafe fn yieldThread() {
    unsafe { sys::yieldThread() }
}

pub type OSThreadProc = Option<unsafe extern "C" fn(arg1: *mut c_void) -> *mut c_void>;
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_createOSThread"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn createOSThread(
    tid: *mut OSThreadId,
    name: *const c_char,
    startProc: OSThreadProc,
    param: *mut c_void,
) -> c_int {
    unsafe { sys::createOSThread(tid, name, startProc, param) }
}

#[instrument]
pub(crate) unsafe fn createAttachedOSThread(
    tid: *mut OSThreadId,
    name: *const c_char,
    startProc: OSThreadProc,
    param: *mut c_void,
) -> c_int {
    unsafe { sys::createAttachedOSThread(tid, name, startProc, param) }
}

#[instrument]
pub(crate) unsafe fn osThreadIsAlive(id: OSThreadId) -> bool {
    unsafe { sys::osThreadIsAlive(id) }
}

#[instrument]
pub(crate) unsafe fn interruptOSThread(id: OSThreadId) {
    unsafe { sys::interruptOSThread(id) }
}

#[instrument]
pub(crate) unsafe fn joinOSThread(id: OSThreadId) {
    unsafe { sys::joinOSThread(id) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_initCondition"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn initCondition(pCond: *mut Condition) {
    unsafe { sys::initCondition(pCond as *mut sys::Condition) }
}

#[instrument]
pub(crate) unsafe fn closeCondition(pCond: *mut Condition) {
    unsafe { sys::closeCondition(pCond as *mut sys::Condition) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_broadcastCondition"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn broadcastCondition(pCond: *mut Condition) {
    unsafe { sys::broadcastCondition(pCond as *mut sys::Condition) }
}

#[instrument]
pub(crate) unsafe fn signalCondition(pCond: *mut Condition) {
    unsafe { sys::signalCondition(pCond as *mut sys::Condition) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_waitCondition"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn waitCondition(pCond: *mut Condition, pMut: *mut Mutex) {
    unsafe { sys::waitCondition(pCond as *mut sys::Condition, pMut) }
}

#[instrument]
pub(crate) unsafe fn timedWaitCondition(
    pCond: *mut Condition,
    pMut: *mut Mutex,
    timeout: Time,
) -> bool {
    unsafe { sys::timedWaitCondition(pCond as *mut sys::Condition, pMut, timeout) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_initMutex"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn initMutex(pMut: *mut Mutex) {
    unsafe { sys::initMutex(pMut) }
}

#[instrument]
pub(crate) unsafe fn closeMutex(pMut: *mut Mutex) {
    unsafe { sys::closeMutex(pMut) }
}

#[instrument]
pub(crate) unsafe fn newThreadLocalKey(key: *mut ThreadLocalKey) {
    unsafe { sys::newThreadLocalKey(key) }
}

#[instrument]
pub(crate) unsafe fn getThreadLocalVar(key: *mut ThreadLocalKey) -> *mut c_void {
    unsafe { sys::getThreadLocalVar(key) }
}

#[instrument]
pub(crate) unsafe fn setThreadLocalVar(key: *mut ThreadLocalKey, value: *mut c_void) {
    unsafe { sys::setThreadLocalVar(key, value) }
}

#[instrument]
pub(crate) unsafe fn freeThreadLocalKey(key: *mut ThreadLocalKey) {
    unsafe { sys::freeThreadLocalKey(key) }
}

#[instrument]
pub(crate) unsafe fn setThreadAffinity(n: u32, m: u32) {
    unsafe { sys::setThreadAffinity(n, m) }
}

#[instrument]
pub(crate) unsafe fn setThreadNode(node: u32) {
    unsafe { sys::setThreadNode(node) }
}

#[instrument]
pub(crate) unsafe fn releaseThreadNode() {
    unsafe { sys::releaseThreadNode() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_forkOS_createThread"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn forkOS_createThread(entry: HsStablePtr) -> c_int {
    unsafe { sys::forkOS_createThread(entry) }
}

#[instrument]
pub(crate) unsafe fn freeThreadingResources() {
    unsafe { sys::freeThreadingResources() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_getNumberOfProcessors"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn getNumberOfProcessors() -> u32 {
    unsafe { sys::getNumberOfProcessors() }
}

pub(crate) type KernelThreadId = StgWord64;

#[instrument]
pub(crate) unsafe fn kernelThreadId() -> KernelThreadId {
    unsafe { sys::kernelThreadId() }
}
