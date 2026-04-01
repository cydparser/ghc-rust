use crate::capability::{interruptCapability, recordClosureMutated, releaseCapability_};
use crate::clone_stack::handleCloneStackMessage;
use crate::ffi::rts::_assertFail;
use crate::ffi::rts::_assertFail;
use crate::ffi::rts::constants::{LDV_SHIFT, LDV_STATE_CREATE};
use crate::ffi::rts::constants::{LDV_SHIFT, LDV_STATE_CREATE, NotBlocked};
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::messages::barf;
use crate::ffi::rts::messages::{doneWithMsgThrowTo, whitehole_executeMessage_spin};
use crate::ffi::rts::non_moving::nonmoving_write_barrier_enabled;
use crate::ffi::rts::non_moving::nonmoving_write_barrier_enabled;
use crate::ffi::rts::prof::ccs::era;
use crate::ffi::rts::prof::ccs::{CCS_SYSTEM, CostCentreStack, era, user_era};
use crate::ffi::rts::storage::closure_macros::{
    UNTAG_CLOSURE, doingErasProfiling, doingLDVProfiling, doingRetainerProfiling,
    overwritingClosure,
};
use crate::ffi::rts::storage::closure_macros::{doingLDVProfiling, overwritingClosure};
use crate::ffi::rts::storage::closures::MessageThrowTo;
use crate::ffi::rts::storage::closures::{
    Message, Message_, MessageBlackHole, MessageBlackHole_, MessageCloneStack, MessageThrowTo,
    MessageWakeup, StgBlockingQueue, StgInd,
};
use crate::ffi::rts::storage::gc::allocate;
use crate::ffi::rts::storage::tso::dirty_TSO;
use crate::ffi::rts::threads::getNumCapabilities;
use crate::ffi::rts::threads::getNumCapabilities;
use crate::ffi::rts::types::StgClosure;
use crate::ffi::rts::types::{StgClosure, StgTSO};
use crate::ffi::rts_api::Capability;
use crate::ffi::rts_api::Capability;
use crate::ffi::stg::W_;
use crate::ffi::stg::misc_closures::{
    __stg_EAGER_BLACKHOLE_info, stg_BLACKHOLE_info, stg_BLOCKING_QUEUE_CLEAN_info,
    stg_BLOCKING_QUEUE_DIRTY_info, stg_CAF_BLACKHOLE_info, stg_END_TSO_QUEUE_closure, stg_IND_info,
    stg_MSG_BLACKHOLE_info, stg_MSG_CLONE_STACK_info, stg_MSG_NULL_info, stg_MSG_THROWTO_info,
    stg_MSG_TRY_WAKEUP_info, stg_TSO_info, stg_WHITEHOLE_info,
};
use crate::ffi::stg::misc_closures::{stg_MSG_NULL_info, stg_WHITEHOLE_info};
use crate::ffi::stg::smp::busy_wait_nop;
use crate::ffi::stg::types::{StgWord, StgWord32, StgWord64};
use crate::ffi::stg::types::{StgWord, StgWord64};
use crate::prelude::*;
use crate::raise_async::throwToMsg;
use crate::schedule::{SCHED_INTERRUPTING, getSchedState};
use crate::schedule::{SCHED_INTERRUPTING, getSchedState, promoteInRunQueue};
use crate::sm::non_moving_mark::updateRemembSetPushMessageThrowTo;
use crate::sm::non_moving_mark::{updateRemembSetPushClosure, updateRemembSetPushMessageThrowTo};
use crate::smp_closure_ops::unlockClosure;
use crate::smp_closure_ops::{lockClosure, unlockClosure};
use crate::task::myTask;
use crate::threads::tryWakeupThread;
use crate::trace::{DEBUG_RTS, traceCap_};

