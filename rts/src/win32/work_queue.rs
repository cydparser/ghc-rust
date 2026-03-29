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
    pub(crate) head: i32,
    pub(crate) tail: i32,
    pub(crate) items: [*mut *mut c_void; 16],
}

pub(crate) const WORKQUEUE_SIZE: i32 = 16;

unsafe fn newSemaphore(mut initCount: i32, mut max: i32) -> Semaphore {
    let mut s = null_mut::<c_void>();

    s = CreateSemaphoreA(
        null_mut::<_SECURITY_ATTRIBUTES>(),
        initCount as LONG,
        max as LONG,
        null::<CHAR>(),
    ) as Semaphore;

    if s.is_null() {
        queue_error_rc(c"newSemaphore".as_ptr(), GetLastError());

        return NULL;
    }

    return s;
}

unsafe fn NewWorkQueue() -> *mut WorkQueue {
    let mut wq =
        stgMallocBytes(size_of::<WorkQueue>() as usize, c"NewWorkQueue".as_ptr()) as *mut WorkQueue;

    memset(wq as *mut c_void, 0, size_of::<WorkQueue>() as usize);
    InitializeSRWLock(&raw mut (*wq).queueLock);
    (*wq).workAvailable = newSemaphore(0, WORKQUEUE_SIZE);
    (*wq).roomAvailable = newSemaphore(WORKQUEUE_SIZE, WORKQUEUE_SIZE);

    if (*wq).workAvailable.is_null() || (*wq).roomAvailable.is_null() {
        FreeWorkQueue(wq);

        return null_mut::<WorkQueue>();
    }

    return wq;
}

unsafe fn FreeWorkQueue(mut pq: *mut WorkQueue) {
    let mut i: i32 = 0;
    i = 0;

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
        queue_error(c"GetWork".as_ptr(), c"NULL WorkQueue object".as_ptr());

        return false;
    }

    if ppw.is_null() {
        queue_error(c"GetWork".as_ptr(), c"NULL WorkItem object".as_ptr());

        return false;
    }

    rc = WaitForSingleObject((*pq).workAvailable as HANDLE, INFINITE);

    if rc != WAIT_OBJECT_0 {
        queue_error_rc(
            c"GetWork.WaitForSingleObject(workAvailable)".as_ptr(),
            if WAIT_FAILED == rc {
                GetLastError()
            } else {
                rc
            },
        );

        return false;
    }

    return FetchWork(pq, ppw);
}

unsafe fn FetchWork(mut pq: *mut WorkQueue, mut ppw: *mut *mut c_void) -> BOOL {
    let mut rc: DWORD = 0;

    if pq.is_null() {
        queue_error(c"FetchWork".as_ptr(), c"NULL WorkQueue object".as_ptr());

        return false;
    }

    if ppw.is_null() {
        queue_error(c"FetchWork".as_ptr(), c"NULL WorkItem object".as_ptr());

        return false;
    }

    AcquireSRWLockExclusive(&raw mut (*pq).queueLock);
    *ppw = (*pq).items[(*pq).head as usize] as *mut c_void;
    (*pq).items[(*pq).head as usize] = null_mut::<*mut c_void>();
    (*pq).head = ((*pq).head + 1) % WORKQUEUE_SIZE;
    rc = ReleaseSemaphore((*pq).roomAvailable as HANDLE, 1, null_mut::<i32>()) as DWORD;
    ReleaseSRWLockExclusive(&raw mut (*pq).queueLock);

    if 0 == rc {
        queue_error_rc(c"FetchWork.ReleaseSemaphore()".as_ptr(), GetLastError());

        return false;
    }

    return true;
}

unsafe fn SubmitWork(mut pq: *mut WorkQueue, mut pw: *mut c_void) -> i32 {
    let mut rc: DWORD = 0;

    if pq.is_null() {
        queue_error(c"SubmitWork".as_ptr(), c"NULL WorkQueue object".as_ptr());

        return false;
    }

    if pw.is_null() {
        queue_error(c"SubmitWork".as_ptr(), c"NULL WorkItem object".as_ptr());

        return false;
    }

    rc = WaitForSingleObject((*pq).roomAvailable as HANDLE, INFINITE);

    if rc != WAIT_OBJECT_0 {
        queue_error_rc(
            c"SubmitWork.WaitForSingleObject(workAvailable)".as_ptr(),
            if WAIT_FAILED == rc {
                GetLastError()
            } else {
                rc
            },
        );

        return false;
    }

    AcquireSRWLockExclusive(&raw mut (*pq).queueLock);
    (*pq).items[(*pq).tail as usize] = pw as *mut *mut c_void;
    (*pq).tail = ((*pq).tail + 1) % WORKQUEUE_SIZE;
    rc = ReleaseSemaphore((*pq).workAvailable as HANDLE, 1, null_mut::<i32>()) as DWORD;
    ReleaseSRWLockExclusive(&raw mut (*pq).queueLock);

    if 0 == rc {
        queue_error_rc(c"SubmitWork.ReleaseSemaphore()".as_ptr(), GetLastError());

        return false;
    }

    return true;
}

unsafe fn queue_error_rc(mut loc: *mut c_char, mut err: DWORD) {
    fprintf(
        __stderrp,
        c"%s failed: return code = 0x%lx\n".as_ptr(),
        loc,
        err,
    );
    fflush(__stderrp);
}

unsafe fn queue_error(mut loc: *mut c_char, mut reason: *mut c_char) {
    fprintf(__stderrp, c"%s failed: %s\n".as_ptr(), loc, reason);
    fflush(__stderrp);
}
