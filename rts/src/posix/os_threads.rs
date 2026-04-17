use crate::ffi::rts::_assertFail;
use crate::ffi::rts::messages::{barf, sysErrorBelch};
use crate::ffi::rts::os_threads::{Condition, KernelThreadId, Mutex, OSThreadId};
use crate::ffi::rts::time::gettimeofday;
use crate::ffi::rts::time::{TIME_RESOLUTION, Time};
use crate::ffi::stg::types::StgWord64;
use crate::ffi::stg::types::StgWord64;
use crate::hs_ffi::HsStablePtr;
use crate::prelude::*;
use crate::rts_api::{rts_done, rts_evalStableIO, rts_lock, rts_unlock};
use crate::rts_utils::{stgFree, stgMallocBytes};

#[cfg(test)]
mod tests;

#[ffi(testsuite)]
pub type Mutex = pthread_mutex_t;

#[ffi(compiler, testsuite)]
#[repr(C)]
pub struct Condition {
    pub cond: pthread_cond_t,
}

#[ffi(testsuite)]
pub type OSThreadId = pthread_t;

#[ffi(testsuite)]
pub type OSThreadProc = unsafe extern "C" fn(*mut c_void) -> *mut c_void;

pub(crate) type KernelThreadId = StgWord64;

#[inline]
pub(crate) unsafe fn OS_TRY_ACQUIRE_LOCK(mut mutex: *mut pthread_mutex_t) -> i32 {
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

    if (pthread_condattr_init(&raw mut attr) == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/posix/OSThreads.c".as_ptr(), 111);
    }

    if (pthread_cond_init(&raw mut (*pCond).cond, &raw mut attr) == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/posix/OSThreads.c".as_ptr(), 118);
    }

    if (pthread_condattr_destroy(&raw mut attr) == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/posix/OSThreads.c".as_ptr(), 119);
    };
}

