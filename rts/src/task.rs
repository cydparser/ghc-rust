use crate::capability::Capability_;
use crate::capability::Capability_;
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::messages::errorBelch;
use crate::ffi::rts::stg_exit;
use crate::ffi::rts::types::{StgClosure, StgTSO};
use crate::ffi::rts::types::{StgClosure, StgTSO};
use crate::ffi::rts_api::{Capability, NoStatus, SchedulerStatus};
use crate::ffi::rts_api::{Capability, SchedulerStatus};
use crate::ffi::stg::types::StgWord64;
use crate::ffi::stg::types::StgWord64;
use crate::prelude::*;
use crate::rts_utils::{stgFree, stgMallocBytes};
use crate::task::{InCall, InCall_, Task, Task_, TaskId, myTask, setMyTask};
use crate::trace::{DEBUG_RTS, trace_};

#[cfg(test)]
mod tests;

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

#[cfg(test)]
impl Arbitrary for InCall_ {
    fn arbitrary(g: &mut Gen) -> Self {
        InCall_ {
            _address: Arbitrary::arbitrary(g),
        }
    }
}

/// cbindgen:no-export
pub(crate) struct Task_ {
    pub(crate) cap: *mut Capability_,
    pub(crate) incall: *mut InCall_,
    pub(crate) n_spare_incalls: uint32_t,
    pub(crate) spare_incalls: *mut InCall_,
    pub(crate) worker: bool,
    pub(crate) stopped: bool,
    pub(crate) running_finalizers: bool,
    pub(crate) preferred_capability: c_int,
    pub(crate) next: *mut Task_,
    pub(crate) all_next: *mut Task_,
    pub(crate) all_prev: *mut Task_,
}

pub(crate) type InCall = InCall_;

pub(crate) type Task = Task_;

pub(crate) type TaskId = StgWord64;

#[inline]
pub(crate) unsafe fn isBoundTask(mut task: *mut Task) -> bool {
    return !(*(*task).incall).tso.is_null();
}

