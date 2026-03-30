use crate::capability::Capability_;
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::stg_exit;
use crate::ffi::rts::types::{StgClosure, StgTSO};
use crate::ffi::rts_api::{Capability, NoStatus, SchedulerStatus};
use crate::ffi::stg::types::StgWord64;
use crate::prelude::*;
use crate::rts_messages::errorBelch;
use crate::rts_utils::{stgFree, stgMallocBytes};
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

/// cbindgen:no-export
pub(crate) struct Task_ {
    pub(crate) cap: *mut Capability_,
    pub(crate) incall: *mut InCall_,
    pub(crate) n_spare_incalls: u32,
    pub(crate) spare_incalls: *mut InCall_,
    pub(crate) worker: bool,
    pub(crate) stopped: bool,
    pub(crate) running_finalizers: bool,
    pub(crate) preferred_capability: i32,
    pub(crate) next: *mut Task_,
    pub(crate) all_next: Option<NonNull<Task_>>,
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
pub(crate) unsafe fn serialisableTaskId(mut task: *mut Task) -> TaskId {
    return task as usize as TaskId;
}

static mut all_tasks: Option<NonNull<Task>> = None;

static mut taskCount: u32 = 0;

static mut workerCount: u32 = 0;

static mut currentWorkerCount: u32 = 0;

static mut peakWorkerCount: u32 = 0;

static mut tasksInitialized: i32 = 0;

static mut my_task: *mut Task = null_mut::<Task>();

unsafe fn initTaskManager() {
    if tasksInitialized == 0 {
        taskCount = 0;
        workerCount = 0;
        currentWorkerCount = 0;
        peakWorkerCount = 0;
        tasksInitialized = 1;
    }
}

unsafe fn freeTaskManager() -> u32 {
    let mut maybe_task = all_tasks;
    let mut tasksRunning: u32 = 0;

    while let Some(task) = maybe_task {
        let next = task.all_next;

        if (*task).stopped {
            freeTask(task.as_ptr());
        } else {
            tasksRunning = tasksRunning.wrapping_add(1);
        }

        maybe_task = next;
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
        trace_(
            c"freeing task manager, %d tasks still running".as_ptr(),
            tasksRunning,
        );
    }

    all_tasks = None;
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

    if !(*task).all_prev.is_null() {
        (*(*task).all_prev).all_next = (*task).all_next;
    } else {
        all_tasks = (*task).all_next;
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
    let mut task = stgMallocBytes(
        (size_of::<Task>() as usize)
            .wrapping_add(63 as usize)
            .wrapping_div(64 as usize)
            .wrapping_mul(64 as usize),
        c"newTask",
    )
    .cast();

    (*task).cap = null_mut::<Capability_>();
    (*task).worker = worker;
    (*task).stopped = true;
    (*task).running_finalizers = false;
    (*task).n_spare_incalls = 0;
    (*task).spare_incalls = null_mut::<InCall_>();
    (*task).incall = null_mut::<InCall_>();
    (*task).preferred_capability = -1;
    (*task).next = null_mut::<Task_>();
    (*task).all_prev = null_mut::<Task_>();
    (*task).all_next = all_tasks;

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
        incall = stgMallocBytes(size_of::<InCall>() as usize, c"newInCall") as *mut InCall;
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
    task = all_tasks;

    while !task.is_null() {
        next = (*task).all_next as *mut Task;

        if task != keep {
            if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
                trace_(c"discarding task %zu".as_ptr(), task as usize);
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
pub unsafe extern "C" fn rts_setInCallCapability(mut preferred_capability: i32, mut affinity: i32) {
    let mut task = getMyTask();
    (*task).preferred_capability = preferred_capability;
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_pinThreadToNumaNode(mut node: i32) {}
