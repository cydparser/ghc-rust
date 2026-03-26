use crate::ffi::rts::os_threads::Mutex;
use crate::ffi::rts::os_threads::Mutex;
use crate::prelude::*;
use crate::rts_utils::{stgFree, stgMallocBytes};

pub(crate) type Semaphore = HANDLE;

/// cbindgen:no-export
pub(crate) struct WorkQueue {
    pub(crate) queueLock: Mutex,
    pub(crate) workAvailable: Semaphore,
    pub(crate) roomAvailable: Semaphore,
    pub(crate) head: c_int,
    pub(crate) tail: c_int,
    pub(crate) items: [*mut *mut c_void; 16],
}

pub(crate) const WORKQUEUE_SIZE: c_int = 16 as c_int;

unsafe fn newSemaphore(mut initCount: c_int, mut max: c_int) -> Semaphore {
    let mut s = null_mut::<c_void>();

    s = CreateSemaphoreA(
        null_mut::<_SECURITY_ATTRIBUTES>(),
        initCount as LONG,
        max as LONG,
        null::<CHAR>(),
    ) as Semaphore;

    if s.is_null() {
        queue_error_rc(
            b"newSemaphore\0" as *const u8 as *const c_char as *mut c_char,
            GetLastError(),
        );

        return NULL;
    }

    return s;
}