#[inline]
pub(crate) unsafe fn doneWithMsgThrowTo(mut cap: *mut Capability, mut m: *mut MessageThrowTo) {
    if (getNumCapabilities() == 1
        || (*m).header.info == &raw const stg_WHITEHOLE_info
        || getSchedState() as u32 == SCHED_INTERRUPTING as i32 as u32) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/Messages.h".as_ptr(), 32);
    }

    if nonmoving_write_barrier_enabled as i64 != 0 {
        updateRemembSetPushMessageThrowTo(cap, m);
    }

    overwritingClosure(m as *mut StgClosure);
    unlockClosure(m as *mut StgClosure, &raw const stg_MSG_NULL_info);

    if doingLDVProfiling() {
        (*(m as *mut StgClosure)).header.prof.hp.ldvw =
            (era as StgWord) << LDV_SHIFT | LDV_STATE_CREATE as StgWord;
    }
}

extern "C" {
    pub(crate) static mut whitehole_executeMessage_spin: StgWord64;
}

unsafe fn sendMessage(
    mut from_cap: *mut Capability,
    mut to_cap: *mut Capability,
    mut msg: *mut Message,
) {
    let mut __r = pthread_mutex_lock(&raw mut (*to_cap).lock);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Messages.c".as_ptr(),
            28,
            __r,
        );
    }

    let mut i = (*msg).header.info;

    if i != &raw const stg_MSG_THROWTO_info
        && i != &raw const stg_MSG_BLACKHOLE_info
        && i != &raw const stg_MSG_TRY_WAKEUP_info
        && i != &raw const stg_IND_info
        && i != &raw const stg_WHITEHOLE_info
        && i != &raw const stg_MSG_CLONE_STACK_info
    {
        barf(c"sendMessage: %p".as_ptr(), i);
    }

    (*msg).link = (*to_cap).inbox as *mut Message_;
    (&raw mut (*to_cap).inbox).store(msg, Ordering::Relaxed);
    recordClosureMutated(from_cap, msg as *mut StgClosure);

    if (*to_cap).running_task.is_null() {
        (*to_cap).running_task = myTask();
        releaseCapability_(to_cap, false);
    } else {
        interruptCapability(to_cap);
    }

    if pthread_mutex_unlock(&raw mut (*to_cap).lock) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Messages.c".as_ptr(),
            57,
        );
    }
}

unsafe fn executeMessage(mut cap: *mut Capability, mut m: *mut Message) {
    let mut i = null::<StgInfoTable>();

    loop {
        i = (&raw mut (*m).header.info).load(Ordering::Acquire);

        if i == &raw const stg_MSG_TRY_WAKEUP_info {
            let mut tso = (*(m as *mut MessageWakeup)).tso;

            if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
                traceCap_(cap, c"message: try wakeup thread %llu".as_ptr(), (*tso).id);
            }

            tryWakeupThread(cap, tso);
            break;
        } else if i == &raw const stg_MSG_THROWTO_info {
            let mut t = m as *mut MessageThrowTo;
            let mut r: u32 = 0;
            let mut i_0 = null::<StgInfoTable>();
            i_0 = lockClosure(m as *mut StgClosure);

            if i_0 != &raw const stg_MSG_THROWTO_info {
                unlockClosure(m as *mut StgClosure, i_0);
            } else {
                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
                    traceCap_(
                        cap,
                        c"message: throwTo %ld -> %ld".as_ptr(),
                        (*(*t).source).id,
                        (*(*t).target).id,
                    );
                }

                r = throwToMsg(cap, t);

                match r {
                    0 => {
                        let mut source = (*t).source;
                        doneWithMsgThrowTo(cap, t);
                        tryWakeupThread(cap, source);
                    }
                    1 => {
                        unlockClosure(m as *mut StgClosure, &raw const stg_MSG_THROWTO_info);
                    }
                    _ => {}
                }

                break;
            }
        } else if i == &raw const stg_MSG_BLACKHOLE_info {
            let mut r_0: u32 = 0;
            let mut b = m as *mut MessageBlackHole;
            r_0 = messageBlackHole(cap, b);

            if r_0 == 0 {
                tryWakeupThread(cap, (*b).tso);
            }

            return;
        } else if i == &raw const stg_IND_info || i == &raw const stg_MSG_NULL_info {
            return;
        } else if i == &raw const stg_WHITEHOLE_info {
            (&raw mut whitehole_executeMessage_spin).store(
                (&raw mut whitehole_executeMessage_spin)
                    .load(Ordering::Relaxed)
                    .wrapping_add(1 as StgWord64),
                Ordering::Relaxed,
            );
        } else {
            if i == &raw const stg_MSG_CLONE_STACK_info {
                let mut cloneStackMessage = m as *mut MessageCloneStack;
                handleCloneStackMessage(cloneStackMessage);
            } else {
                barf(c"executeMessage: %p".as_ptr(), i);
            }

            break;
        }
    }
}

