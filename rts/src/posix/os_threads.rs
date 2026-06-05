use std::mem::MaybeUninit;

use crate::hs_ffi::HsStablePtr;
use crate::prelude::*;
use crate::rts_api::{rts_done, rts_evalStableIO, rts_lock, rts_unlock};
use crate::rts_messages::{_assertFail, barf, sysErrorBelch};
use crate::rts_utils::{stgFree, stgMallocBytes};
use crate::stg::types::StgWord64;
use crate::time::{SecondsToTime, Time, TimeToNS, TimeToSeconds};

#[cfg(test)]
mod tests;

#[ffi(testsuite)]
#[repr(transparent)]
pub struct Mutex(pub(crate) libc::pthread_mutex_t);

impl Mutex {
    pub fn new() -> Self {
        Mutex(libc::PTHREAD_MUTEX_INITIALIZER)
    }
}

#[ffi(compiler, testsuite)]
#[repr(C)]
pub struct Condition {
    cond: libc::pthread_cond_t,
}

impl Condition {
    pub fn new() -> Self {
        Condition {
            cond: libc::PTHREAD_COND_INITIALIZER,
        }
    }
}

#[ffi(testsuite)]
pub type OSThreadId = libc::pthread_t;

#[ffi(testsuite)]
pub type OSThreadProc = unsafe extern "C" fn(*mut c_void) -> *mut c_void;

pub(crate) type KernelThreadId = StgWord64;

macro_rules! LOCK_DEBUG_BELCH {
    ($msg:literal, $mutex:expr) => {
        if false {
            unsafe {
                $crate::rts_messages::debugBelch(
                    c"%s(0x%p) %s %d\n".as_ptr(),
                    $msg.as_ptr(),
                    $mutex,
                    concat!(file!(), "\0"),
                    line!(),
                )
            }
        }
    };
}

macro_rules! OS_ACQUIRE_LOCK {
    ($mutex:expr) => {
        LOCK_DEBUG_BELCH!(c"ACQUIRE_LOCK", $mutex);

        let r = libc::pthread_mutex_lock($mutex);

        if r != 0 {
            barf(
                c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
                concat!(file!(), "\0"),
                line!(),
                r,
            );
        }
    };
}

pub(crate) use OS_ACQUIRE_LOCK;

#[inline]
pub(crate) unsafe fn OS_TRY_ACQUIRE_LOCK(mutex: *mut Mutex) -> i32 {
    return libc::pthread_mutex_trylock(&raw mut (*mutex).0);
}

macro_rules! OS_RELEASE_LOCK {
    ($mutex:expr) => {
        LOCK_DEBUG_BELCH!(c"RELEASE_LOCK", $mutex);

        if libc::pthread_mutex_unlock(mutex.0) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                concat!(file!()),
                line!(),
            );
        }
    };
}

/// Note: this assertion calls pthread_mutex_lock() on a mutex that
/// is already held by the calling thread.  The mutex should therefore
/// have been created with PTHREAD_MUTEX_ERRORCHECK, otherwise this
/// assertion will hang.  We always initialise mutexes with
/// PTHREAD_MUTEX_ERRORCHECK when DEBUG is on (see rts/posix/OSThreads.h).
macro_rules! OS_ASSERT_LOCK_HELD {
    ($mutex:expr) => {
        rts_assert!(libc::pthread_mutex_lock(&raw mut $mutex.0) == libc::EDEADLK)
    };
}

pub(crate) use OS_ASSERT_LOCK_HELD;

