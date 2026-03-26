use crate::ffi::hs_ffi::HsWord32;
use crate::ffi::rts::_assertFail;
use crate::ffi::rts::io_interface::{IO_MANAGER_DIE, readIOManagerEvent};
use crate::ffi::rts::messages::barf;
use crate::prelude::*;
use crate::win32::thr_io_manager::{ioManagerDie, ioManagerStart, ioManagerWakeup};

static mut completionPortHandle: HANDLE = unsafe { INVALID_HANDLE_VALUE };

static mut running: bool = r#false != 0;

static mut outstanding_service_requests: bool = r#false != 0;

static mut queue_full: bool = r#false != 0;

static mut timeout: DWORD = INFINITE;

static mut workerThread: HANDLE = NULL;

static mut workerThreadId: DWORD = 0 as DWORD;

static mut wio_runner_lock: SRWLOCK = _RTL_SRWLOCK {
    Ptr: null::<c_void>() as *mut c_void,
};

static mut wakeEvent: CONDITION_VARIABLE = _RTL_CONDITION_VARIABLE {
    Ptr: null::<c_void>() as *mut c_void,
};

static mut threadIOWait: CONDITION_VARIABLE = _RTL_CONDITION_VARIABLE {
    Ptr: null::<c_void>() as *mut c_void,
};

static mut num_callbacks: uint32_t = 32 as uint32_t;

static mut entries: *mut OVERLAPPED_ENTRY = null::<OVERLAPPED_ENTRY>() as *mut OVERLAPPED_ENTRY;

static mut num_notify: uint32_t = 0;

static mut canQueueIOThread: bool = false;

