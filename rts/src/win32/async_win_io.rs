use crate::ffi::rts::_assertFail;
use crate::ffi::rts::io_interface::{IO_MANAGER_DIE, readIOManagerEvent};
use crate::ffi::rts::messages::barf;
use crate::hs_ffi::HsWord32;
use crate::prelude::*;
use crate::win32::thr_io_manager::{ioManagerDie, ioManagerStart, ioManagerWakeup};

static mut completionPortHandle: HANDLE = unsafe { INVALID_HANDLE_VALUE };

static mut running: bool = false;

static mut outstanding_service_requests: bool = false;

static mut queue_full: bool = false;

static mut timeout: DWORD = INFINITE;

static mut workerThread: HANDLE = NULL;

static mut workerThreadId: DWORD = 0;

static mut wio_runner_lock: SRWLOCK = _RTL_SRWLOCK {
    Ptr: null_mut::<c_void>(),
};

static mut wakeEvent: CONDITION_VARIABLE = _RTL_CONDITION_VARIABLE {
    Ptr: null_mut::<c_void>(),
};

static mut threadIOWait: CONDITION_VARIABLE = _RTL_CONDITION_VARIABLE {
    Ptr: null_mut::<c_void>(),
};

static mut num_callbacks: u32 = 32;

static mut entries: *mut OVERLAPPED_ENTRY = null_mut::<OVERLAPPED_ENTRY>();

static mut num_notify: u32 = 0;

static mut canQueueIOThread: bool = false;

unsafe fn startupAsyncWinIO() -> bool {
    if !running as i32 as i64 != 0 {
    } else {
        _assertFail(c"/Users/cyd/src/ghc/rts/win32/AsyncWinIO.c".as_ptr(), 227);
    }

    running = true;
    write_volatile(&mut outstanding_service_requests as *mut bool, false);
    completionPortHandle = INVALID_HANDLE_VALUE;
    InitializeSRWLock(&raw mut wio_runner_lock);
    InitializeConditionVariable(&raw mut wakeEvent);
    InitializeConditionVariable(&raw mut threadIOWait);
    entries = calloc(
        size_of::<OVERLAPPED_ENTRY>() as usize,
        num_callbacks as usize,
    ) as *mut OVERLAPPED_ENTRY;
    ioManagerStart();

    workerThread = CreateThread(
        null_mut::<_SECURITY_ATTRIBUTES>(),
        0,
        Some(runner as unsafe extern "C" fn(LPVOID) -> DWORD),
        NULL,
        0,
        &raw mut workerThreadId,
    );

    if workerThread.is_null() {
        barf(c"could not create I/O manager thread.".as_ptr());
    }

    return true;
}

