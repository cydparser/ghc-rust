use crate::capability::contextSwitchAllCapabilities;
use crate::ffi::rts::event_log_writer::{EVENTLOG_RUNNING, eventLogStatus, flushEventLog};
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::time::Time;
use crate::ffi::stg::types::StgWord;
use crate::prelude::*;
use crate::proftimer::{handleProfTick, initProfTimer};
use crate::schedule::{
    ACTIVITY_DONE_GC, ACTIVITY_INACTIVE, ACTIVITY_MAYBE_NO, getRecentActivity, setRecentActivity,
};
use crate::ticker::{exitTicker, initTicker, startTicker, stopTicker};

#[cfg(test)]
mod tests;

static mut timer_disabled: StgWord = 0;

static mut ticks_to_ctxt_switch: i32 = 0;

static mut ticks_to_eventlog_flush: i32 = 0;

static mut idle_ticks_to_gc: i32 = 0;

static mut inter_gc_ticks_to_gc: i32 = 0;

unsafe fn handle_tick(mut unused: i32) {
    handleProfTick();

    if RtsFlags.ConcFlags.ctxtSwitchTicks > 0
        && (&raw mut timer_disabled).load(Ordering::SeqCst) == 0
    {
        ticks_to_ctxt_switch -= 1;

        if ticks_to_ctxt_switch <= 0 {
            ticks_to_ctxt_switch = RtsFlags.ConcFlags.ctxtSwitchTicks;
            contextSwitchAllCapabilities();
        }
    }

    if eventLogStatus() as u32 == EVENTLOG_RUNNING as i32 as u32
        && RtsFlags.TraceFlags.eventlogFlushTicks > 0
    {
        ticks_to_eventlog_flush -= 1;

        if ticks_to_eventlog_flush <= 0 {
            ticks_to_eventlog_flush = RtsFlags.TraceFlags.eventlogFlushTicks;
            flushEventLog(null_mut::<*mut Capability>());
        }
    }

    match getRecentActivity() as u32 {
        0 => {
            setRecentActivity(ACTIVITY_MAYBE_NO);
            idle_ticks_to_gc =
                (RtsFlags.GcFlags.idleGCDelayTime / RtsFlags.MiscFlags.tickInterval) as i32;
        }
        1 => {
            if idle_ticks_to_gc == 0 && inter_gc_ticks_to_gc == 0 {
                if RtsFlags.GcFlags.doIdleGC {
                    setRecentActivity(ACTIVITY_INACTIVE);
                    inter_gc_ticks_to_gc =
                        (RtsFlags.GcFlags.interIdleGCWait / RtsFlags.MiscFlags.tickInterval) as i32;
                } else {
                    setRecentActivity(ACTIVITY_DONE_GC);
                    stopTimer();
                }
            } else {
                if idle_ticks_to_gc != 0 {
                    idle_ticks_to_gc -= 1;
                }

                if inter_gc_ticks_to_gc != 0 {
                    inter_gc_ticks_to_gc -= 1;
                }
            }
        }
        _ => {}
    };
}

unsafe fn initTimer() {
    initProfTimer();

    if RtsFlags.MiscFlags.tickInterval != 0 {
        initTicker(
            RtsFlags.MiscFlags.tickInterval,
            Some(handle_tick as unsafe extern "C" fn(c_int) -> ()),
        );
    }

    (&raw mut timer_disabled).store(1, Ordering::SeqCst);
}

#[ffi(libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn startTimer() {
    let fresh5 = &raw mut timer_disabled;
    let fresh6 = 1;

    if (fresh5).xsub(fresh6, Ordering::SeqCst) - fresh6 == 0 {
        if RtsFlags.MiscFlags.tickInterval != 0 {
            startTicker();
        }
    }
}

#[ffi(libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn stopTimer() {
    let fresh7 = &raw mut timer_disabled;
    let fresh8 = 1;

    if (fresh7).xadd(fresh8, Ordering::SeqCst) + fresh8 == 1 {
        if RtsFlags.MiscFlags.tickInterval != 0 {
            stopTicker();
        }
    }
}

unsafe fn exitTimer(mut wait: bool) {
    if RtsFlags.MiscFlags.tickInterval != 0 {
        exitTicker(wait);
    }
}
