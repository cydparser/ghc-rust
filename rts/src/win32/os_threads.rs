use crate::ffi::hs_ffi::HsStablePtr;
use crate::ffi::rts::messages::{barf, sysErrorBelch};
use crate::ffi::rts::os_threads::{Condition, KernelThreadId, Mutex, OSThreadId};
use crate::ffi::rts::time::Time;
use crate::ffi::rts::{_assertFail, stg_exit};
use crate::ffi::rts_api::{rts_done, rts_evalStableIO, rts_lock, rts_unlock};
use crate::ffi::stg::types::StgWord64;
use crate::ffi::stg::types::{StgWord, StgWord64};
use crate::prelude::*;
use crate::rts_utils::{stgFree, stgMallocBytes};
use crate::sm::os_mem::osNumaAvailable;

#[cfg(test)]
mod tests;

#[ffi(compiler, testsuite)]
#[c2rust::src_loc(105, 1)]
pub type Condition = CONDITION_VARIABLE;

#[ffi(testsuite)]
pub type OSThreadId = DWORD;

#[ffi(testsuite)]
pub type Mutex = SRWLOCK;

#[ffi(testsuite)]
pub type OSThreadProc = unsafe extern "C" fn(*mut c_void) -> *mut c_void;

pub(crate) type KernelThreadId = StgWord64;

static mut cpuGroupCache: *mut u8 = null_mut::<u8>();

static mut cpuGroupCumulativeCache: *mut u32 = null_mut::<u32>();

static mut cpuGroupDistCache: *mut u8 = null_mut::<u8>();

unsafe fn yieldThread() {
    SwitchToThread();
}

#[ffi(utils)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn shutdownThread() -> ! {
    ExitThread(0);
    barf(c"ExitThread() returned".as_ptr());
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
    let mut h = null_mut::<c_void>();

    h = CreateThread(
        null_mut::<_SECURITY_ATTRIBUTES>(),
        0,
        transmute::<*mut c_void, LPTHREAD_START_ROUTINE>(transmute::<
            Option<OSThreadProc>,
            *mut c_void,
        >(startProc)),
        param as LPVOID,
        0,
        pId as LPDWORD,
    );

    if h.is_null() {
        return 1;
    } else {
        CloseHandle(h);

        return 0;
    };
}

unsafe fn osThreadId() -> OSThreadId {
    return GetCurrentThreadId() as OSThreadId;
}

unsafe fn osThreadIsAlive(mut id: OSThreadId) -> bool {
    let mut exit_code: DWORD = 0;
    let mut hdl = null_mut::<c_void>();
    hdl = OpenThread(THREAD_QUERY_INFORMATION as DWORD, FALSE, id as DWORD);

    if hdl.is_null() {
        sysErrorBelch(c"osThreadIsAlive: OpenThread".as_ptr());
        stg_exit(EXIT_FAILURE);
    }

    if GetExitCodeThread(hdl, &raw mut exit_code) == 0 {
        sysErrorBelch(c"osThreadIsAlive: GetExitCodeThread".as_ptr());
        stg_exit(EXIT_FAILURE);
    }

    CloseHandle(hdl);

    return exit_code == STILL_ACTIVE;
}