/// cbindgen:no-export
struct ThreadDesc {
    startProc: Option<OSThreadProc>,
    param: *mut c_void,
    name: *mut c_char,
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn initCondition(pCond: *mut Condition) {
    let mut attr = MaybeUninit::uninit();

    if (libc::pthread_condattr_init(attr.as_mut_ptr()) == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/posix/OSThreads.c".as_ptr(), 111);
    }

    if (libc::pthread_cond_init(&raw mut (*pCond).cond, attr.as_mut_ptr()) == 0) as i32 as i64 != 0
    {
    } else {
        _assertFail(c"rts/posix/OSThreads.c".as_ptr(), 118);
    }

    if (libc::pthread_condattr_destroy(attr.as_mut_ptr()) == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/posix/OSThreads.c".as_ptr(), 119);
    };
}

unsafe fn closeCondition(pCond: *mut Condition) {
    if (libc::pthread_cond_destroy(&raw mut (*pCond).cond) == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/posix/OSThreads.c".as_ptr(), 125);
    };
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn broadcastCondition(pCond: *mut Condition) {
    if (libc::pthread_cond_broadcast(&raw mut (*pCond).cond) == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/posix/OSThreads.c".as_ptr(), 131);
    };
}

pub(crate) unsafe fn signalCondition(pCond: *mut Condition) {
    if (libc::pthread_cond_signal(&raw mut (*pCond).cond) == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/posix/OSThreads.c".as_ptr(), 137);
    };
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn waitCondition(pCond: *mut Condition, pMut: *mut Mutex) {
    if (libc::pthread_cond_wait(&raw mut (*pCond).cond, pMut as *mut libc::pthread_mutex_t) == 0)
        as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/posix/OSThreads.c".as_ptr(), 143);
    };
}

unsafe fn timedWaitCondition(pCond: *mut Condition, pMut: *mut Mutex, timeout: Time) -> bool {
    let mut tv = libc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };

    if (libc::gettimeofday(&raw mut tv, null_mut::<c_void>()) == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/posix/OSThreads.c".as_ptr(), 154);
    }

    let mut ts = libc::timespec {
        tv_sec: tv.tv_sec,
        tv_nsec: (1000 * tv.tv_usec) as i64,
    };

    let sec = TimeToSeconds(timeout);
    ts.tv_sec += sec;
    ts.tv_nsec += TimeToNS(timeout - SecondsToTime(sec));
    ts.tv_sec += ts.tv_nsec / 1000000000;
    ts.tv_nsec %= 1000000000;

    let ret = libc::pthread_cond_timedwait(
        &raw mut (*pCond).cond,
        pMut as *mut libc::pthread_mutex_t,
        &raw mut ts,
    );

    match ret {
        libc::ETIMEDOUT => false,
        0 => true,
        _ => {
            barf(c"pthread_cond_timedwait failed".as_ptr());
        }
    }
}

pub(crate) unsafe fn yieldThread() {
    libc::sched_yield();
}

#[ffi(utils)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn shutdownThread() -> ! {
    libc::pthread_exit(null_mut())
}

