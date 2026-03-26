use crate::ffi::rts::messages::sysErrorBelch;
use crate::ffi::rts::stg_exit;
use crate::ffi::rts::time::Time;
use crate::prelude::*;
use crate::ticker::TickProc;

pub(crate) type TickProc = Option<unsafe extern "C" fn(c_int) -> ()>;

static mut tick_proc: TickProc = None;

static mut timer_queue: HANDLE = NULL;

static mut timer: HANDLE = NULL;

static mut tick_interval: Time = 0 as Time;

unsafe fn tick_callback(mut lpParameter: PVOID, mut TimerOrWaitFired: BOOLEAN) {
    tick_proc.expect("non-null function pointer")(0 as c_int);
}

unsafe fn initTicker(mut interval: Time, mut handle_tick: TickProc) {
    tick_interval = interval;
    tick_proc = handle_tick;
    timer_queue = CreateTimerQueue();

    if timer_queue.is_null() {
        sysErrorBelch(b"CreateTimerQueue\0" as *const u8 as *const c_char);
        stg_exit(EXIT_FAILURE);
    }
}

unsafe fn startTicker() {
    let mut r: BOOL = 0;

    r = CreateTimerQueueTimer(
        &raw mut timer,
        timer_queue,
        Some(tick_callback as unsafe extern "C" fn(PVOID, BOOLEAN) -> ()),
        null_mut::<c_void>(),
        0 as DWORD,
        (tick_interval / 1000000 as Time) as DWORD,
        WT_EXECUTEINTIMERTHREAD as ULONG,
    ) as BOOL;

    if r == 0 as c_int {
        sysErrorBelch(b"CreateTimerQueueTimer\0" as *const u8 as *const c_char);
        stg_exit(EXIT_FAILURE);
    }
}

unsafe fn stopTicker() {
    if !timer_queue.is_null() && !timer.is_null() {
        DeleteTimerQueueTimer(timer_queue, timer, NULL);
        timer = NULL as HANDLE;
    }
}

unsafe fn exitTicker(mut wait: bool) {
    stopTicker();

    if !timer_queue.is_null() {
        DeleteTimerQueueEx(
            timer_queue,
            if wait as c_int != 0 {
                INVALID_HANDLE_VALUE
            } else {
                NULL
            },
        );

        timer_queue = NULL as HANDLE;
    }
}