unsafe fn forkOS_createThreadWrapper(mut entry: *mut c_void) -> u32 {
    let mut cap = null_mut::<Capability>();
    cap = rts_lock();
    rts_evalStableIO(&raw mut cap, entry, null_mut::<HsStablePtr>());
    rts_unlock(cap);
    rts_done();

    return 0;
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn forkOS_createThread(mut entry: HsStablePtr) -> c_int {
    let mut pId: u64 = 0;

    return (_beginthreadex(
        NULL,
        0,
        Some(forkOS_createThreadWrapper as unsafe extern "C" fn(*mut c_void) -> c_uint),
        entry,
        0,
        &raw mut pId as *mut u32,
    ) == 0) as i32;
}

unsafe fn freeThreadingResources() {
    if !cpuGroupCache.is_null() {
        stgFree(cpuGroupCache as *mut c_void);
        cpuGroupCache = null_mut::<u8>();
    }

    if !cpuGroupCumulativeCache.is_null() {
        stgFree(cpuGroupCumulativeCache as *mut c_void);
        cpuGroupCumulativeCache = null_mut::<u32>();
    }

    if !cpuGroupDistCache.is_null() {
        stgFree(cpuGroupDistCache as *mut c_void);
        cpuGroupDistCache = null_mut::<u8>();
    }
}

unsafe fn getNumberOfProcessorsGroups() -> u8 {
    static mut n_groups: u8 = 0;

    if n_groups == 0 {
        n_groups = 1;
    }

    return n_groups;
}

unsafe fn getProcessorsCumulativeSum() -> *mut u32 {
    if !cpuGroupCumulativeCache.is_null() {
        return cpuGroupCumulativeCache;
    }

    if cpuGroupCumulativeCache.is_null() {
        let mut n_groups = getNumberOfProcessorsGroups();

        cpuGroupCumulativeCache = stgMallocBytes(
            (n_groups as usize).wrapping_mul(size_of::<u32>() as usize),
            c"getProcessorsCumulativeSum".as_ptr(),
        ) as *mut u32;

        memset(
            cpuGroupCumulativeCache as *mut c_void,
            0,
            (n_groups as usize).wrapping_mul(size_of::<u32>() as usize),
        );
    }

    return cpuGroupCumulativeCache;
}

unsafe fn createProcessorGroupMap() -> *mut u8 {
    if !cpuGroupCache.is_null() {
        return cpuGroupCache;
    }

    let mut numProcs = getNumberOfProcessors();

    cpuGroupCache = stgMallocBytes(
        (numProcs as usize).wrapping_mul(size_of::<u8>() as usize),
        c"createProcessorGroupMap".as_ptr(),
    ) as *mut u8;

    memset(
        cpuGroupCache as *mut c_void,
        0,
        (numProcs as usize).wrapping_mul(size_of::<u8>() as usize),
    );

    return cpuGroupCache;
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getNumberOfProcessors() -> c_uint {
    static mut nproc: u32 = 0;

    if nproc == 0 {
        let mut si = _SYSTEM_INFO {
            c2rust_unnamed: C2RustUnnamed_9 { dwOemId: 0 },
            dwPageSize: 0,
            lpMinimumApplicationAddress: null_mut::<c_void>(),
            lpMaximumApplicationAddress: null_mut::<c_void>(),
            dwActiveProcessorMask: 0,
            dwNumberOfProcessors: 0,
            dwProcessorType: 0,
            dwAllocationGranularity: 0,
            wProcessorLevel: 0,
            wProcessorRevision: 0,
        };

        GetSystemInfo(&raw mut si);
        nproc = si.dwNumberOfProcessors as u32;
    }

    return nproc;
}

unsafe fn setThreadAffinity(mut n: u32, mut m: u32) {
    if (n <= m) as i32 as i64 != 0 {
    } else {
        _assertFail(c"/Users/cyd/src/ghc/rts/win32/OSThreads.c".as_ptr(), 321);
    }

    let mut hThread = null_mut::<c_void>();
    let mut mask = null_mut::<DWORD_PTR>();
    let mut r: DWORD_PTR = 0;
    let mut n_proc: u32 = 0;
    let mut i: u32 = 0;
    let mut ix: u32 = 0;
    let mut proc_map = createProcessorGroupMap();
    let mut n_groups: u32 = getNumberOfProcessorsGroups() as u32;
    let mut proc_cum = getProcessorsCumulativeSum();
    n_proc = getNumberOfProcessors();
    hThread = GetCurrentThread();

    if !proc_map.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"/Users/cyd/src/ghc/rts/win32/OSThreads.c".as_ptr(), 333);
    }

    if !proc_cum.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"/Users/cyd/src/ghc/rts/win32/OSThreads.c".as_ptr(), 334);
    }

    if !hThread.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"/Users/cyd/src/ghc/rts/win32/OSThreads.c".as_ptr(), 335);
    }

    if (n_groups > 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"/Users/cyd/src/ghc/rts/win32/OSThreads.c".as_ptr(), 336);
    }

    if (n_proc > 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"/Users/cyd/src/ghc/rts/win32/OSThreads.c".as_ptr(), 337);
    }

    mask = stgMallocBytes(
        (n_groups as usize).wrapping_mul(size_of::<DWORD_PTR>() as usize),
        c"setThreadAffinity".as_ptr(),
    ) as *mut DWORD_PTR;

    memset(
        mask as *mut c_void,
        0,
        (n_groups as usize).wrapping_mul(size_of::<DWORD_PTR>() as usize),
    );

    let mut group: i32 = 0;
    i = n;

    while i < n_proc {
        group = *proc_map.offset(i as isize) as i32;
        ix = i.wrapping_sub(*proc_cum.offset(group as isize));
        *mask.offset(group as isize) |= (1 << ix) as DWORD_PTR;
        i = i.wrapping_add(m);
    }

    i = 0;

    while i < n_groups {
        if *mask.offset(i as isize) > 0 {
            r = SetThreadAffinityMask(hThread, *mask.offset(i as isize));

            if r == 0 {
                stgFree(mask as *mut c_void);
                sysErrorBelch(c"SetThreadAffinity".as_ptr());
                stg_exit(EXIT_FAILURE);
            }
        }

        i = i.wrapping_add(1);
    }

    stgFree(mask as *mut c_void);
}