#[inline]
pub(crate) unsafe fn isWorker(mut task: *mut Task) -> bool {
    return (*task).worker as c_int != 0 && (*(*task).incall).prev_stack.is_null();
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
pub(crate) unsafe fn serialisableTaskId(mut task: *mut Task) -> TaskId {
    return task as uintptr_t as TaskId;
}

static mut all_tasks: *mut Task = null::<Task>() as *mut Task;

static mut taskCount: uint32_t = 0;

static mut workerCount: uint32_t = 0;

static mut currentWorkerCount: uint32_t = 0;

static mut peakWorkerCount: uint32_t = 0;

static mut tasksInitialized: c_int = 0 as c_int;

static mut my_task: *mut Task = null::<Task>() as *mut Task;

unsafe fn initTaskManager() {
    if tasksInitialized == 0 {
        taskCount = 0 as uint32_t;
        workerCount = 0 as uint32_t;
        currentWorkerCount = 0 as uint32_t;
        peakWorkerCount = 0 as uint32_t;
        tasksInitialized = 1 as c_int;
    }
}

unsafe fn freeTaskManager() -> uint32_t {
    let mut task = null_mut::<Task>();
    let mut next = null_mut::<Task>();
    let mut tasksRunning: uint32_t = 0 as uint32_t;
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

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as c_long != 0 {
        trace_(
            b"freeing task manager, %d tasks still running\0" as *const u8 as *const c_char
                as *mut c_char,
            tasksRunning,
        );
    }

    all_tasks = null_mut::<Task>();
    tasksInitialized = 0 as c_int;

    return tasksRunning;
}

unsafe fn getMyTask() -> *mut Task {
    let mut task = null_mut::<Task>();
    task = myTask();

    if !task.is_null() {
        return task;
    } else {
        task = newTask(r#false != 0);
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
        errorBelch(
            b"freeMyTask() called, but the Task is not stopped; ignoring\0" as *const u8
                as *const c_char,
        );

        return;
    }

    if (*task).worker {
        errorBelch(b"freeMyTask() called on a worker; ignoring\0" as *const u8 as *const c_char);
        return;
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
    freeTask(task);
    setMyTask(null_mut::<Task>());
}

unsafe fn freeTask(mut task: *mut Task) {
    let mut incall = null_mut::<InCall>();
    let mut next = null_mut::<InCall>();
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
        (size_of::<Task>() as size_t)
            .wrapping_add(63 as size_t)
            .wrapping_div(64 as size_t)
            .wrapping_mul(64 as size_t),
        b"newTask\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut Task;

    (*task).cap = null_mut::<Capability_>();
    (*task).worker = worker;
    (*task).stopped = r#true != 0;
    (*task).running_finalizers = r#false != 0;
    (*task).n_spare_incalls = 0 as uint32_t;
    (*task).spare_incalls = null_mut::<InCall_>();
    (*task).incall = null_mut::<InCall_>();
    (*task).preferred_capability = -(1 as c_int);
    (*task).next = null_mut::<Task_>();
    (*task).all_prev = null_mut::<Task_>();
    (*task).all_next = all_tasks as *mut Task_;

    if !all_tasks.is_null() {
        (*all_tasks).all_prev = task as *mut Task_;
    }

    all_tasks = task;
    taskCount = taskCount.wrapping_add(1);

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as c_long != 0 {
        trace_(
            b"new task (taskCount: %d)\0" as *const u8 as *const c_char as *mut c_char,
            taskCount,
        );
    }

    if worker {
        workerCount = workerCount.wrapping_add(1);
        currentWorkerCount = currentWorkerCount.wrapping_add(1);

        if currentWorkerCount > peakWorkerCount {
            peakWorkerCount = currentWorkerCount;
        }
    }

    return task;
}

const MAX_SPARE_INCALLS: c_int = 8 as c_int;

unsafe fn newInCall(mut task: *mut Task) {
    let mut incall = null_mut::<InCall>();

    if !(*task).spare_incalls.is_null() {
        incall = (*task).spare_incalls as *mut InCall;
        (*task).spare_incalls = (*incall).next;
        (*task).n_spare_incalls = (*task).n_spare_incalls.wrapping_sub(1);
    } else {
        incall = stgMallocBytes(
            size_of::<InCall>() as size_t,
            b"newInCall\0" as *const u8 as *const c_char as *mut c_char,
        ) as *mut InCall;
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

    if (*task).n_spare_incalls >= MAX_SPARE_INCALLS as uint32_t {
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
        errorBelch(
            b"newBoundTask: RTS is not initialised; call hs_init() first\0" as *const u8
                as *const c_char,
        );

        stg_exit(EXIT_FAILURE);
    }

    task = getMyTask();
    (*task).stopped = r#false != 0;
    newInCall(task);

    return task;
}

unsafe fn exitMyTask() {
    let mut task = myTask();
    endInCall(task);

    if (*task).incall.is_null() {
        (*task).stopped = r#true != 0;
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as c_long != 0 {
        trace_(b"task exiting\0" as *const u8 as *const c_char as *mut c_char);
    }
}

unsafe fn discardTasksExcept(mut keep: *mut Task) {
    let mut task = null_mut::<Task>();
    let mut next = null_mut::<Task>();
    task = all_tasks;

    while !task.is_null() {
        next = (*task).all_next as *mut Task;

        if task != keep {
            if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as c_long != 0 {
                trace_(
                    b"discarding task %zu\0" as *const u8 as *const c_char as *mut c_char,
                    task as size_t,
                );
            }

            freeTask(task);
        }

        task = next;
    }

    all_tasks = keep;
    (*keep).all_next = null_mut::<Task_>();
    (*keep).all_prev = null_mut::<Task_>();
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
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_pinThreadToNumaNode(mut node: c_int) {}
