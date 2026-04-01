use crate::capability::Capability_;
use crate::capability::{Capability_, n_numa_nodes, numa_map};
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::messages::{barf, debugBelch, errorBelch, sysErrorBelch};
use crate::ffi::rts::os_threads::{Condition, Mutex, OSThreadId};
use crate::ffi::rts::os_threads::{
    Condition, Mutex, OSThreadId, closeCondition, closeMutex, createOSThread, initCondition,
    initMutex, interruptOSThread, osThreadId, setThreadAffinity, setThreadNode,
};
use crate::ffi::rts::threads::{getNumCapabilities, n_capabilities};
use crate::ffi::rts::types::{StgClosure, StgTSO};
use crate::ffi::rts::types::{StgClosure, StgTSO};
use crate::ffi::rts::{_assertFail, DEBUG_IS_ON, stg_exit};
use crate::ffi::rts_api::{Capability, NoStatus, SchedulerStatus};
use crate::ffi::rts_api::{Capability, SchedulerStatus};
use crate::ffi::stg::types::StgWord64;
use crate::ffi::stg::types::StgWord64;
use crate::prelude::*;
use crate::rts_utils::{stgFree, stgMallocBytes};
use crate::schedule::scheduleWorker;
use crate::task::{
    InCall, InCall_, Task, Task_, TaskId, myTask, serialisableTaskId, serialiseTaskId, setMyTask,
};
use crate::trace::{DEBUG_RTS, trace_, traceTaskCreate, traceTaskDelete};

#[cfg(test)]
mod tests;

pub(crate) type Task = Task_;

/// cbindgen:no-export
pub(crate) struct Task_ {
    pub(crate) id: OSThreadId,
    pub(crate) node: u32,
    pub(crate) cond: Condition,
    pub(crate) lock: Mutex,
    pub(crate) wakeup: bool,
    pub(crate) cap: *mut Capability_,
    pub(crate) incall: *mut InCall_,
    pub(crate) n_spare_incalls: u32,
    pub(crate) spare_incalls: *mut InCall_,
    pub(crate) worker: bool,
    pub(crate) stopped: bool,
    pub(crate) running_finalizers: bool,
    pub(crate) preferred_capability: i32,
    pub(crate) next: *mut Task_,
    pub(crate) all_next: *mut Task_,
    pub(crate) all_prev: *mut Task_,
}

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub struct InCall_ {
    pub(crate) tso: *mut StgTSO,
    pub(crate) suspended_tso: *mut StgTSO,
    pub(crate) suspended_cap: *mut Capability,
    pub(crate) rstat: SchedulerStatus,
    pub(crate) ret: *mut *mut StgClosure,
    pub(crate) task: *mut Task_,
    pub(crate) prev_stack: *mut InCall_,
    pub(crate) prev: *mut InCall_,
    pub(crate) next: *mut InCall_,
}

pub(crate) type InCall = InCall_;

pub(crate) type TaskId = StgWord64;

#[inline]
pub(crate) unsafe fn isBoundTask(mut task: *mut Task) -> bool {
    return !(*(*task).incall).tso.is_null();
}

#[inline]
pub(crate) unsafe fn isWorker(mut task: *mut Task) -> bool {
    return (*task).worker as i32 != 0 && (*(*task).incall).prev_stack.is_null();
}

#[inline]
pub(crate) unsafe fn myTask() -> *mut Task {
    return my_task;
}

#[inline]
pub(crate) unsafe fn setMyTask(mut task: *mut Task) {
    my_task = task;
}

#[inline]
pub(crate) unsafe fn serialiseTaskId(mut taskID: OSThreadId) -> TaskId {
    return taskID as usize as TaskId;
}

#[inline]
pub(crate) unsafe fn serialisableTaskId(mut task: *mut Task) -> TaskId {
    return serialiseTaskId((*task).id);
}

static mut all_tasks: *mut Task = null_mut::<Task>();

static mut taskCount: u32 = 0;

static mut workerCount: u32 = 0;

static mut currentWorkerCount: u32 = 0;

static mut peakWorkerCount: u32 = 0;

static mut tasksInitialized: i32 = 0;

static mut all_tasks_mutex: Mutex = _opaque_pthread_mutex_t {
    __sig: 0,
    __opaque: [0; 56],
};

#[thread_local]
static mut my_task: *mut Task = null_mut::<Task>();

unsafe fn initTaskManager() {
    if tasksInitialized == 0 {
        taskCount = 0;
        workerCount = 0;
        currentWorkerCount = 0;
        peakWorkerCount = 0;
        tasksInitialized = 1;
        initMutex(&raw mut all_tasks_mutex);
    }
}