unsafe fn interruptOSThread(mut id: OSThreadId) {
    let mut hdl = null_mut::<c_void>();
    hdl = OpenThread(THREAD_TERMINATE as DWORD, FALSE, id as DWORD);

    if hdl.is_null() {
        sysErrorBelch(c"interruptOSThread: OpenThread".as_ptr());
        stg_exit(EXIT_FAILURE);
    }

    CancelSynchronousIo(hdl);
    CloseHandle(hdl);
}

unsafe fn joinOSThread(mut id: OSThreadId) {
    let mut hdl = null_mut::<c_void>();
    hdl = OpenThread(SYNCHRONIZE as DWORD, FALSE, id as DWORD);

    if hdl.is_null() {
        sysErrorBelch(c"interruptOSThread: OpenThread".as_ptr());
        stg_exit(EXIT_FAILURE);
    }

    let mut ret = WaitForSingleObject(hdl, INFINITE) as i32;

    if ret as DWORD != WAIT_OBJECT_0 {
        sysErrorBelch(c"joinOSThread: error %d".as_ptr(), ret);
    }
}

unsafe fn setThreadNode(mut node: u32) {
    if osNumaAvailable() {
        let mut mask: u64 = 0;

        if GetNumaNodeProcessorMask(node as UCHAR, &raw mut mask) == 0
            && SetThreadAffinityMask(GetCurrentThread(), mask as DWORD_PTR) == 0
        {
            sysErrorBelch(
                c"setThreadNode: Error setting affinity of thread to NUMA node `%u': %lu.".as_ptr(),
                node,
                GetLastError(),
            );

            stg_exit(EXIT_FAILURE);
        }
    }
}

unsafe fn releaseThreadNode() {
    if osNumaAvailable() {
        let mut processMask = null_mut::<ULONG_PTR>();
        let mut systemMask = null_mut::<ULONG_PTR>();

        if GetProcessAffinityMask(GetCurrentProcess(), processMask, systemMask) == 0 {
            sysErrorBelch(
                c"releaseThreadNode: Error resetting affinity of thread: %lu".as_ptr(),
                GetLastError(),
            );

            stg_exit(EXIT_FAILURE);
        }

        if SetThreadAffinityMask(GetCurrentThread(), *processMask) == 0 {
            sysErrorBelch(
                c"releaseThreadNode: Error reseting NUMA affinity mask of thread: %lu.".as_ptr(),
                GetLastError(),
            );

            stg_exit(EXIT_FAILURE);
        }
    }
}

unsafe fn kernelThreadId() -> KernelThreadId {
    let mut tid = GetCurrentThreadId();

    return tid as KernelThreadId;
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn initCondition(mut pCond: *mut Condition) {
    InitializeConditionVariable(pCond as PCONDITION_VARIABLE);
}

unsafe fn closeCondition(mut pCond: *mut Condition) {}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn broadcastCondition(mut pCond: *mut Condition) {
    WakeAllConditionVariable(pCond as PCONDITION_VARIABLE);
}

unsafe fn signalCondition(mut pCond: *mut Condition) {
    WakeConditionVariable(pCond as PCONDITION_VARIABLE);
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn waitCondition(mut pCond: *mut Condition, mut pMut: *mut Mutex) {
    if (SleepConditionVariableSRW(
        pCond as PCONDITION_VARIABLE,
        pMut as PSRWLOCK,
        0xffffffff,
        0,
    ) != 0) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"/Users/cyd/src/ghc/rts/win32/OSThreads.c".as_ptr(), 511);
    };
}

unsafe fn timedWaitCondition(
    mut pCond: *mut Condition,
    mut pMut: *mut Mutex,
    mut timeout: Time,
) -> bool {
    let mut ms = ({
        let mut _a = 1;
        let mut _b = (timeout / 1000000) as i32;

        if _a <= _b { _a as i32 } else { _b as i32 }
    }) as DWORD;

    let mut res =
        SleepConditionVariableSRW(pCond as PCONDITION_VARIABLE, pMut as PSRWLOCK, ms, 0) as BOOL;

    if res != 0 {
        return true;
    } else if GetLastError() == 1460 {
        return false;
    } else {
        barf(
            c"timedWaitCondition: error %llu".as_ptr(),
            GetLastError() as StgWord,
        );
    };
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn initMutex(mut pMut: *mut Mutex) {
    InitializeSRWLock(pMut as PSRWLOCK);
}

unsafe fn closeMutex(mut pMut: *mut Mutex) {}
