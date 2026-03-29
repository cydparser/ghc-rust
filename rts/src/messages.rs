use crate::capability::recordClosureMutated;
use crate::ffi::rts::constants::NotBlocked;
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::storage::closure_macros::UNTAG_CLOSURE;
use crate::ffi::rts::storage::closures::MessageThrowTo;
use crate::ffi::rts::storage::closures::{
    MessageBlackHole, MessageBlackHole_, MessageThrowTo, StgBlockingQueue, StgBlockingQueue_,
    StgInd,
};
use crate::ffi::rts::storage::gc::allocate;
use crate::ffi::rts::storage::tso::dirty_TSO;
use crate::ffi::rts::types::StgClosure;
use crate::ffi::rts::types::{StgClosure, StgTSO};
use crate::ffi::rts_api::Capability;
use crate::ffi::rts_api::Capability;
use crate::ffi::stg::W_;
use crate::ffi::stg::misc_closures::stg_MSG_NULL_info;
use crate::ffi::stg::misc_closures::{
    __stg_EAGER_BLACKHOLE_info, stg_BLACKHOLE_info, stg_BLOCKING_QUEUE_CLEAN_info,
    stg_BLOCKING_QUEUE_DIRTY_info, stg_CAF_BLACKHOLE_info, stg_END_TSO_QUEUE_closure, stg_IND_info,
    stg_MSG_NULL_info, stg_TSO_info, stg_WHITEHOLE_info,
};
use crate::ffi::stg::types::StgWord32;
use crate::prelude::*;
use crate::schedule::promoteInRunQueue;
use crate::smp_closure_ops::unlockClosure;
use crate::smp_closure_ops::unlockClosure;
use crate::trace::{DEBUG_RTS, traceCap_};

#[inline]
pub(crate) unsafe fn doneWithMsgThrowTo(mut cap: *mut Capability, mut m: *mut MessageThrowTo) {
    unlockClosure(m as *mut StgClosure, &raw const stg_MSG_NULL_info);
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
    let bh_info = (*bh).header.info;

    if bh_info != &raw const stg_BLACKHOLE_info
        && bh_info != &raw const stg_CAF_BLACKHOLE_info
        && bh_info != &raw const __stg_EAGER_BLACKHOLE_info
        && bh_info != &raw const stg_WHITEHOLE_info
    {
        return 0;
    }

    let mut p = null_mut::<StgClosure>();
    let mut info = null::<StgInfoTable>();

    loop {
        p = UNTAG_CLOSURE((*(bh as *mut StgInd)).indirectee);
        info = (*p).header.info;

        if !(info == &raw const stg_IND_info) {
            break;
        }
    }

    if info == &raw const stg_TSO_info {
        let mut owner = p as *mut StgTSO;

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
        (*bq).header.info = &raw const stg_BLOCKING_QUEUE_DIRTY_info;
        dirty_TSO(cap, owner);
        (*owner).bq = bq as *mut StgBlockingQueue_;

        if (*owner).why_blocked == NotBlocked as StgWord32 && (*owner).id != (*(*msg).tso).id {
            promoteInRunQueue(cap, owner);
        }

        let ref mut fresh7 = (*(bh as *mut StgInd)).indirectee;
        *fresh7 = bq as *mut StgClosure;
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
        let mut owner_0 = (*bq_0).owner;
        (*msg).link = (*bq_0).queue as *mut MessageBlackHole_;
        (*bq_0).queue = msg as *mut MessageBlackHole_;
        recordClosureMutated(cap, msg as *mut StgClosure);

        if info == &raw const stg_BLOCKING_QUEUE_CLEAN_info {
            (*bq_0).header.info = &raw const stg_BLOCKING_QUEUE_DIRTY_info;
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
    info = (*bh).header.info;

    if info != &raw const stg_BLACKHOLE_info
        && info != &raw const stg_CAF_BLACKHOLE_info
        && info != &raw const __stg_EAGER_BLACKHOLE_info
        && info != &raw const stg_WHITEHOLE_info
    {
        return null_mut::<StgTSO>();
    }

    loop {
        p = UNTAG_CLOSURE((*(bh as *mut StgInd)).indirectee);
        info = (*p).header.info;

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

        return (*bq).owner;
    }

    return null_mut::<StgTSO>();
}