unsafe fn freeTaskManager() -> u32 {
    let mut task = null_mut::<Task>();
    let mut next = null_mut::<Task>();
    let mut tasksRunning: u32 = 0;
    let mut __r = pthread_mutex_lock(&raw mut all_tasks_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Task.c".as_ptr(),
            85,
            __r,
        );
    }

    task = all_tasks;

    while !task.is_null() {
        next = (*task).all_next as *mut Task;

        if (*task).stopped {
            freeTask(task);
        } else {
            tasksRunning = tasksRunning.wrapping_add(1);
        }

        task = next;
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
        trace_(
            c"freeing task manager, %d tasks still running".as_ptr(),
            tasksRunning,
        );
    }

    all_tasks = null_mut::<Task>();

    if pthread_mutex_unlock(&raw mut all_tasks_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Task.c".as_ptr(),
            101,
        );
    }

    closeMutex(&raw mut all_tasks_mutex);
    tasksInitialized = 0;

    return tasksRunning;
}

unsafe fn getMyTask() -> *mut Task {
    let mut task = null_mut::<Task>();
    task = myTask();

    if !task.is_null() {
        return task;
    } else {
        task = newTask(false);
        (*task).id = osThreadId();
        setMyTask(task);

        return task;
    };
}

unsafe fn freeMyTask() {
    let mut task = null_mut::<Task>();
    task = myTask();

    if task.is_null() {
        return;
    }

    if !(*task).stopped {
        errorBelch(c"freeMyTask() called, but the Task is not stopped; ignoring".as_ptr());

        return;
    }

    if (*task).worker {
        errorBelch(c"freeMyTask() called on a worker; ignoring".as_ptr());
        return;
    }

    let mut __r = pthread_mutex_lock(&raw mut all_tasks_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Task.c".as_ptr(),
            148,
            __r,
        );
    }

    if !(*task).all_prev.is_null() {
        (*(*task).all_prev).all_next = (*task).all_next;
    } else {
        all_tasks = (*task).all_next as *mut Task;
    }

    if !(*task).all_next.is_null() {
        (*(*task).all_next).all_prev = (*task).all_prev;
    }

    taskCount = taskCount.wrapping_sub(1);

    if pthread_mutex_unlock(&raw mut all_tasks_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Task.c".as_ptr(),
            161,
        );
    }

    freeTask(task);
    setMyTask(null_mut::<Task>());
}

unsafe fn freeTask(mut task: *mut Task) {
    let mut incall = null_mut::<InCall>();
    let mut next = null_mut::<InCall>();
    closeCondition(&raw mut (*task).cond);
    closeMutex(&raw mut (*task).lock);
    incall = (*task).incall as *mut InCall;

    while !incall.is_null() {
        next = (*incall).prev_stack as *mut InCall;
        stgFree(incall as *mut c_void);
        incall = next;
    }

    incall = (*task).spare_incalls as *mut InCall;

    while !incall.is_null() {
        next = (*incall).next as *mut InCall;
        stgFree(incall as *mut c_void);
        incall = next;
    }

    stgFree(task as *mut c_void);
}

unsafe fn newTask(mut worker: bool) -> *mut Task {
    let mut task = null_mut::<Task>();

    task = stgMallocBytes(
        (size_of::<Task>() as usize)
            .wrapping_add(63 as usize)
            .wrapping_div(64 as usize)
            .wrapping_mul(64 as usize),
        c"newTask".as_ptr(),
    ) as *mut Task;

    (*task).cap = null_mut::<Capability_>();
    (*task).worker = worker;
    (*task).stopped = true;
    (*task).running_finalizers = false;
    (*task).n_spare_incalls = 0;
    (*task).spare_incalls = null_mut::<InCall_>();
    (*task).incall = null_mut::<InCall_>();
    (*task).preferred_capability = -1;
    initCondition(&raw mut (*task).cond);
    initMutex(&raw mut (*task).lock);
    (*task).id = null_mut::<_opaque_pthread_t>();
    (*task).wakeup = false;
    (*task).node = 0;
    (*task).next = null_mut::<Task_>();

    let mut __r = pthread_mutex_lock(&raw mut all_tasks_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Task.c".as_ptr(),
            221,
            __r,
        );
    }

    (*task).all_prev = null_mut::<Task_>();
    (*task).all_next = all_tasks as *mut Task_;

    if !all_tasks.is_null() {
        (*all_tasks).all_prev = task as *mut Task_;
    }

    all_tasks = task;
    taskCount = taskCount.wrapping_add(1);

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
        trace_(c"new task (taskCount: %d)".as_ptr(), taskCount);
    }

    if worker {
        workerCount = workerCount.wrapping_add(1);
        currentWorkerCount = currentWorkerCount.wrapping_add(1);

        if currentWorkerCount > peakWorkerCount {
            peakWorkerCount = currentWorkerCount;
        }
    }

    if pthread_mutex_unlock(&raw mut all_tasks_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Task.c".as_ptr(),
            240,
        );
    }

    return task;
}