unsafe fn messageBlackHole(mut cap: *mut Capability, mut msg: *mut MessageBlackHole) -> u32 {
    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
        traceCap_(
            cap,
            c"message: thread %llu blocking on blackhole %p".as_ptr(),
            (*(*msg).tso).id,
            (*msg).bh,
        );
    }

    let mut bh = UNTAG_CLOSURE((*msg).bh);
    let bh_info = (&raw mut (*bh).header.info).load(Ordering::Acquire);

    if bh_info != &raw const stg_BLACKHOLE_info
        && bh_info != &raw const stg_CAF_BLACKHOLE_info
        && bh_info != &raw const __stg_EAGER_BLACKHOLE_info
        && bh_info != &raw const stg_WHITEHOLE_info
    {
        return 0;
    }

    if bh_info == &raw const stg_WHITEHOLE_info {
        while (&raw mut (*bh).header.info).load(Ordering::Acquire) == &raw const stg_WHITEHOLE_info
        {
            busy_wait_nop();
        }
    }

    let mut p = null_mut::<StgClosure>();
    let mut info = null::<StgInfoTable>();

    loop {
        p = UNTAG_CLOSURE((&raw mut (*(bh as *mut StgInd)).indirectee).load(Ordering::Acquire));

        info = (&raw mut (*p).header.info).load(Ordering::Relaxed);

        if !(info == &raw const stg_IND_info) {
            break;
        }
    }

    if info == &raw const stg_TSO_info {
        let mut owner = p as *mut StgTSO;

        if (&raw mut (*owner).cap).load(Ordering::Relaxed) != cap {
            sendMessage(cap, (*owner).cap as *mut Capability, msg as *mut Message);

            if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
                traceCap_(
                    cap,
                    c"forwarding message to cap %d".as_ptr(),
                    (*(*owner).cap).no,
                );
            }

            return 1;
        }

        let mut bq = allocate(
            cap,
            (size_of::<StgBlockingQueue>() as usize)
                .wrapping_add(size_of::<W_>() as usize)
                .wrapping_sub(1 as usize)
                .wrapping_div(size_of::<W_>() as usize) as W_,
        ) as *mut StgBlockingQueue;

        (*bq).bh = bh;
        (*bq).queue = msg as *mut MessageBlackHole_;
        (*bq).owner = owner;
        (*msg).link = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
            as *mut MessageBlackHole as *mut MessageBlackHole_;
        (*bq).link = (*owner).bq;

        let ref mut fresh13 = (*(bq as *mut StgClosure)).header.prof.ccs;
        *fresh13 = &raw mut CCS_SYSTEM as *mut CostCentreStack;

        if doingLDVProfiling() {
            if doingLDVProfiling() {
                (*(bq as *mut StgClosure)).header.prof.hp.ldvw =
                    (era as StgWord) << LDV_SHIFT | LDV_STATE_CREATE as StgWord;
            }
        } else if doingRetainerProfiling() {
            (*(bq as *mut StgClosure)).header.prof.hp.trav = 0;
        } else if doingErasProfiling() {
            (*(bq as *mut StgClosure)).header.prof.hp.era = user_era;
        }

        (&raw mut (*bq).header.info)
            .store(&raw const stg_BLOCKING_QUEUE_DIRTY_info, Ordering::Relaxed);
        dirty_TSO(cap, owner);
        (&raw mut (*owner).bq).store(bq, Ordering::Release);

        if (*owner).why_blocked == NotBlocked as StgWord32 && (*owner).id != (*(*msg).tso).id {
            promoteInRunQueue(cap, owner);
        }

        (&raw mut (*(bh as *mut StgInd)).indirectee)
            .store(bq as *mut StgClosure, Ordering::Release);

        if nonmoving_write_barrier_enabled as i64 != 0 {
            updateRemembSetPushClosure(cap, p);
        }

        recordClosureMutated(cap, bh);

        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
            traceCap_(
                cap,
                c"thread %llu blocked on thread %llu".as_ptr(),
                (*(*msg).tso).id,
                (*owner).id,
            );
        }

        return 1;
    } else if info == &raw const stg_BLOCKING_QUEUE_CLEAN_info
        || info == &raw const stg_BLOCKING_QUEUE_DIRTY_info
    {
        let mut bq_0 = p as *mut StgBlockingQueue;

        if ((*bq_0).bh == bh) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/Messages.c".as_ptr(), 281);
        }

        let mut owner_0 = (*bq_0).owner;

        if (owner_0 != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO) as i32
            as i64
            != 0
        {
        } else {
            _assertFail(c"rts/Messages.c".as_ptr(), 285);
        }

        if (&raw mut (*owner_0).cap).load(Ordering::Relaxed) != cap {
            sendMessage(cap, (*owner_0).cap as *mut Capability, msg as *mut Message);

            if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
                traceCap_(
                    cap,
                    c"forwarding message to cap %d".as_ptr(),
                    (*(*owner_0).cap).no,
                );
            }

            return 1;
        }

        if nonmoving_write_barrier_enabled as i64 != 0 {
            updateRemembSetPushClosure(cap, (*bq_0).queue as *mut StgClosure);
        }

        (*msg).link = (*bq_0).queue as *mut MessageBlackHole_;
        (*bq_0).queue = msg as *mut MessageBlackHole_;
        recordClosureMutated(cap, msg as *mut StgClosure);

        if info == &raw const stg_BLOCKING_QUEUE_CLEAN_info {
            (&raw mut (*bq_0).header.info)
                .store(&raw const stg_BLOCKING_QUEUE_DIRTY_info, Ordering::Relaxed);
            recordClosureMutated(cap, bq_0 as *mut StgClosure);
        }

        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
            traceCap_(
                cap,
                c"thread %llu blocked on existing BLOCKING_QUEUE owned by thread %llu".as_ptr(),
                (*(*msg).tso).id,
                (*owner_0).id,
            );
        }

        if (*owner_0).why_blocked == NotBlocked as StgWord32 && (*owner_0).id != (*(*msg).tso).id {
            promoteInRunQueue(cap, owner_0);
        }

        return 1;
    }

    return 0;
}

