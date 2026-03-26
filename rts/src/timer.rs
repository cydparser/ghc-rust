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

static mut ticks_to_ctxt_switch: c_int = 0 as c_int;

static mut ticks_to_eventlog_flush: c_int = 0 as c_int;

static mut idle_ticks_to_gc: c_int = 0 as c_int;

static mut inter_gc_ticks_to_gc: c_int = 0 as c_int;

unsafe fn handle_tick(mut unused: c_int) {
    handleProfTick();

    if RtsFlags.ConcFlags.ctxtSwitchTicks > 0 as c_int
        && ::core::intrinsics::atomic_load_seqcst(&raw mut timer_disabled) == 0 as StgWord
    {
        ticks_to_ctxt_switch -= 1;

        if ticks_to_ctxt_switch <= 0 as c_int {
            ticks_to_ctxt_switch = RtsFlags.ConcFlags.ctxtSwitchTicks;
            contextSwitchAllCapabilities();
        }
    }

    if eventLogStatus() as c_uint == EVENTLOG_RUNNING as c_int as c_uint
        && RtsFlags.TraceFlags.eventlogFlushTicks > 0 as c_int
    {
        ticks_to_eventlog_flush -= 1;

        if ticks_to_eventlog_flush <= 0 as c_int {
            ticks_to_eventlog_flush = RtsFlags.TraceFlags.eventlogFlushTicks;
            flushEventLog(null_mut::<*mut Capability>());
        }
    }

    match getRecentActivity() as c_uint {
        0 => {
            setRecentActivity(ACTIVITY_MAYBE_NO);
            idle_ticks_to_gc =
                (RtsFlags.GcFlags.idleGCDelayTime / RtsFlags.MiscFlags.tickInterval) as c_int;
        }
        1 => {
            if idle_ticks_to_gc == 0 as c_int && inter_gc_ticks_to_gc == 0 as c_int {
                if RtsFlags.GcFlags.doIdleGC {
                    setRecentActivity(ACTIVITY_INACTIVE);
                    inter_gc_ticks_to_gc = (RtsFlags.GcFlags.interIdleGCWait
                        / RtsFlags.MiscFlags.tickInterval)
                        as c_int;
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

    if RtsFlags.MiscFlags.tickInterval != 0 as Time {
        initTicker(
            RtsFlags.MiscFlags.tickInterval,
            Some(handle_tick as unsafe extern "C" fn(c_int) -> ()),
        );
    }

    ::core::intrinsics::atomic_store_seqcst(&raw mut timer_disabled, 1 as c_int as StgWord);
}

#[ffi(libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn startTimer() {
    let fresh5 = &raw mut timer_disabled;
    let fresh6 = 1 as c_int as StgWord;

    if ::core::intrinsics::atomic_xsub_seqcst(fresh5, fresh6) - fresh6 == 0 as StgWord {
        if RtsFlags.MiscFlags.tickInterval != 0 as Time {
            startTicker();
        }
    }
}

#[ffi(libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn stopTimer() {
    let fresh7 = &raw mut timer_disabled;
    let fresh8 = 1 as c_int as StgWord;

    if ::core::intrinsics::atomic_xadd_seqcst(fresh7, fresh8) + fresh8 == 1 as StgWord {
        if RtsFlags.MiscFlags.tickInterval != 0 as Time {
            stopTicker();
        }
    }
}

unsafe fn exitTimer(mut wait: bool) {
    if RtsFlags.MiscFlags.tickInterval != 0 as Time {
        exitTicker(wait);
    }
}