unsafe fn startupAsyncWinIO() -> bool {
    if !running as c_int as c_long != 0 {
    } else {
        _assertFail(
            b"/Users/cyd/src/ghc/rts/win32/AsyncWinIO.c\0" as *const u8 as *const c_char,
            227 as c_uint,
        );
    }

    running = r#true != 0;
    write_volatile(&mut outstanding_service_requests as *mut bool, r#false != 0);
    completionPortHandle = INVALID_HANDLE_VALUE;
    InitializeSRWLock(&raw mut wio_runner_lock);
    InitializeConditionVariable(&raw mut wakeEvent);
    InitializeConditionVariable(&raw mut threadIOWait);

    entries = calloc(
        size_of::<OVERLAPPED_ENTRY>() as size_t,
        num_callbacks as size_t,
    ) as *mut OVERLAPPED_ENTRY;

    ioManagerStart();

    workerThread = CreateThread(
        null_mut::<_SECURITY_ATTRIBUTES>(),
        0 as SIZE_T,
        Some(runner as unsafe extern "C" fn(LPVOID) -> DWORD),
        NULL,
        0 as DWORD,
        &raw mut workerThreadId,
    );

    if workerThread.is_null() {
        barf(b"could not create I/O manager thread.\0" as *const u8 as *const c_char);
    }

    return r#true != 0;
}

unsafe fn shutdownAsyncWinIO(mut wait_threads: bool) {
    if !workerThread.is_null() {
        if wait_threads {
            AcquireSRWLockExclusive(&raw mut wio_runner_lock);
            running = r#false != 0;
            ioManagerWakeup();

            PostQueuedCompletionStatus(
                completionPortHandle,
                0 as DWORD,
                0 as ULONG_PTR,
                null_mut::<_OVERLAPPED>(),
            );

            WakeConditionVariable(&raw mut wakeEvent);
            WakeConditionVariable(&raw mut threadIOWait);
            ReleaseSRWLockExclusive(&raw mut wio_runner_lock);
            WaitForSingleObject(workerThread, INFINITE);
        }

        completionPortHandle = INVALID_HANDLE_VALUE;
        CloseHandle(workerThread);
        workerThread = NULL as HANDLE;
        workerThreadId = 0 as DWORD;
        free(entries as *mut c_void);
        entries = null_mut::<OVERLAPPED_ENTRY>();
    }

    ioManagerDie();
}

unsafe fn registerIOCPHandle(mut port: HANDLE) {
    AcquireSRWLockExclusive(&raw mut wio_runner_lock);
    completionPortHandle = port;
    ReleaseSRWLockExclusive(&raw mut wio_runner_lock);
}

unsafe fn completeSynchronousRequest() {
    AcquireSRWLockExclusive(&raw mut wio_runner_lock);
    WakeConditionVariable(&raw mut threadIOWait);
    ReleaseSRWLockExclusive(&raw mut wio_runner_lock);
}

unsafe fn registerAlertableWait(mut has_timeout: bool, mut mssec: DWORD) {
    if (completionPortHandle != -(1 as c_int) as LONG_PTR as HANDLE) as c_int as c_long != 0 {
    } else {
        _assertFail(
            b"/Users/cyd/src/ghc/rts/win32/AsyncWinIO.c\0" as *const u8 as *const c_char,
            330 as c_uint,
        );
    }

    AcquireSRWLockExclusive(&raw mut wio_runner_lock);

    let mut interrupt = r#false != 0;

    if mssec == 0 as DWORD && !has_timeout {
        timeout = INFINITE as DWORD;
    } else if has_timeout {
        timeout = mssec;
    }

    write_volatile(&mut outstanding_service_requests as *mut bool, r#false != 0);

    if queue_full {
        num_callbacks = num_callbacks.wrapping_mul(2 as uint32_t);

        let mut new = realloc(
            entries as *mut c_void,
            (size_of::<OVERLAPPED_ENTRY>() as size_t).wrapping_mul(num_callbacks as size_t),
        ) as *mut OVERLAPPED_ENTRY;

        if !new.is_null() {
            entries = new;
        }

        queue_full = r#false != 0;
    }

    if timeout > mssec && mssec > 0 as DWORD {
        timeout = mssec;
        interrupt = r#true != 0;
    }

    ReleaseSRWLockExclusive(&raw mut wio_runner_lock);
    WakeConditionVariable(&raw mut wakeEvent);

    if interrupt {
        PostQueuedCompletionStatus(
            completionPortHandle,
            0 as DWORD,
            0 as ULONG_PTR,
            null_mut::<_OVERLAPPED>(),
        );
    }
}

unsafe fn getOverlappedEntries(mut num: *mut uint32_t) -> *mut OVERLAPPED_ENTRY {
    *num = num_notify;

    return entries;
}

unsafe fn awaitAsyncRequests(mut wait: bool) {
    if queueIOThread() {
        return;
    }

    AcquireSRWLockExclusive(&raw mut wio_runner_lock);

    if wait as c_int != 0 && outstanding_service_requests as c_int != 0 {
        SleepConditionVariableSRW(
            &raw mut threadIOWait,
            &raw mut wio_runner_lock,
            INFINITE,
            0 as ULONG,
        );
    }

    ReleaseSRWLockExclusive(&raw mut wio_runner_lock);
}

unsafe fn notifyScheduler(mut num: uint32_t) {
    AcquireSRWLockExclusive(&raw mut wio_runner_lock);

    if !canQueueIOThread as c_int as c_long != 0 {
    } else {
        _assertFail(
            b"/Users/cyd/src/ghc/rts/win32/AsyncWinIO.c\0" as *const u8 as *const c_char,
            420 as c_uint,
        );
    }

    num_notify = num;
    write_volatile(&mut canQueueIOThread as *mut bool, r#true != 0);
    WakeConditionVariable(&raw mut threadIOWait);
    ReleaseSRWLockExclusive(&raw mut wio_runner_lock);
}

unsafe fn queueIOThread() -> bool {
    let mut result = r#false != 0;

    return result;
}

unsafe fn runner(mut lpParam: LPVOID) -> DWORD {
    let mut lastEvent: HsWord32 = 0 as HsWord32;

    while running {
        AcquireSRWLockExclusive(&raw mut wio_runner_lock);
        lastEvent = readIOManagerEvent();

        while completionPortHandle == INVALID_HANDLE_VALUE
            || lastEvent == IO_MANAGER_DIE as HsWord32
            || outstanding_service_requests as c_int != 0
            || canQueueIOThread as c_int != 0
        {
            SleepConditionVariableSRW(
                &raw mut wakeEvent,
                &raw mut wio_runner_lock,
                INFINITE,
                0 as ULONG,
            );

            let mut nextEvent = readIOManagerEvent();
            lastEvent = if nextEvent != 0 { nextEvent } else { lastEvent };
        }

        ReleaseSRWLockExclusive(&raw mut wio_runner_lock);

        let mut num_removed = 0 as ULONG;

        memset(
            entries as *mut c_void,
            0 as c_int,
            (size_of::<OVERLAPPED_ENTRY>() as size_t).wrapping_mul(num_callbacks as size_t),
        );

        if GetQueuedCompletionStatusEx(
            completionPortHandle,
            entries as LPOVERLAPPED_ENTRY,
            num_callbacks as ULONG,
            &raw mut num_removed,
            timeout,
            r#false,
        ) != 0
        {
            if num_removed > 0 as ULONG {
                queue_full = num_removed as uint32_t == num_callbacks;
            }
        } else if 258 as DWORD == GetLastError() {
            num_removed = 0 as ULONG;
        }

        notifyScheduler(num_removed as uint32_t);
        AcquireSRWLockExclusive(&raw mut wio_runner_lock);

        if !running {
            ExitThread(0 as DWORD);
        }

        ReleaseSRWLockExclusive(&raw mut wio_runner_lock);
    }

    return 0 as DWORD;
}
