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
    unsafe { transmute(sys::osThreadId()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn shutdownThread() -> ! {
    unsafe { transmute(sys::shutdownThread()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn yieldThread() {
    unsafe { transmute(sys::yieldThread()) }
}

pub type OSThreadProc = ::core::option::Option<
    unsafe extern "C" fn(arg1: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
>;
#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn createOSThread(
    tid: *mut OSThreadId,
    name: *const ::core::ffi::c_char,
    startProc: OSThreadProc,
    param: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        transmute(sys::createOSThread(
            &mut tid.into(),
            &name.into(),
            startProc.into(),
            &mut param.into(),
        ))
    }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn createAttachedOSThread(
    tid: *mut OSThreadId,
    name: *const ::core::ffi::c_char,
    startProc: OSThreadProc,
    param: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        transmute(sys::createAttachedOSThread(
            &mut tid.into(),
            &name.into(),
            startProc.into(),
            &mut param.into(),
        ))
    }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn osThreadIsAlive(id: OSThreadId) -> bool {
    unsafe { transmute(sys::osThreadIsAlive(id.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn interruptOSThread(id: OSThreadId) {
    unsafe { transmute(sys::interruptOSThread(id.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn joinOSThread(id: OSThreadId) {
    unsafe { transmute(sys::joinOSThread(id.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn initCondition(pCond: *mut Condition) {
    unsafe { transmute(sys::initCondition(&mut pCond.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn closeCondition(pCond: *mut Condition) {
    unsafe { transmute(sys::closeCondition(&mut pCond.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn broadcastCondition(pCond: *mut Condition) {
    unsafe { transmute(sys::broadcastCondition(&mut pCond.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn signalCondition(pCond: *mut Condition) {
    unsafe { transmute(sys::signalCondition(&mut pCond.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn waitCondition(pCond: *mut Condition, pMut: *mut Mutex) {
    unsafe { transmute(sys::waitCondition(&mut pCond.into(), &mut pMut.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn timedWaitCondition(
    pCond: *mut Condition,
    pMut: *mut Mutex,
    timeout: Time,
) -> bool {
    unsafe {
        transmute(sys::timedWaitCondition(
            &mut pCond.into(),
            &mut pMut.into(),
            timeout.into(),
        ))
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn initMutex(pMut: *mut Mutex) {
    unsafe { transmute(sys::initMutex(&mut pMut.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn closeMutex(pMut: *mut Mutex) {
    unsafe { transmute(sys::closeMutex(&mut pMut.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn newThreadLocalKey(key: *mut ThreadLocalKey) {
    unsafe { transmute(sys::newThreadLocalKey(&mut key.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn getThreadLocalVar(key: *mut ThreadLocalKey) -> *mut ::core::ffi::c_void {
    unsafe { transmute(sys::getThreadLocalVar(&mut key.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn setThreadLocalVar(key: *mut ThreadLocalKey, value: *mut ::core::ffi::c_void) {
    unsafe { transmute(sys::setThreadLocalVar(&mut key.into(), &mut value.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn freeThreadLocalKey(key: *mut ThreadLocalKey) {
    unsafe { transmute(sys::freeThreadLocalKey(&mut key.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn setThreadAffinity(n: u32, m: u32) {
    unsafe { transmute(sys::setThreadAffinity(n.into(), m.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn setThreadNode(node: u32) {
    unsafe { transmute(sys::setThreadNode(node.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn releaseThreadNode() {
    unsafe { transmute(sys::releaseThreadNode()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn forkOS_createThread(entry: HsStablePtr) -> ::core::ffi::c_int {
    unsafe { transmute(sys::forkOS_createThread(entry.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn freeThreadingResources() {
    unsafe { transmute(sys::freeThreadingResources()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getNumberOfProcessors() -> u32 {
    unsafe { transmute(sys::getNumberOfProcessors()) }
}

pub(crate) type KernelThreadId = StgWord64;

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn kernelThreadId() -> KernelThreadId {
    unsafe { transmute(sys::kernelThreadId()) }
}