unsafe fn shutdownAsyncWinIO(mut wait_threads: bool) {
    if !workerThread.is_null() {
        if wait_threads {
            AcquireSRWLockExclusive(&raw mut wio_runner_lock);
            running = false;
            ioManagerWakeup();

            PostQueuedCompletionStatus(completionPortHandle, 0, 0, null_mut::<_OVERLAPPED>());

            WakeConditionVariable(&raw mut wakeEvent);
            WakeConditionVariable(&raw mut threadIOWait);
            ReleaseSRWLockExclusive(&raw mut wio_runner_lock);
            WaitForSingleObject(workerThread, INFINITE);
        }

        completionPortHandle = INVALID_HANDLE_VALUE;
        CloseHandle(workerThread);
        workerThread = NULL as HANDLE;
        workerThreadId = 0;
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
    if (completionPortHandle != -1 as LONG_PTR as HANDLE) as i32 as i64 != 0 {
    } else {
        _assertFail(c"/Users/cyd/src/ghc/rts/win32/AsyncWinIO.c".as_ptr(), 330);
    }

    AcquireSRWLockExclusive(&raw mut wio_runner_lock);

    let mut interrupt = false;

    if mssec == 0 && !has_timeout {
        timeout = INFINITE as DWORD;
    } else if has_timeout {
        timeout = mssec;
    }

    write_volatile(&mut outstanding_service_requests as *mut bool, false);

    if queue_full {
        num_callbacks = num_callbacks.wrapping_mul(2 as u32);

        let mut new = realloc(
            entries as *mut c_void,
            (size_of::<OVERLAPPED_ENTRY>() as usize).wrapping_mul(num_callbacks as usize),
        ) as *mut OVERLAPPED_ENTRY;

        if !new.is_null() {
            entries = new;
        }

        queue_full = false;
    }

    if timeout > mssec && mssec > 0 {
        timeout = mssec;
        interrupt = true;
    }

    ReleaseSRWLockExclusive(&raw mut wio_runner_lock);
    WakeConditionVariable(&raw mut wakeEvent);

    if interrupt {
        PostQueuedCompletionStatus(completionPortHandle, 0, 0, null_mut::<_OVERLAPPED>());
    }
}

unsafe fn getOverlappedEntries(mut num: *mut u32) -> *mut OVERLAPPED_ENTRY {
    *num = num_notify;

    return entries;
}

unsafe fn awaitAsyncRequests(mut wait: bool) {
    if queueIOThread() {
        return;
    }

    AcquireSRWLockExclusive(&raw mut wio_runner_lock);

    if wait as i32 != 0 && outstanding_service_requests as i32 != 0 {
        SleepConditionVariableSRW(&raw mut threadIOWait, &raw mut wio_runner_lock, INFINITE, 0);
    }

    ReleaseSRWLockExclusive(&raw mut wio_runner_lock);
}

unsafe fn notifyScheduler(mut num: u32) {
    AcquireSRWLockExclusive(&raw mut wio_runner_lock);

    if !canQueueIOThread as i32 as i64 != 0 {
    } else {
        _assertFail(c"/Users/cyd/src/ghc/rts/win32/AsyncWinIO.c".as_ptr(), 420);
    }

    num_notify = num;
    write_volatile(&mut canQueueIOThread as *mut bool, true);
    WakeConditionVariable(&raw mut threadIOWait);
    ReleaseSRWLockExclusive(&raw mut wio_runner_lock);
}

unsafe fn queueIOThread() -> bool {
    let mut result = false;

    return result;
}

unsafe fn runner(mut lpParam: LPVOID) -> DWORD {
    let mut lastEvent: HsWord32 = 0;

    while running {
        AcquireSRWLockExclusive(&raw mut wio_runner_lock);
        lastEvent = readIOManagerEvent();

        while completionPortHandle == INVALID_HANDLE_VALUE
            || lastEvent == IO_MANAGER_DIE as HsWord32
            || outstanding_service_requests as i32 != 0
            || canQueueIOThread as i32 != 0
        {
            SleepConditionVariableSRW(&raw mut wakeEvent, &raw mut wio_runner_lock, INFINITE, 0);

            let mut nextEvent = readIOManagerEvent();
            lastEvent = if nextEvent != 0 { nextEvent } else { lastEvent };
        }

        ReleaseSRWLockExclusive(&raw mut wio_runner_lock);

        let mut num_removed = 0;

        memset(
            entries as *mut c_void,
            0,
            (size_of::<OVERLAPPED_ENTRY>() as usize).wrapping_mul(num_callbacks as usize),
        );

        if GetQueuedCompletionStatusEx(
            completionPortHandle,
            entries as LPOVERLAPPED_ENTRY,
            num_callbacks as ULONG,
            &raw mut num_removed,
            timeout,
            false,
        ) != 0
        {
            if num_removed > 0 {
                queue_full = num_removed as u32 == num_callbacks;
            }
        } else if 258 == GetLastError() {
            num_removed = 0;
        }

        notifyScheduler(num_removed as u32);
        AcquireSRWLockExclusive(&raw mut wio_runner_lock);

        if !running {
            ExitThread(0);
        }

        ReleaseSRWLockExclusive(&raw mut wio_runner_lock);
    }

    return 0;
}
