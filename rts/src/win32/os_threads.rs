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

static mut cpuGroupCache: *mut uint8_t = null::<uint8_t>() as *mut uint8_t;

static mut cpuGroupCumulativeCache: *mut uint32_t = null::<uint32_t>() as *mut uint32_t;

static mut cpuGroupDistCache: *mut uint8_t = null::<uint8_t>() as *mut uint8_t;

unsafe fn yieldThread() {
    SwitchToThread();
}

#[ffi(utils)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn shutdownThread() -> ! {
    ExitThread(0 as DWORD);
    barf(b"ExitThread() returned\0" as *const u8 as *const c_char);
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
        0 as SIZE_T,
        transmute::<*mut c_void, LPTHREAD_START_ROUTINE>(transmute::<
            Option<OSThreadProc>,
            *mut c_void,
        >(startProc)),
        param as LPVOID,
        0 as DWORD,
        pId as LPDWORD,
    );

    if h.is_null() {
        return 1 as c_int;
    } else {
        CloseHandle(h);

        return 0 as c_int;
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
        sysErrorBelch(b"osThreadIsAlive: OpenThread\0" as *const u8 as *const c_char);
        stg_exit(EXIT_FAILURE);
    }

    if GetExitCodeThread(hdl, &raw mut exit_code) == 0 {
        sysErrorBelch(b"osThreadIsAlive: GetExitCodeThread\0" as *const u8 as *const c_char);
        stg_exit(EXIT_FAILURE);
    }

    CloseHandle(hdl);

    return exit_code == STILL_ACTIVE;
}