unsafe fn closeCondition(mut pCond: *mut Condition) {
    if (pthread_cond_destroy(&raw mut (*pCond).cond) == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/posix/OSThreads.c".as_ptr(), 125);
    };
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn broadcastCondition(mut pCond: *mut Condition) {
    if (pthread_cond_broadcast(&raw mut (*pCond).cond) == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/posix/OSThreads.c".as_ptr(), 131);
    };
}

unsafe fn signalCondition(mut pCond: *mut Condition) {
    if (pthread_cond_signal(&raw mut (*pCond).cond) == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/posix/OSThreads.c".as_ptr(), 137);
    };
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn waitCondition(mut pCond: *mut Condition, mut pMut: *mut Mutex) {
    if (pthread_cond_wait(&raw mut (*pCond).cond, pMut as *mut pthread_mutex_t) == 0) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/posix/OSThreads.c".as_ptr(), 143);
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

    if (gettimeofday(&raw mut tv, null_mut::<c_void>()) == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/posix/OSThreads.c".as_ptr(), 154);
    }

    ts.tv_sec = tv.tv_sec;
    ts.tv_nsec = (1000 * tv.tv_usec) as i64;

    let mut sec: u64 = (timeout / TIME_RESOLUTION as Time) as u64;
    ts.tv_sec = (ts.tv_sec as u64).wrapping_add(sec) as i64 as i64;
    ts.tv_nsec = (ts.tv_nsec as Time + (timeout - sec as Time * 1000000000)) as i64;
    ts.tv_sec += ts.tv_nsec / 1000000000;
    ts.tv_nsec %= 1000000000;

    let mut ret = pthread_cond_timedwait(
        &raw mut (*pCond).cond,
        pMut as *mut pthread_mutex_t,
        &raw mut ts,
    );

    match ret {
        ETIMEDOUT => return false,
        0 => return true,
        _ => {
            barf(c"pthread_cond_timedwait failed".as_ptr());
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
) -> i32 {
    let mut desc = stgMallocBytes(
        size_of::<ThreadDesc>() as usize,
        c"createAttachedOSThread".as_ptr(),
    ) as *mut ThreadDesc;

    (*desc).startProc = startProc;
    (*desc).param = param;

    (*desc).name = stgMallocBytes(
        strlen(name).wrapping_add(1 as usize),
        c"createAttachedOSThread".as_ptr(),
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
    return true;
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn initMutex(mut pMut: *mut Mutex) {
    let mut attr = _opaque_pthread_mutexattr_t {
        __sig: 0,
        __opaque: [0; 8],
    };

    pthread_mutexattr_init(&raw mut attr);
    pthread_mutexattr_settype(&raw mut attr, PTHREAD_MUTEX_ERRORCHECK);
    pthread_mutex_init(pMut as *mut pthread_mutex_t, &raw mut attr);
}

unsafe fn closeMutex(mut pMut: *mut Mutex) {
    pthread_mutex_destroy(pMut as *mut pthread_mutex_t);
}

unsafe fn forkOS_createThreadWrapper(mut entry: *mut c_void) -> *mut c_void {
    let mut cap = null_mut::<Capability>();
    cap = rts_lock();
    rts_evalStableIO(&raw mut cap, entry, null_mut::<HsStablePtr>());
    rts_unlock(cap);
    rts_done();

    return NULL;
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn forkOS_createThread(mut entry: HsStablePtr) -> c_int {
    let mut tid = null_mut::<_opaque_pthread_t>();

    let mut result = pthread_create(
        &raw mut tid,
        null::<pthread_attr_t>(),
        Some(forkOS_createThreadWrapper as unsafe extern "C" fn(*mut c_void) -> *mut c_void),
        entry,
    );

    if result == 0 {
        pthread_detach(tid as pthread_t);
    }

    return result;
}

unsafe fn freeThreadingResources() {}

static mut nproc_cache: u32 = 0;

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getNumberOfProcessors() -> c_uint {
    let mut nproc: u32 = (&raw mut nproc_cache).load(Ordering::Relaxed);

    if nproc == 0 {
        let mut size: usize = size_of::<u32>() as usize;

        if sysctlbyname(
            c"machdep.cpu.thread_count".as_ptr(),
            &raw mut nproc as *mut c_void,
            &raw mut size,
            NULL,
            0,
        ) != 0
        {
            if sysctlbyname(
                c"hw.logicalcpu".as_ptr(),
                &raw mut nproc as *mut c_void,
                &raw mut size,
                NULL,
                0,
            ) != 0
            {
                if sysctlbyname(
                    c"hw.ncpu".as_ptr(),
                    &raw mut nproc as *mut c_void,
                    &raw mut size,
                    NULL,
                    0,
                ) != 0
                {
                    nproc = 1;
                }
            }
        }

        (&raw mut nproc_cache).store(nproc, Ordering::Relaxed);
    }

    return nproc;
}

unsafe fn setThreadAffinity(mut n: u32, mut m: u32) {
    let mut policy = thread_affinity_policy { affinity_tag: 0 };

    policy.affinity_tag = n as integer_t;

    thread_policy_set(
        mach_thread_self() as thread_act_t,
        THREAD_AFFINITY_POLICY as thread_policy_flavor_t,
        &raw mut policy as thread_policy_t,
        THREAD_AFFINITY_POLICY_COUNT,
    );
}

unsafe fn setThreadNode(mut node: u32) {}

unsafe fn releaseThreadNode() {}

unsafe fn interruptOSThread(mut id: OSThreadId) {
    pthread_kill(id as pthread_t, SIGPIPE);
}

unsafe fn joinOSThread(mut id: OSThreadId) {
    let mut ret = pthread_join(id as pthread_t, null_mut::<*mut c_void>());

    if ret != 0 {
        sysErrorBelch(c"joinOSThread: error %d".as_ptr(), ret);
    }
}

unsafe fn kernelThreadId() -> KernelThreadId {
    let mut ktid: u64 = 0;
    pthread_threadid_np(null_mut::<_opaque_pthread_t>(), &raw mut ktid);

    return ktid as KernelThreadId;
}