const MAX_SPARE_INCALLS: i32 = 8;

unsafe fn newInCall(mut task: *mut Task) {
    let mut incall = null_mut::<InCall>();

    if !(*task).spare_incalls.is_null() {
        incall = (*task).spare_incalls as *mut InCall;
        (*task).spare_incalls = (*incall).next;
        (*task).n_spare_incalls = (*task).n_spare_incalls.wrapping_sub(1);
    } else {
        incall = stgMallocBytes(size_of::<InCall>() as usize, c"newInCall".as_ptr()) as *mut InCall;
    }

    (*incall).tso = null_mut::<StgTSO>();
    (*incall).task = task as *mut Task_;
    (*incall).suspended_tso = null_mut::<StgTSO>();
    (*incall).suspended_cap = null_mut::<Capability>();
    (*incall).rstat = NoStatus;
    (*incall).ret = null_mut::<*mut StgClosure>();
    (*incall).next = null_mut::<InCall_>();
    (*incall).prev = null_mut::<InCall_>();
    (*incall).prev_stack = (*task).incall;
    (*task).incall = incall as *mut InCall_;
}

unsafe fn endInCall(mut task: *mut Task) {
    let mut incall = null_mut::<InCall>();
    incall = (*task).incall as *mut InCall;
    (*incall).tso = null_mut::<StgTSO>();
    (*task).incall = (*(*task).incall).prev_stack;

    if (*task).n_spare_incalls >= MAX_SPARE_INCALLS as u32 {
        stgFree(incall as *mut c_void);
    } else {
        (*incall).next = (*task).spare_incalls;
        (*task).spare_incalls = incall as *mut InCall_;
        (*task).n_spare_incalls = (*task).n_spare_incalls.wrapping_add(1);
    };
}

unsafe fn newBoundTask() -> *mut Task {
    let mut task = null_mut::<Task>();

    if tasksInitialized == 0 {
        errorBelch(c"newBoundTask: RTS is not initialised; call hs_init() first".as_ptr());

        stg_exit(EXIT_FAILURE);
    }

    task = getMyTask();
    (*task).stopped = false;
    newInCall(task);

    return task;
}

unsafe fn exitMyTask() {
    let mut task = myTask();

    if (osThreadId() == (*task).id) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Task.c".as_ptr(), 315);
    }

    endInCall(task);

    if (*task).incall.is_null() {
        (*task).stopped = true;
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
        trace_(c"task exiting".as_ptr());
    }
}

unsafe fn discardTasksExcept(mut keep: *mut Task) {
    let mut task = null_mut::<Task>();
    let mut next = null_mut::<Task>();
    let mut __r = pthread_mutex_lock(&raw mut all_tasks_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Task.c".as_ptr(),
            343,
            __r,
        );
    }

    task = all_tasks;

    while !task.is_null() {
        next = (*task).all_next as *mut Task;

        if task != keep {
            if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
                trace_(c"discarding task %zu".as_ptr(), (*task).id as usize);
            }

            initCondition(&raw mut (*task).cond);
            initMutex(&raw mut (*task).lock);
            freeTask(task);
        }

        task = next;
    }

    all_tasks = keep;
    (*keep).all_next = null_mut::<Task_>();
    (*keep).all_prev = null_mut::<Task_>();

    if pthread_mutex_unlock(&raw mut all_tasks_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Task.c".as_ptr(),
            373,
        );
    }
}

unsafe fn workerTaskStop(mut task: *mut Task) {
    let mut id = osThreadId();

    if ((*task).id == id) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Task.c".as_ptr(), 382);
    }

    if (myTask() == task) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Task.c".as_ptr(), 383);
    }

    let mut __r = pthread_mutex_lock(&raw mut all_tasks_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Task.c".as_ptr(),
            385,
            __r,
        );
    }

    if !(*task).all_prev.is_null() {
        (*(*task).all_prev).all_next = (*task).all_next;
    } else {
        all_tasks = (*task).all_next as *mut Task;
    }

    if !(*task).all_next.is_null() {
        (*(*task).all_next).all_prev = (*task).all_prev;
    }

    currentWorkerCount = currentWorkerCount.wrapping_sub(1);

    if pthread_mutex_unlock(&raw mut all_tasks_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Task.c".as_ptr(),
            398,
        );
    }

    traceTaskDelete(task);
    freeTask(task);
}