extern "C" fn start_thread(param: *mut c_void) -> *mut c_void {
    unsafe {
        let desc = param as *mut ThreadDesc;
        let startProc: Option<OSThreadProc> = (*desc).startProc;
        let startParam = (*desc).param;
        libc::pthread_setname_np((*desc).name as *const c_char);
        stgFree((*desc).name as *mut c_void);
        stgFree(desc as *mut c_void);

        startProc.expect("non-null function pointer")(startParam)
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn createOSThread(
    pId: *mut OSThreadId,
    name: *const c_char,
    startProc: Option<OSThreadProc>,
    param: *mut c_void,
) -> c_int {
    let result = createAttachedOSThread(pId, name, startProc, param);

    if result == 0 {
        libc::pthread_detach(*pId);
    }

    return result;
}

unsafe fn createAttachedOSThread(
    pId: *mut OSThreadId,
    name: *const c_char,
    startProc: Option<OSThreadProc>,
    param: *mut c_void,
) -> i32 {
    let desc = stgMallocBytes(
        size_of::<ThreadDesc>() as usize,
        c"createAttachedOSThread".as_ptr(),
    ) as *mut ThreadDesc;

    (*desc).startProc = startProc;
    (*desc).param = param;

    (*desc).name = stgMallocBytes(
        libc::strlen(name).wrapping_add(1 as usize),
        c"createAttachedOSThread".as_ptr(),
    ) as *mut c_char;

    libc::strcpy((*desc).name, name);

    let result = libc::pthread_create(
        pId as *mut libc::pthread_t,
        null::<libc::pthread_attr_t>(),
        start_thread as extern "C" fn(*mut c_void) -> *mut c_void,
        desc as *mut c_void,
    );

    if result != 0 {
        stgFree((*desc).name as *mut c_void);
        stgFree(desc as *mut c_void);
    }

    return result;
}

pub(crate) unsafe fn osThreadId() -> OSThreadId {
    libc::pthread_self() as OSThreadId
}

pub(crate) unsafe fn osThreadIsAlive(_id: OSThreadId) -> bool {
    return true;
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn initMutex(pMut: *mut Mutex) {
    let mut attr = MaybeUninit::uninit();

    libc::pthread_mutexattr_init(attr.as_mut_ptr());
    libc::pthread_mutexattr_settype(attr.as_mut_ptr(), libc::PTHREAD_MUTEX_ERRORCHECK);
    libc::pthread_mutex_init(pMut as *mut libc::pthread_mutex_t, attr.as_mut_ptr());
}

pub(crate) unsafe fn closeMutex(pMut: *mut Mutex) {
    libc::pthread_mutex_destroy(pMut as *mut libc::pthread_mutex_t);
}

extern "C" fn forkOS_createThreadWrapper(entry: *mut c_void) -> *mut c_void {
    unsafe {
        fork_os_create_thread_wrapper(entry);
    }

    null_mut()
}

#[inline]
unsafe fn fork_os_create_thread_wrapper(entry: *mut c_void) {
    let cap = rts_lock();
    rts_evalStableIO(&raw const cap, entry, null_mut::<HsStablePtr>());
    rts_unlock(cap);
    rts_done();
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn forkOS_createThread(entry: HsStablePtr) -> c_int {
    let mut tid = 0;

    let result = libc::pthread_create(
        &raw mut tid,
        null::<libc::pthread_attr_t>(),
        forkOS_createThreadWrapper as extern "C" fn(*mut c_void) -> *mut c_void,
        entry,
    );

    if result == 0 {
        libc::pthread_detach(tid as libc::pthread_t);
    }

    result
}

unsafe fn freeThreadingResources() {}

static nproc_cache: AtomicU32 = AtomicU32::new(0);

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getNumberOfProcessors() -> c_uint {
    let mut nproc: u32 = nproc_cache.load(Relaxed);

    if nproc == 0 {
        let mut size: usize = size_of::<u32>() as usize;

        if libc::sysctlbyname(
            c"machdep.cpu.thread_count".as_ptr(),
            &raw mut nproc as *mut c_void,
            &raw mut size,
            null_mut(),
            0,
        ) != 0
        {
            if libc::sysctlbyname(
                c"hw.logicalcpu".as_ptr(),
                &raw mut nproc as *mut c_void,
                &raw mut size,
                null_mut(),
                0,
            ) != 0
            {
                if libc::sysctlbyname(
                    c"hw.ncpu".as_ptr(),
                    &raw mut nproc as *mut c_void,
                    &raw mut size,
                    null_mut(),
                    0,
                ) != 0
                {
                    nproc = 1;
                }
            }
        }

        nproc_cache.store(nproc, Relaxed);
    }

    return nproc;
}

unsafe fn setThreadAffinity(n: u32, _m: u32) {
    let mut policy = libc::thread_affinity_policy {
        affinity_tag: n as i32,
    };

    libc::thread_policy_set(
        libc::mach_thread_self() as libc::thread_act_t,
        libc::THREAD_AFFINITY_POLICY as libc::thread_policy_flavor_t,
        &raw mut policy as libc::thread_policy_t,
        libc::THREAD_AFFINITY_POLICY_COUNT,
    );
}

unsafe fn setThreadNode(_node: u32) {}

unsafe fn releaseThreadNode() {}

unsafe fn interruptOSThread(id: OSThreadId) {
    libc::pthread_kill(id as libc::pthread_t, libc::SIGPIPE);
}

unsafe fn joinOSThread(id: OSThreadId) {
    let ret = libc::pthread_join(id as libc::pthread_t, null_mut::<*mut c_void>());

    if ret != 0 {
        sysErrorBelch(c"joinOSThread: error %d".as_ptr(), ret);
    }
}

/// TODO(rust): Implement kernelThreadId
unsafe fn kernelThreadId() -> KernelThreadId {
    0
}
