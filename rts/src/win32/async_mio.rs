use crate::ffi::hs_ffi::HsInt;
use crate::ffi::rts::os_threads::Mutex;
use crate::prelude::*;
use crate::win32::mio_manager::{
    AddDelayRequest, AddIORequest, AddProcRequest, ShutdownIOManager, StartIOManager,
    interruptIOManagerEvent,
};

/// cbindgen:no-export
struct CompletedReq {
    reqID: u32,
    len: HsInt,
    errCode: HsInt,
}

const MAX_REQUESTS: i32 = 200;

static mut queue_lock: Mutex = _RTL_SRWLOCK {
    Ptr: null_mut::<c_void>(),
};

static mut completed_req_event: HANDLE = unsafe { INVALID_HANDLE_VALUE };

static mut abandon_req_wait: HANDLE = unsafe { INVALID_HANDLE_VALUE };

static mut wait_handles: [HANDLE; 2] = [null_mut::<c_void>(); 2];

static mut completedTable: [CompletedReq; 200] = [CompletedReq {
    reqID: 0,
    len: 0,
    errCode: 0,
}; 200];

static mut completed_hw: i32 = 0;

static mut completed_table_sema: HANDLE = null_mut::<c_void>();

static mut issued_reqs: i32 = 0;

unsafe fn onIOComplete(
    mut reqID: u32,
    mut fd: i32,
    mut len: HsInt,
    mut buf: *mut c_void,
    mut errCode: HsInt,
) {
    let mut dwRes: DWORD = 0;
    dwRes = WaitForSingleObject(completed_table_sema, INFINITE);

    match dwRes {
        WAIT_OBJECT_0 => {}
        _ => {
            fprintf(
                __stderrp,
                c"onIOComplete: failed to grab table semaphore (res=%d, err=%ld), dropping request 0x%lx\n"
                    .as_ptr(),
                reqID,
                dwRes,
                GetLastError(),
            );

            fflush(__stderrp);
            return;
        }
    }

    AcquireSRWLockExclusive(&raw mut queue_lock);

    if completed_hw == MAX_REQUESTS {
        fprintf(
            __stderrp,
            c"onIOComplete: ERROR -- Request table overflow (%d); dropping.\n".as_ptr(),
            reqID,
        );

        fflush(__stderrp);
    } else {
        completedTable[completed_hw as usize].reqID = reqID;
        completedTable[completed_hw as usize].len = len;
        completedTable[completed_hw as usize].errCode = errCode;
        completed_hw += 1;
        issued_reqs -= 1;

        if completed_hw == 1 {
            SetEvent(completed_req_event);
        }
    }

    ReleaseSRWLockExclusive(&raw mut queue_lock);
}

unsafe fn addIORequest(
    mut fd: i32,
    mut forWriting: bool,
    mut isSock: bool,
    mut len: HsInt,
    mut buf: *mut c_char,
) -> u32 {
    AcquireSRWLockExclusive(&raw mut queue_lock);
    issued_reqs += 1;
    ReleaseSRWLockExclusive(&raw mut queue_lock);

    return AddIORequest(
        fd,
        forWriting,
        isSock,
        len,
        buf,
        Some(onIOComplete as unsafe extern "C" fn(c_uint, c_int, HsInt, *mut c_void, HsInt) -> ()),
    ) as u32;
}

unsafe fn addDelayRequest(mut usecs: HsInt) -> u32 {
    AcquireSRWLockExclusive(&raw mut queue_lock);
    issued_reqs += 1;
    ReleaseSRWLockExclusive(&raw mut queue_lock);

    return AddDelayRequest(
        usecs,
        Some(onIOComplete as unsafe extern "C" fn(c_uint, c_int, HsInt, *mut c_void, HsInt) -> ()),
    ) as u32;
}

unsafe fn addDoProcRequest(mut proc: *mut c_void, mut param: *mut c_void) -> u32 {
    AcquireSRWLockExclusive(&raw mut queue_lock);
    issued_reqs += 1;
    ReleaseSRWLockExclusive(&raw mut queue_lock);

    return AddProcRequest(
        proc,
        param,
        Some(onIOComplete as unsafe extern "C" fn(c_uint, c_int, HsInt, *mut c_void, HsInt) -> ()),
    ) as u32;
}

unsafe fn startupAsyncIO() -> i32 {
    if !StartIOManager() {
        return 0;
    }

    InitializeSRWLock(&raw mut queue_lock);

    completed_req_event = CreateEventA(
        null_mut::<_SECURITY_ATTRIBUTES>(),
        TRUE,
        FALSE,
        null::<CHAR>(),
    );

    abandon_req_wait = CreateEventA(
        null_mut::<_SECURITY_ATTRIBUTES>(),
        FALSE,
        FALSE,
        null::<CHAR>(),
    );

    wait_handles[0] = completed_req_event;
    wait_handles[1] = abandon_req_wait;
    completed_hw = 0;

    completed_table_sema = CreateSemaphoreA(
        null_mut::<_SECURITY_ATTRIBUTES>(),
        MAX_REQUESTS,
        MAX_REQUESTS,
        null::<CHAR>(),
    );

    if completed_table_sema.is_null() {
        let mut rc = GetLastError();

        fprintf(
            __stderrp,
            c"startupAsyncIO: CreateSemaphore failed 0x%x\n".as_ptr(),
            rc as i32,
        );

        fflush(__stderrp);
    }

    return (completed_req_event != INVALID_HANDLE_VALUE
        && abandon_req_wait != INVALID_HANDLE_VALUE
        && !completed_table_sema.is_null()) as i32;
}

unsafe fn shutdownAsyncIO(mut wait_threads: bool) {
    ShutdownIOManager(wait_threads);

    if completed_req_event != INVALID_HANDLE_VALUE {
        CloseHandle(completed_req_event);
        completed_req_event = INVALID_HANDLE_VALUE;
    }

    if abandon_req_wait != INVALID_HANDLE_VALUE {
        CloseHandle(abandon_req_wait);
        abandon_req_wait = INVALID_HANDLE_VALUE;
    }

    if !completed_table_sema.is_null() {
        CloseHandle(completed_table_sema);
        completed_table_sema = NULL as HANDLE;
    }
}

unsafe fn awaitRequests(mut wait: bool) -> i32 {
    panic!("Reached end of non-void function without returning");
}

unsafe fn abandonRequestWait() {
    SetEvent(abandon_req_wait);
    interruptIOManagerEvent();
}

unsafe fn resetAbandonRequestWait() {
    ResetEvent(abandon_req_wait);
}