unsafe fn NewWorkQueue() -> *mut WorkQueue {
    let mut wq = stgMallocBytes(
        size_of::<WorkQueue>() as size_t,
        b"NewWorkQueue\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut WorkQueue;

    memset(
        wq as *mut c_void,
        0 as c_int,
        size_of::<WorkQueue>() as size_t,
    );

    InitializeSRWLock(&raw mut (*wq).queueLock);
    (*wq).workAvailable = newSemaphore(0 as c_int, WORKQUEUE_SIZE);
    (*wq).roomAvailable = newSemaphore(WORKQUEUE_SIZE, WORKQUEUE_SIZE);

    if (*wq).workAvailable.is_null() || (*wq).roomAvailable.is_null() {
        FreeWorkQueue(wq);

        return null_mut::<WorkQueue>();
    }

    return wq;
}

unsafe fn FreeWorkQueue(mut pq: *mut WorkQueue) {
    let mut i: c_int = 0;
    i = 0 as c_int;

    while i < WORKQUEUE_SIZE {
        if !(*pq).items[i as usize].is_null() {
            stgFree((*pq).items[i as usize] as *mut c_void);
        }

        i += 1;
    }

    if !(*pq).workAvailable.is_null() {
        CloseHandle((*pq).workAvailable as HANDLE);
    }

    if !(*pq).roomAvailable.is_null() {
        CloseHandle((*pq).roomAvailable as HANDLE);
    }

    stgFree(pq as *mut c_void);
}

unsafe fn GetWorkQueueHandle(mut pq: *mut WorkQueue) -> HANDLE {
    if pq.is_null() {
        return NULL;
    }

    return (*pq).workAvailable as HANDLE;
}

unsafe fn GetWork(mut pq: *mut WorkQueue, mut ppw: *mut *mut c_void) -> BOOL {
    let mut rc: DWORD = 0;

    if pq.is_null() {
        queue_error(
            b"GetWork\0" as *const u8 as *const c_char as *mut c_char,
            b"NULL WorkQueue object\0" as *const u8 as *const c_char as *mut c_char,
        );

        return r#false;
    }

    if ppw.is_null() {
        queue_error(
            b"GetWork\0" as *const u8 as *const c_char as *mut c_char,
            b"NULL WorkItem object\0" as *const u8 as *const c_char as *mut c_char,
        );

        return r#false;
    }

    rc = WaitForSingleObject((*pq).workAvailable as HANDLE, INFINITE);

    if rc != WAIT_OBJECT_0 {
        queue_error_rc(
            b"GetWork.WaitForSingleObject(workAvailable)\0" as *const u8 as *const c_char
                as *mut c_char,
            if WAIT_FAILED == rc {
                GetLastError()
            } else {
                rc
            },
        );

        return r#false;
    }

    return FetchWork(pq, ppw);
}

unsafe fn FetchWork(mut pq: *mut WorkQueue, mut ppw: *mut *mut c_void) -> BOOL {
    let mut rc: DWORD = 0;

    if pq.is_null() {
        queue_error(
            b"FetchWork\0" as *const u8 as *const c_char as *mut c_char,
            b"NULL WorkQueue object\0" as *const u8 as *const c_char as *mut c_char,
        );

        return r#false;
    }

    if ppw.is_null() {
        queue_error(
            b"FetchWork\0" as *const u8 as *const c_char as *mut c_char,
            b"NULL WorkItem object\0" as *const u8 as *const c_char as *mut c_char,
        );

        return r#false;
    }

    AcquireSRWLockExclusive(&raw mut (*pq).queueLock);
    *ppw = (*pq).items[(*pq).head as usize] as *mut c_void;
    (*pq).items[(*pq).head as usize] = null_mut::<*mut c_void>();
    (*pq).head = ((*pq).head + 1 as c_int) % WORKQUEUE_SIZE;

    rc = ReleaseSemaphore(
        (*pq).roomAvailable as HANDLE,
        1 as LONG,
        null_mut::<c_int>(),
    ) as DWORD;

    ReleaseSRWLockExclusive(&raw mut (*pq).queueLock);

    if 0 as DWORD == rc {
        queue_error_rc(
            b"FetchWork.ReleaseSemaphore()\0" as *const u8 as *const c_char as *mut c_char,
            GetLastError(),
        );

        return r#false;
    }

    return r#true;
}

unsafe fn SubmitWork(mut pq: *mut WorkQueue, mut pw: *mut c_void) -> c_int {
    let mut rc: DWORD = 0;

    if pq.is_null() {
        queue_error(
            b"SubmitWork\0" as *const u8 as *const c_char as *mut c_char,
            b"NULL WorkQueue object\0" as *const u8 as *const c_char as *mut c_char,
        );

        return r#false;
    }

    if pw.is_null() {
        queue_error(
            b"SubmitWork\0" as *const u8 as *const c_char as *mut c_char,
            b"NULL WorkItem object\0" as *const u8 as *const c_char as *mut c_char,
        );

        return r#false;
    }

    rc = WaitForSingleObject((*pq).roomAvailable as HANDLE, INFINITE);

    if rc != WAIT_OBJECT_0 {
        queue_error_rc(
            b"SubmitWork.WaitForSingleObject(workAvailable)\0" as *const u8 as *const c_char
                as *mut c_char,
            if WAIT_FAILED == rc {
                GetLastError()
            } else {
                rc
            },
        );

        return r#false;
    }

    AcquireSRWLockExclusive(&raw mut (*pq).queueLock);
    (*pq).items[(*pq).tail as usize] = pw as *mut *mut c_void;
    (*pq).tail = ((*pq).tail + 1 as c_int) % WORKQUEUE_SIZE;

    rc = ReleaseSemaphore(
        (*pq).workAvailable as HANDLE,
        1 as LONG,
        null_mut::<c_int>(),
    ) as DWORD;

    ReleaseSRWLockExclusive(&raw mut (*pq).queueLock);

    if 0 as DWORD == rc {
        queue_error_rc(
            b"SubmitWork.ReleaseSemaphore()\0" as *const u8 as *const c_char as *mut c_char,
            GetLastError(),
        );

        return r#false;
    }

    return r#true;
}

unsafe fn queue_error_rc(mut loc: *mut c_char, mut err: DWORD) {
    fprintf(
        __stderrp,
        b"%s failed: return code = 0x%lx\n\0" as *const u8 as *const c_char,
        loc,
        err,
    );

    fflush(__stderrp);
}

unsafe fn queue_error(mut loc: *mut c_char, mut reason: *mut c_char) {
    fprintf(
        __stderrp,
        b"%s failed: %s\n\0" as *const u8 as *const c_char,
        loc,
        reason,
    );

    fflush(__stderrp);
}