unsafe fn blackHoleOwner(mut bh: *mut StgClosure) -> *mut StgTSO {
    let mut info = null::<StgInfoTable>();
    let mut p = null_mut::<StgClosure>();
    info = (&raw mut (*bh).header.info).load(Ordering::Relaxed);

    if info != &raw const stg_BLACKHOLE_info
        && info != &raw const stg_CAF_BLACKHOLE_info
        && info != &raw const __stg_EAGER_BLACKHOLE_info
        && info != &raw const stg_WHITEHOLE_info
    {
        return null_mut::<StgTSO>();
    }

    loop {
        p = UNTAG_CLOSURE((&raw mut (*(bh as *mut StgInd)).indirectee).load(Ordering::Acquire));

        info = (&raw mut (*p).header.info).load(Ordering::Relaxed);

        if !(info == &raw const stg_IND_info) {
            break;
        }
    }

    if info == &raw const stg_TSO_info {
        return p as *mut StgTSO;
    } else if info == &raw const stg_BLOCKING_QUEUE_CLEAN_info
        || info == &raw const stg_BLOCKING_QUEUE_DIRTY_info
    {
        let mut bq = p as *mut StgBlockingQueue;

        return (&raw mut (*bq).owner).load(Ordering::Relaxed);
    }

    return null_mut::<StgTSO>();
}
