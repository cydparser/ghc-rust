use crate::ffi::hs_ffi::HsStablePtr;
use crate::ffi::rts::_assertFail;
use crate::ffi::rts::messages::{barf, sysErrorBelch};
use crate::ffi::rts::os_threads::{Condition, KernelThreadId, Mutex, OSThreadId};
use crate::ffi::rts::time::gettimeofday;
use crate::ffi::rts::time::{TIME_RESOLUTION, Time};
use crate::ffi::stg::types::StgWord64;
use crate::ffi::stg::types::StgWord64;
use crate::prelude::*;
use crate::rts_utils::{stgFree, stgMallocBytes};

#[cfg(test)]
mod tests;

#[ffi(compiler, testsuite)]
#[repr(C)]
pub struct Condition {
    pub cond: pthread_cond_t,
}

#[ffi(testsuite)]
pub type Mutex = pthread_mutex_t;

#[ffi(testsuite)]
pub type OSThreadId = pthread_t;

#[ffi(testsuite)]
pub type OSThreadProc = unsafe extern "C" fn(*mut c_void) -> *mut c_void;

pub(crate) type KernelThreadId = StgWord64;

#[inline]
pub(crate) unsafe fn OS_TRY_ACQUIRE_LOCK(mut mutex: *mut pthread_mutex_t) -> c_int {
    return pthread_mutex_trylock(mutex as *mut pthread_mutex_t);
}

/// cbindgen:no-export
struct ThreadDesc {
    startProc: Option<OSThreadProc>,
    param: *mut c_void,
    name: *mut c_char,
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn initCondition(mut pCond: *mut Condition) {
    let mut attr = _opaque_pthread_condattr_t {
        __sig: 0,
        __opaque: [0; 8],
    };

    if (pthread_condattr_init(&raw mut attr) == 0 as c_int) as c_int as c_long != 0 {
    } else {
        _assertFail(
            b"rts/posix/OSThreads.c\0" as *const u8 as *const c_char,
            111 as c_uint,
        );
    }

    if (pthread_cond_init(&raw mut (*pCond).cond, &raw mut attr) == 0 as c_int) as c_int as c_long
        != 0
    {
    } else {
        _assertFail(
            b"rts/posix/OSThreads.c\0" as *const u8 as *const c_char,
            118 as c_uint,
        );
    }

    if (pthread_condattr_destroy(&raw mut attr) == 0 as c_int) as c_int as c_long != 0 {
    } else {
        _assertFail(
            b"rts/posix/OSThreads.c\0" as *const u8 as *const c_char,
            119 as c_uint,
        );
    };
}

unsafe fn closeCondition(mut pCond: *mut Condition) {
    if (pthread_cond_destroy(&raw mut (*pCond).cond) == 0 as c_int) as c_int as c_long != 0 {
    } else {
        _assertFail(
            b"rts/posix/OSThreads.c\0" as *const u8 as *const c_char,
            125 as c_uint,
        );
    };
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn broadcastCondition(mut pCond: *mut Condition) {
    if (pthread_cond_broadcast(&raw mut (*pCond).cond) == 0 as c_int) as c_int as c_long != 0 {
    } else {
        _assertFail(
            b"rts/posix/OSThreads.c\0" as *const u8 as *const c_char,
            131 as c_uint,
        );
    };
}

unsafe fn signalCondition(mut pCond: *mut Condition) {
    if (pthread_cond_signal(&raw mut (*pCond).cond) == 0 as c_int) as c_int as c_long != 0 {
    } else {
        _assertFail(
            b"rts/posix/OSThreads.c\0" as *const u8 as *const c_char,
            137 as c_uint,
        );
    };
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn waitCondition(mut pCond: *mut Condition, mut pMut: *mut Mutex) {
    if (pthread_cond_wait(&raw mut (*pCond).cond, pMut as *mut pthread_mutex_t) == 0 as c_int)
        as c_int as c_long
        != 0
    {
    } else {
        _assertFail(
            b"rts/posix/OSThreads.c\0" as *const u8 as *const c_char,
            143 as c_uint,
        );
    };
}

unsafe fn timedWaitCondition(
    mut pCond: *mut Condition,
    mut pMut: *mut Mutex,
    mut timeout: Time,
) -> bool {
    let mut ts = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };

    let mut tv = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };

    if (gettimeofday(&raw mut tv, null_mut::<c_void>()) == 0 as c_int) as c_int as c_long != 0 {
    } else {
        _assertFail(
            b"rts/posix/OSThreads.c\0" as *const u8 as *const c_char,
            154 as c_uint,
        );
    }

    ts.tv_sec = tv.tv_sec;
    ts.tv_nsec = (1000 as __darwin_suseconds_t * tv.tv_usec) as c_long;

    let mut sec: uint64_t = (timeout / TIME_RESOLUTION as Time) as uint64_t;
    ts.tv_sec = (ts.tv_sec as uint64_t).wrapping_add(sec) as __darwin_time_t as __darwin_time_t;
    ts.tv_nsec = (ts.tv_nsec as Time + (timeout - sec as Time * 1000000000 as Time)) as c_long;
    ts.tv_sec += ts.tv_nsec / 1000000000 as c_long;
    ts.tv_nsec %= 1000000000 as c_long;

    let mut ret = pthread_cond_timedwait(
        &raw mut (*pCond).cond,
        pMut as *mut pthread_mutex_t,
        &raw mut ts,
    );

    match ret {
        ETIMEDOUT => return r#false != 0,
        0 => return r#true != 0,
        _ => {
            barf(b"pthread_cond_timedwait failed\0" as *const u8 as *const c_char);
        }
    };
}

unsafe fn yieldThread() {
    sched_yield();
}

#[ffi(utils)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn shutdownThread() -> ! {
    pthread_exit(NULL);
}