unsafe fn workerStart(mut task: *mut Task) -> *mut c_void {
    let mut cap = null_mut::<Capability>();
    let mut __r = pthread_mutex_lock(&raw mut (*task).lock);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Task.c".as_ptr(),
            415,
            __r,
        );
    }

    cap = (*task).cap as *mut Capability;

    if pthread_mutex_unlock(&raw mut (*task).lock) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Task.c".as_ptr(),
            417,
        );
    }

    if RtsFlags.ParFlags.setAffinity {
        setThreadAffinity((*cap).no, n_capabilities);
    }

    if RtsFlags.GcFlags.numa as i32 != 0 && !RtsFlags.DebugFlags.numa {
        setThreadNode(numa_map[(*task).node as usize]);
    }

    setMyTask(task);
    newInCall(task);
    traceTaskCreate(task, cap);
    scheduleWorker(cap, task);

    return NULL;
}

unsafe fn startWorkerTask(mut cap: *mut Capability) {
    let mut r: i32 = 0;
    let mut tid = null_mut::<_opaque_pthread_t>();
    let mut task = null_mut::<Task>();
    task = newTask(true);
    (*task).stopped = false;

    let mut __r = pthread_mutex_lock(&raw mut (*task).lock);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Task.c".as_ptr(),
            454,
            __r,
        );
    }

    (*task).cap = cap as *mut Capability_;
    (*task).node = (*cap).node;

    if (pthread_mutex_lock(&raw mut (*cap).lock) == 11) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Task.c".as_ptr(), 464);
    }

    (&raw mut (*cap).running_task).store(task, Ordering::Relaxed);

    let mut worker_name = c"ghc_worker".as_ptr();

    r = createOSThread(
        &raw mut tid,
        worker_name,
        transmute::<Option<unsafe extern "C" fn(*mut Task) -> *mut c_void>, Option<OSThreadProc>>(
            Some(workerStart as unsafe extern "C" fn(*mut Task) -> *mut c_void),
        ),
        task as *mut c_void,
    );

    if r != 0 {
        sysErrorBelch(c"failed to create OS thread".as_ptr());
        stg_exit(EXIT_FAILURE);
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
        trace_(c"new worker task (taskCount: %d)".as_ptr(), taskCount);
    }

    (*task).id = tid;

    if pthread_mutex_unlock(&raw mut (*task).lock) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Task.c".as_ptr(),
            497,
        );
    }
}

unsafe fn interruptWorkerTask(mut task: *mut Task) {
    if (osThreadId() != (*task).id) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Task.c".as_ptr(), 503);
    }

    if !(*(*task).incall).suspended_tso.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Task.c".as_ptr(), 504);
    }

    interruptOSThread((*task).id);

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
        trace_(
            c"interrupted worker task %#llx".as_ptr(),
            serialisableTaskId(task),
        );
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_setInCallCapability(
    mut preferred_capability: c_int,
    mut affinity: c_int,
) {
    let mut task = getMyTask();
    (*task).preferred_capability = preferred_capability;

    if affinity != 0 {
        if RtsFlags.ParFlags.setAffinity {
            setThreadAffinity(preferred_capability as u32, getNumCapabilities() as u32);
        }
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_pinThreadToNumaNode(mut node: c_int) {
    if RtsFlags.GcFlags.numa {
        let mut task = getMyTask();
        (*task).node = (node as u32).wrapping_rem(n_numa_nodes);

        if DEBUG_IS_ON == 0 || !RtsFlags.DebugFlags.numa {
            setThreadNode(numa_map[(*task).node as usize]);
        }
    }
}

unsafe fn printAllTasks() {
    let mut task = null_mut::<Task>();
    task = all_tasks;

    while !task.is_null() {
        debugBelch(
            c"task %#llx is %s, ".as_ptr(),
            serialisableTaskId(task),
            if (*task).stopped as i32 != 0 {
                c"stopped".as_ptr()
            } else {
                c"alive".as_ptr()
            },
        );

        if !(*task).stopped {
            if !(*task).cap.is_null() {
                debugBelch(c"on capability %d, ".as_ptr(), (*(*task).cap).no);
            }

            if !(*(*task).incall).tso.is_null() {
                debugBelch(
                    c"bound to thread %llu".as_ptr(),
                    (*(*(*task).incall).tso).id,
                );
            } else {
                debugBelch(c"worker".as_ptr());
            }
        }

        debugBelch(c"\n".as_ptr());
        task = (*task).all_next as *mut Task;
    }
}