unsafe fn forkOS_createThreadWrapper(mut entry: *mut c_void) -> c_uint {
    let mut cap = null_mut::<Capability>();
    cap = rts_lock();
    rts_evalStableIO(&raw mut cap, entry, null_mut::<HsStablePtr>());
    rts_unlock(cap);
    rts_done();

    return 0 as c_uint;
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn forkOS_createThread(mut entry: HsStablePtr) -> c_int {
    let mut pId: c_ulong = 0;

    return (_beginthreadex(
        NULL,
        0 as c_uint,
        Some(forkOS_createThreadWrapper as unsafe extern "C" fn(*mut c_void) -> c_uint),
        entry,
        0 as c_uint,
        &raw mut pId as *mut c_uint,
    ) == 0 as uintptr_t) as c_int;
}

unsafe fn freeThreadingResources() {
    if !cpuGroupCache.is_null() {
        stgFree(cpuGroupCache as *mut c_void);
        cpuGroupCache = null_mut::<uint8_t>();
    }

    if !cpuGroupCumulativeCache.is_null() {
        stgFree(cpuGroupCumulativeCache as *mut c_void);
        cpuGroupCumulativeCache = null_mut::<uint32_t>();
    }

    if !cpuGroupDistCache.is_null() {
        stgFree(cpuGroupDistCache as *mut c_void);
        cpuGroupDistCache = null_mut::<uint8_t>();
    }
}

unsafe fn getNumberOfProcessorsGroups() -> uint8_t {
    static mut n_groups: uint8_t = 0 as uint8_t;

    if n_groups == 0 {
        n_groups = 1 as uint8_t;
    }

    return n_groups;
}

unsafe fn getProcessorsCumulativeSum() -> *mut uint32_t {
    if !cpuGroupCumulativeCache.is_null() {
        return cpuGroupCumulativeCache;
    }

    if cpuGroupCumulativeCache.is_null() {
        let mut n_groups = getNumberOfProcessorsGroups();

        cpuGroupCumulativeCache = stgMallocBytes(
            (n_groups as size_t).wrapping_mul(size_of::<uint32_t>() as size_t),
            b"getProcessorsCumulativeSum\0" as *const u8 as *const c_char as *mut c_char,
        ) as *mut uint32_t;

        memset(
            cpuGroupCumulativeCache as *mut c_void,
            0 as c_int,
            (n_groups as size_t).wrapping_mul(size_of::<uint32_t>() as size_t),
        );
    }

    return cpuGroupCumulativeCache;
}

unsafe fn createProcessorGroupMap() -> *mut uint8_t {
    if !cpuGroupCache.is_null() {
        return cpuGroupCache;
    }

    let mut numProcs = getNumberOfProcessors();

    cpuGroupCache = stgMallocBytes(
        (numProcs as size_t).wrapping_mul(size_of::<uint8_t>() as size_t),
        b"createProcessorGroupMap\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut uint8_t;

    memset(
        cpuGroupCache as *mut c_void,
        0 as c_int,
        (numProcs as size_t).wrapping_mul(size_of::<uint8_t>() as size_t),
    );

    return cpuGroupCache;
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getNumberOfProcessors() -> uint32_t {
    static mut nproc: uint32_t = 0 as uint32_t;

    if nproc == 0 as uint32_t {
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
        nproc = si.dwNumberOfProcessors as uint32_t;
    }

    return nproc;
}

unsafe fn setThreadAffinity(mut n: uint32_t, mut m: uint32_t) {
    if (n <= m) as c_int as c_long != 0 {
    } else {
        _assertFail(
            b"/Users/cyd/src/ghc/rts/win32/OSThreads.c\0" as *const u8 as *const c_char,
            321 as c_uint,
        );
    }

    let mut hThread = null_mut::<c_void>();
    let mut mask = null_mut::<DWORD_PTR>();
    let mut r: DWORD_PTR = 0;
    let mut n_proc: uint32_t = 0;
    let mut i: uint32_t = 0;
    let mut ix: uint32_t = 0;
    let mut proc_map = createProcessorGroupMap();
    let mut n_groups: uint32_t = getNumberOfProcessorsGroups() as uint32_t;
    let mut proc_cum = getProcessorsCumulativeSum();
    n_proc = getNumberOfProcessors();
    hThread = GetCurrentThread();

    if !proc_map.is_null() as c_int as c_long != 0 {
    } else {
        _assertFail(
            b"/Users/cyd/src/ghc/rts/win32/OSThreads.c\0" as *const u8 as *const c_char,
            333 as c_uint,
        );
    }

    if !proc_cum.is_null() as c_int as c_long != 0 {
    } else {
        _assertFail(
            b"/Users/cyd/src/ghc/rts/win32/OSThreads.c\0" as *const u8 as *const c_char,
            334 as c_uint,
        );
    }

    if !hThread.is_null() as c_int as c_long != 0 {
    } else {
        _assertFail(
            b"/Users/cyd/src/ghc/rts/win32/OSThreads.c\0" as *const u8 as *const c_char,
            335 as c_uint,
        );
    }

    if (n_groups > 0 as uint32_t) as c_int as c_long != 0 {
    } else {
        _assertFail(
            b"/Users/cyd/src/ghc/rts/win32/OSThreads.c\0" as *const u8 as *const c_char,
            336 as c_uint,
        );
    }

    if (n_proc > 0 as uint32_t) as c_int as c_long != 0 {
    } else {
        _assertFail(
            b"/Users/cyd/src/ghc/rts/win32/OSThreads.c\0" as *const u8 as *const c_char,
            337 as c_uint,
        );
    }

    mask = stgMallocBytes(
        (n_groups as size_t).wrapping_mul(size_of::<DWORD_PTR>() as size_t),
        b"setThreadAffinity\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut DWORD_PTR;

    memset(
        mask as *mut c_void,
        0 as c_int,
        (n_groups as size_t).wrapping_mul(size_of::<DWORD_PTR>() as size_t),
    );

    let mut group: c_int = 0;
    i = n;

    while i < n_proc {
        group = *proc_map.offset(i as isize) as c_int;
        ix = i.wrapping_sub(*proc_cum.offset(group as isize));
        *mask.offset(group as isize) |= ((1 as c_int) << ix) as DWORD_PTR;
        i = i.wrapping_add(m);
    }

    i = 0 as uint32_t;

    while i < n_groups {
        if *mask.offset(i as isize) > 0 as DWORD_PTR {
            r = SetThreadAffinityMask(hThread, *mask.offset(i as isize));

            if r == 0 as DWORD_PTR {
                stgFree(mask as *mut c_void);
                sysErrorBelch(b"SetThreadAffinity\0" as *const u8 as *const c_char);
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
        sysErrorBelch(b"interruptOSThread: OpenThread\0" as *const u8 as *const c_char);
        stg_exit(EXIT_FAILURE);
    }

    CancelSynchronousIo(hdl);
    CloseHandle(hdl);
}

unsafe fn joinOSThread(mut id: OSThreadId) {
    let mut hdl = null_mut::<c_void>();
    hdl = OpenThread(SYNCHRONIZE as DWORD, FALSE, id as DWORD);

    if hdl.is_null() {
        sysErrorBelch(b"interruptOSThread: OpenThread\0" as *const u8 as *const c_char);
        stg_exit(EXIT_FAILURE);
    }

    let mut ret = WaitForSingleObject(hdl, INFINITE) as c_int;

    if ret as DWORD != WAIT_OBJECT_0 {
        sysErrorBelch(
            b"joinOSThread: error %d\0" as *const u8 as *const c_char,
            ret,
        );
    }
}

unsafe fn setThreadNode(mut node: uint32_t) {
    if osNumaAvailable() {
        let mut mask: uint64_t = 0 as uint64_t;

        if GetNumaNodeProcessorMask(node as UCHAR, &raw mut mask) == 0
            && SetThreadAffinityMask(GetCurrentThread(), mask as DWORD_PTR) == 0
        {
            sysErrorBelch(
                b"setThreadNode: Error setting affinity of thread to NUMA node `%u': %lu.\0"
                    as *const u8 as *const c_char,
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
                b"releaseThreadNode: Error resetting affinity of thread: %lu\0" as *const u8
                    as *const c_char,
                GetLastError(),
            );

            stg_exit(EXIT_FAILURE);
        }

        if SetThreadAffinityMask(GetCurrentThread(), *processMask) == 0 {
            sysErrorBelch(
                b"releaseThreadNode: Error reseting NUMA affinity mask of thread: %lu.\0"
                    as *const u8 as *const c_char,
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
        0xffffffff as DWORD,
        0 as ULONG,
    ) != 0) as c_int as c_long
        != 0
    {
    } else {
        _assertFail(
            b"/Users/cyd/src/ghc/rts/win32/OSThreads.c\0" as *const u8 as *const c_char,
            511 as c_uint,
        );
    };
}

unsafe fn timedWaitCondition(
    mut pCond: *mut Condition,
    mut pMut: *mut Mutex,
    mut timeout: Time,
) -> bool {
    let mut ms = ({
        let mut _a = 1 as c_int;
        let mut _b = (timeout / 1000000 as Time) as c_int;

        if _a <= _b { _a as c_int } else { _b as c_int }
    }) as DWORD;

    let mut res = SleepConditionVariableSRW(
        pCond as PCONDITION_VARIABLE,
        pMut as PSRWLOCK,
        ms,
        0 as ULONG,
    ) as BOOL;

    if res != 0 {
        return r#true != 0;
    } else if GetLastError() == 1460 as DWORD {
        return r#false != 0;
    } else {
        barf(
            b"timedWaitCondition: error %llu\0" as *const u8 as *const c_char,
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