unsafe fn start_thread(mut param: *mut c_void) -> *mut c_void {
    let mut desc = param as *mut ThreadDesc;
    let mut startProc: Option<OSThreadProc> = (*desc).startProc;
    let mut startParam = (*desc).param;
    pthread_setname_np((*desc).name as *const c_char);
    stgFree((*desc).name as *mut c_void);
    stgFree(desc as *mut c_void);

    return startProc.expect("non-null function pointer")(startParam);
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn createOSThread(
    mut pId: *mut OSThreadId,
    mut name: *const c_char,
    mut startProc: Option<OSThreadProc>,
    mut param: *mut c_void,
) -> c_int {
    let mut result = createAttachedOSThread(pId, name, startProc, param);

    if result == 0 {
        pthread_detach(*pId);
    }

    return result;
}

unsafe fn createAttachedOSThread(
    mut pId: *mut OSThreadId,
    mut name: *const c_char,
    mut startProc: Option<OSThreadProc>,
    mut param: *mut c_void,
) -> c_int {
    let mut desc = stgMallocBytes(
        size_of::<ThreadDesc>() as size_t,
        b"createAttachedOSThread\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut ThreadDesc;

    (*desc).startProc = startProc;
    (*desc).param = param;

    (*desc).name = stgMallocBytes(
        strlen(name).wrapping_add(1 as size_t),
        b"createAttachedOSThread\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut c_char;

    strcpy((*desc).name, name);

    let mut result = pthread_create(
        pId as *mut pthread_t,
        null::<pthread_attr_t>(),
        Some(start_thread as unsafe extern "C" fn(*mut c_void) -> *mut c_void),
        desc as *mut c_void,
    );

    if result != 0 {
        stgFree((*desc).name as *mut c_void);
        stgFree(desc as *mut c_void);
    }

    return result;
}

unsafe fn osThreadId() -> OSThreadId {
    return pthread_self() as OSThreadId;
}

unsafe fn osThreadIsAlive(mut id: OSThreadId) -> bool {
    return r#true != 0;
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn initMutex(mut pMut: *mut Mutex) {
    pthread_mutex_init(pMut as *mut pthread_mutex_t, null::<pthread_mutexattr_t>());
}

unsafe fn closeMutex(mut pMut: *mut Mutex) {
    pthread_mutex_destroy(pMut as *mut pthread_mutex_t);
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn forkOS_createThread(mut entry: HsStablePtr) -> c_int {
    return -(1 as c_int);
}

unsafe fn freeThreadingResources() {}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getNumberOfProcessors() -> uint32_t {
    return 1 as uint32_t;
}

unsafe fn setThreadAffinity(mut n: uint32_t, mut m: uint32_t) {
    let mut policy = thread_affinity_policy { affinity_tag: 0 };
    policy.affinity_tag = n as integer_t;

    thread_policy_set(
        mach_thread_self() as thread_act_t,
        THREAD_AFFINITY_POLICY as thread_policy_flavor_t,
        &raw mut policy as thread_policy_t,
        THREAD_AFFINITY_POLICY_COUNT,
    );
}

unsafe fn setThreadNode(mut node: uint32_t) {}

unsafe fn releaseThreadNode() {}

unsafe fn interruptOSThread(mut id: OSThreadId) {
    pthread_kill(id as pthread_t, SIGPIPE);
}

unsafe fn joinOSThread(mut id: OSThreadId) {
    let mut ret = pthread_join(id as pthread_t, null_mut::<*mut c_void>());

    if ret != 0 as c_int {
        sysErrorBelch(
            b"joinOSThread: error %d\0" as *const u8 as *const c_char,
            ret,
        );
    }
}

unsafe fn kernelThreadId() -> KernelThreadId {
    let mut ktid: uint64_t = 0;
    pthread_threadid_np(null_mut::<_opaque_pthread_t>(), &raw mut ktid);

    return ktid as KernelThreadId;
}
