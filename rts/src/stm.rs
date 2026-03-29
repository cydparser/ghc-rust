use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::storage::closure_macros::{GET_INFO, UNTAG_CLOSURE};
use crate::ffi::rts::storage::closures::{
    StgTRecChunk, StgTRecChunk_, StgTRecHeader, StgTRecHeader_, StgTVar, StgTVarWatchQueue,
    StgTVarWatchQueue_, TREC_ABORTED, TREC_ACTIVE, TREC_CHUNK_NUM_ENTRIES, TREC_CONDEMNED,
    TREC_WAITING, TRecEntry,
};
use crate::ffi::rts::storage::gc::allocate;
use crate::ffi::rts::threads::getNumCapabilities;
use crate::ffi::rts::types::{StgClosure, StgTSO};
use crate::ffi::rts_api::Capability;
use crate::ffi::stg::W_;
use crate::ffi::stg::misc_closures::{
    stg_END_STM_CHUNK_LIST_closure, stg_END_STM_WATCH_QUEUE_closure, stg_END_TSO_QUEUE_closure,
    stg_NO_TREC_closure, stg_TREC_CHUNK_info, stg_TREC_HEADER_info, stg_TVAR_WATCH_QUEUE_info,
};
use crate::ffi::stg::types::{StgBool, StgInt64, StgWord, StgWord32};
use crate::prelude::*;
use crate::sm::storage::dirty_TVAR;
use crate::threads::tryWakeupThread;
use crate::trace::{DEBUG_RTS, trace_};

unsafe fn shake() -> i32 {
    return false;
}

static mut config_use_read_phase: StgBool = false;

unsafe fn lock_tvar(
    mut cap: *mut Capability,
    mut trec: *mut StgTRecHeader,
    mut s: *mut StgTVar,
) -> *mut StgClosure {
    let mut result = null_mut::<StgClosure>();

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(c"STM: %p : lock_tvar(%p)".as_ptr(), trec, s);
    }

    result = (*s).current_value;

    return result;
}

unsafe fn unlock_tvar(
    mut cap: *mut Capability,
    mut trec: *mut StgTRecHeader,
    mut s: *mut StgTVar,
    mut c: *mut StgClosure,
    mut force_update: StgBool,
) {
    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(c"STM: %p : unlock_tvar(%p)".as_ptr(), trec, s);
    }

    if force_update != 0 {
        let mut old_value = (*s).current_value;
        (*s).current_value = c;
        dirty_TVAR(cap, s, old_value);
    }
}

unsafe fn cond_lock_tvar(
    mut cap: *mut Capability,
    mut trec: *mut StgTRecHeader,
    mut s: *mut StgTVar,
    mut expected: *mut StgClosure,
) -> StgBool {
    let mut result = null_mut::<StgClosure>();
    result = (*s).current_value;

    return (result == expected) as i32;
}

unsafe fn park_tso(mut tso: *mut StgTSO) {
    (*tso).block_info.closure =
        &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO as *mut StgClosure;
    (*tso).why_blocked = 6;

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(c"STM: park_tso on tso=%p".as_ptr(), tso);
    }
}

unsafe fn unpark_tso(mut cap: *mut Capability, mut tso: *mut StgTSO) {
    tryWakeupThread(cap, tso);
}

unsafe fn unpark_waiters_on(mut cap: *mut Capability, mut s: *mut StgTVar) {
    let mut q = null_mut::<StgTVarWatchQueue>();
    let mut trail = null_mut::<StgTVarWatchQueue>();

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(c"STM: unpark_waiters_on tvar=%p".as_ptr(), s);
    }

    q = (*s).first_watch_queue_entry;
    trail = q;

    while q != &raw mut stg_END_STM_WATCH_QUEUE_closure as *mut c_void as *mut StgTVarWatchQueue {
        trail = q;
        q = (*q).next_queue_entry as *mut StgTVarWatchQueue;
    }

    q = trail;

    while q != &raw mut stg_END_STM_WATCH_QUEUE_closure as *mut c_void as *mut StgTVarWatchQueue {
        unpark_tso(cap, (*q).closure as *mut StgTSO);
        q = (*q).prev_queue_entry as *mut StgTVarWatchQueue;
    }
}

unsafe fn new_stg_tvar_watch_queue(
    mut cap: *mut Capability,
    mut closure: *mut StgClosure,
) -> *mut StgTVarWatchQueue {
    let mut result = null_mut::<StgTVarWatchQueue>();

    result = allocate(
        cap,
        (size_of::<StgTVarWatchQueue>() as usize)
            .wrapping_add(size_of::<W_>() as usize)
            .wrapping_sub(1 as usize)
            .wrapping_div(size_of::<W_>() as usize) as W_,
    ) as *mut StgTVarWatchQueue;

    (*result).header.info = &raw const stg_TVAR_WATCH_QUEUE_info;
    (*result).closure = closure;

    return result;
}

unsafe fn new_stg_trec_chunk(mut cap: *mut Capability) -> *mut StgTRecChunk {
    let mut result = null_mut::<StgTRecChunk>();

    result = allocate(
        cap,
        (size_of::<StgTRecChunk>() as usize)
            .wrapping_add(size_of::<W_>() as usize)
            .wrapping_sub(1 as usize)
            .wrapping_div(size_of::<W_>() as usize) as W_,
    ) as *mut StgTRecChunk;

    (*result).header.info = &raw const stg_TREC_CHUNK_info;
    (*result).prev_chunk = &raw mut stg_END_STM_CHUNK_LIST_closure as *mut c_void
        as *mut StgTRecChunk as *mut StgTRecChunk_;
    (*result).next_entry_idx = 0;

    return result;
}

unsafe fn new_stg_trec_header(
    mut cap: *mut Capability,
    mut enclosing_trec: *mut StgTRecHeader,
) -> *mut StgTRecHeader {
    let mut result = null_mut::<StgTRecHeader>();

    result = allocate(
        cap,
        (size_of::<StgTRecHeader>() as usize)
            .wrapping_add(size_of::<W_>() as usize)
            .wrapping_sub(1 as usize)
            .wrapping_div(size_of::<W_>() as usize) as W_,
    ) as *mut StgTRecHeader;

    (*result).header.info = &raw const stg_TREC_HEADER_info;
    (*result).enclosing_trec = enclosing_trec as *mut StgTRecHeader_;
    (*result).current_chunk = new_stg_trec_chunk(cap);

    if enclosing_trec == &raw mut stg_NO_TREC_closure as *mut c_void as *mut StgTRecHeader {
        (*result).state = TREC_ACTIVE;
    } else {
        (*result).state = (*enclosing_trec).state;
    }

    return result;
}

unsafe fn alloc_stg_tvar_watch_queue(
    mut cap: *mut Capability,
    mut closure: *mut StgClosure,
) -> *mut StgTVarWatchQueue {
    let mut result = null_mut::<StgTVarWatchQueue>();

    if (*cap).free_tvar_watch_queues
        == &raw mut stg_END_STM_WATCH_QUEUE_closure as *mut c_void as *mut StgTVarWatchQueue
    {
        result = new_stg_tvar_watch_queue(cap, closure);
    } else {
        result = (*cap).free_tvar_watch_queues;
        (*result).closure = closure;
        (*cap).free_tvar_watch_queues = (*result).next_queue_entry as *mut StgTVarWatchQueue;
    }

    return result;
}

unsafe fn free_stg_tvar_watch_queue(mut cap: *mut Capability, mut wq: *mut StgTVarWatchQueue) {
    (*wq).next_queue_entry = (*cap).free_tvar_watch_queues as *mut StgTVarWatchQueue_;
    (*cap).free_tvar_watch_queues = wq;
}

unsafe fn alloc_stg_trec_chunk(mut cap: *mut Capability) -> *mut StgTRecChunk {
    let mut result = null_mut::<StgTRecChunk>();

    if (*cap).free_trec_chunks
        == &raw mut stg_END_STM_CHUNK_LIST_closure as *mut c_void as *mut StgTRecChunk
    {
        result = new_stg_trec_chunk(cap);
    } else {
        result = (*cap).free_trec_chunks;
        (*cap).free_trec_chunks = (*result).prev_chunk as *mut StgTRecChunk;
        (*result).prev_chunk = &raw mut stg_END_STM_CHUNK_LIST_closure as *mut c_void
            as *mut StgTRecChunk as *mut StgTRecChunk_;
        (*result).next_entry_idx = 0;
    }

    return result;
}

unsafe fn free_stg_trec_chunk(mut cap: *mut Capability, mut c: *mut StgTRecChunk) {
    (*c).prev_chunk = (*cap).free_trec_chunks as *mut StgTRecChunk_;
    (*cap).free_trec_chunks = c;
}

unsafe fn alloc_stg_trec_header(
    mut cap: *mut Capability,
    mut enclosing_trec: *mut StgTRecHeader,
) -> *mut StgTRecHeader {
    let mut result = null_mut::<StgTRecHeader>();

    if (*cap).free_trec_headers == &raw mut stg_NO_TREC_closure as *mut c_void as *mut StgTRecHeader
    {
        result = new_stg_trec_header(cap, enclosing_trec);
    } else {
        result = (*cap).free_trec_headers;
        (*cap).free_trec_headers = (*result).enclosing_trec as *mut StgTRecHeader;
        (*result).enclosing_trec = enclosing_trec as *mut StgTRecHeader_;
        (*(*result).current_chunk).next_entry_idx = 0;

        if enclosing_trec == &raw mut stg_NO_TREC_closure as *mut c_void as *mut StgTRecHeader {
            (*result).state = TREC_ACTIVE;
        } else {
            (*result).state = (*enclosing_trec).state;
        }
    }

    return result;
}

unsafe fn free_stg_trec_header(mut cap: *mut Capability, mut trec: *mut StgTRecHeader) {
    let mut chunk = (*(*trec).current_chunk).prev_chunk as *mut StgTRecChunk;

    while chunk != &raw mut stg_END_STM_CHUNK_LIST_closure as *mut c_void as *mut StgTRecChunk {
        let mut prev_chunk = (*chunk).prev_chunk as *mut StgTRecChunk;
        free_stg_trec_chunk(cap, chunk);
        chunk = prev_chunk;
    }

    (*(*trec).current_chunk).prev_chunk = &raw mut stg_END_STM_CHUNK_LIST_closure as *mut c_void
        as *mut StgTRecChunk as *mut StgTRecChunk_;
    (*trec).enclosing_trec = (*cap).free_trec_headers as *mut StgTRecHeader_;
    (*cap).free_trec_headers = trec;
}

unsafe fn build_watch_queue_entries_for_trec(
    mut cap: *mut Capability,
    mut tso: *mut StgTSO,
    mut trec: *mut StgTRecHeader,
) {
    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(
            c"STM: %p : build_watch_queue_entries_for_trec()".as_ptr(),
            trec,
        );
    }

    let mut __t = trec;
    let mut __c = (*__t).current_chunk;
    let mut __limit: StgWord = (*__c).next_entry_idx;

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(
            c"STM: %p : FOR_EACH_ENTRY, current_chunk=%p limit=%ld".as_ptr(),
            __t,
            __c,
            __limit,
        );
    }

    while __c != &raw mut stg_END_STM_CHUNK_LIST_closure as *mut c_void as *mut StgTRecChunk {
        let mut __i: StgWord = 0;
        __i = 0;

        while __i < __limit {
            let mut e: *mut TRecEntry =
                (&raw mut (*__c).entries as *mut TRecEntry).offset(__i as isize) as *mut TRecEntry;

            let mut s = null_mut::<StgTVar>();
            let mut q = null_mut::<StgTVarWatchQueue>();
            let mut fq = null_mut::<StgTVarWatchQueue>();
            s = (*e).tvar;

            if 0 != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
                trace_(
                    c"STM: %p : adding tso=%p to watch queue for tvar=%p".as_ptr(),
                    trec,
                    tso,
                    s,
                );
            }

            fq = (*s).first_watch_queue_entry;
            q = alloc_stg_tvar_watch_queue(cap, tso as *mut StgClosure);
            (*q).next_queue_entry = fq as *mut StgTVarWatchQueue_;
            (*q).prev_queue_entry = &raw mut stg_END_STM_WATCH_QUEUE_closure as *mut c_void
                as *mut StgTVarWatchQueue
                as *mut StgTVarWatchQueue_;

            if fq
                != &raw mut stg_END_STM_WATCH_QUEUE_closure as *mut c_void as *mut StgTVarWatchQueue
            {
                (*fq).prev_queue_entry = q as *mut StgTVarWatchQueue_;
            }

            (*s).first_watch_queue_entry = q;
            (*e).new_value = q as *mut StgClosure;
            dirty_TVAR(cap, s, fq as *mut StgClosure);
            __i = __i.wrapping_add(1);
        }

        __c = (*__c).prev_chunk as *mut StgTRecChunk;
        __limit = TREC_CHUNK_NUM_ENTRIES as StgWord;
    }
}

unsafe fn remove_watch_queue_entries_for_trec(
    mut cap: *mut Capability,
    mut trec: *mut StgTRecHeader,
) {
    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(
            c"STM: %p : remove_watch_queue_entries_for_trec()".as_ptr(),
            trec,
        );
    }

    let mut __t = trec;
    let mut __c = (*__t).current_chunk;
    let mut __limit: StgWord = (*__c).next_entry_idx;

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(
            c"STM: %p : FOR_EACH_ENTRY, current_chunk=%p limit=%ld".as_ptr(),
            __t,
            __c,
            __limit,
        );
    }

    while __c != &raw mut stg_END_STM_CHUNK_LIST_closure as *mut c_void as *mut StgTRecChunk {
        let mut __i: StgWord = 0;
        __i = 0;

        while __i < __limit {
            let mut e: *mut TRecEntry =
                (&raw mut (*__c).entries as *mut TRecEntry).offset(__i as isize) as *mut TRecEntry;

            let mut s = null_mut::<StgTVar>();
            let mut pq = null_mut::<StgTVarWatchQueue>();
            let mut nq = null_mut::<StgTVarWatchQueue>();
            let mut q = null_mut::<StgTVarWatchQueue>();
            let mut saw = null_mut::<StgClosure>();
            s = (*e).tvar;
            saw = lock_tvar(cap, trec, s);
            q = (*e).new_value as *mut StgTVarWatchQueue;

            if 0 != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
                trace_(
                    c"STM: %p : removing tso=%p from watch queue for tvar=%p".as_ptr(),
                    trec,
                    (*q).closure,
                    s,
                );
            }

            nq = (*q).next_queue_entry as *mut StgTVarWatchQueue;
            pq = (*q).prev_queue_entry as *mut StgTVarWatchQueue;

            if nq
                != &raw mut stg_END_STM_WATCH_QUEUE_closure as *mut c_void as *mut StgTVarWatchQueue
            {
                (*nq).prev_queue_entry = pq as *mut StgTVarWatchQueue_;
            }

            if pq
                != &raw mut stg_END_STM_WATCH_QUEUE_closure as *mut c_void as *mut StgTVarWatchQueue
            {
                (*pq).next_queue_entry = nq as *mut StgTVarWatchQueue_;
            } else {
                (*s).first_watch_queue_entry = nq;
                dirty_TVAR(cap, s, q as *mut StgClosure);
            }

            free_stg_tvar_watch_queue(cap, q);
            unlock_tvar(cap, trec, s, saw, 0);
            __i = __i.wrapping_add(1);
        }

        __c = (*__c).prev_chunk as *mut StgTRecChunk;
        __limit = TREC_CHUNK_NUM_ENTRIES as StgWord;
    }
}

unsafe fn get_new_entry(mut cap: *mut Capability, mut t: *mut StgTRecHeader) -> *mut TRecEntry {
    let mut result = null_mut::<TRecEntry>();
    let mut c = null_mut::<StgTRecChunk>();
    let mut i: i32 = 0;
    c = (*t).current_chunk;
    i = (*c).next_entry_idx as i32;

    if i < TREC_CHUNK_NUM_ENTRIES {
        result = (&raw mut (*c).entries as *mut TRecEntry).offset(i as isize) as *mut TRecEntry;
        (*c).next_entry_idx = (*c).next_entry_idx.wrapping_add(1);
    } else {
        let mut nc = null_mut::<StgTRecChunk>();
        nc = alloc_stg_trec_chunk(cap);
        (*nc).prev_chunk = c as *mut StgTRecChunk_;
        (*nc).next_entry_idx = 1;
        (*t).current_chunk = nc;
        result = (&raw mut (*nc).entries as *mut TRecEntry).offset(0) as *mut TRecEntry;
    }

    return result;
}

unsafe fn merge_update_into(
    mut cap: *mut Capability,
    mut t: *mut StgTRecHeader,
    mut tvar: *mut StgTVar,
    mut expected_value: *mut StgClosure,
    mut new_value: *mut StgClosure,
) {
    let mut found = false;
    let mut __t = t;
    let mut __c = (*__t).current_chunk;
    let mut __limit: StgWord = (*__c).next_entry_idx;

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(
            c"STM: %p : FOR_EACH_ENTRY, current_chunk=%p limit=%ld".as_ptr(),
            __t,
            __c,
            __limit,
        );
    }

    's_28: while __c != &raw mut stg_END_STM_CHUNK_LIST_closure as *mut c_void as *mut StgTRecChunk
    {
        let mut __i: StgWord = 0;
        __i = 0;

        while __i < __limit {
            let mut e: *mut TRecEntry =
                (&raw mut (*__c).entries as *mut TRecEntry).offset(__i as isize) as *mut TRecEntry;

            let mut s = null_mut::<StgTVar>();
            s = (*e).tvar;

            if s == tvar {
                found = 1 != 0;

                if (*e).expected_value != expected_value {
                    if 0 != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
                        trace_(
                            c"STM: %p : update entries inconsistent at %p (%p vs %p)".as_ptr(),
                            t,
                            tvar,
                            (*e).expected_value,
                            expected_value,
                        );
                    }

                    (*t).state = TREC_CONDEMNED;
                }

                (*e).new_value = new_value;
                break 's_28;
            } else {
                __i = __i.wrapping_add(1);
            }
        }

        __c = (*__c).prev_chunk as *mut StgTRecChunk;
        __limit = TREC_CHUNK_NUM_ENTRIES as StgWord;
    }

    if !found {
        let mut ne = null_mut::<TRecEntry>();
        ne = get_new_entry(cap, t);
        (*ne).tvar = tvar;
        (*ne).expected_value = expected_value;
        (*ne).new_value = new_value;
    }
}

unsafe fn merge_read_into(
    mut cap: *mut Capability,
    mut trec: *mut StgTRecHeader,
    mut tvar: *mut StgTVar,
    mut expected_value: *mut StgClosure,
) {
    let mut t = null_mut::<StgTRecHeader>();
    let mut found = false;
    t = trec;

    while !found && t != &raw mut stg_NO_TREC_closure as *mut c_void as *mut StgTRecHeader {
        let mut __t = t;
        let mut __c = (*__t).current_chunk;
        let mut __limit: StgWord = (*__c).next_entry_idx;

        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
            trace_(
                c"STM: %p : FOR_EACH_ENTRY, current_chunk=%p limit=%ld".as_ptr(),
                __t,
                __c,
                __limit,
            );
        }

        's_39: while __c
            != &raw mut stg_END_STM_CHUNK_LIST_closure as *mut c_void as *mut StgTRecChunk
        {
            let mut __i: StgWord = 0;
            __i = 0;

            while __i < __limit {
                let mut e: *mut TRecEntry = (&raw mut (*__c).entries as *mut TRecEntry)
                    .offset(__i as isize)
                    as *mut TRecEntry;

                if (*e).tvar == tvar {
                    found = 1 != 0;

                    if (*e).expected_value != expected_value {
                        if 0 != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
                            trace_(
                                c"STM: %p : read entries inconsistent at %p (%p vs %p)".as_ptr(),
                                t,
                                tvar,
                                (*e).expected_value,
                                expected_value,
                            );
                        }

                        (*t).state = TREC_CONDEMNED;
                    }

                    break 's_39;
                } else {
                    __i = __i.wrapping_add(1);
                }
            }

            __c = (*__c).prev_chunk as *mut StgTRecChunk;
            __limit = TREC_CHUNK_NUM_ENTRIES as StgWord;
        }

        t = (*t).enclosing_trec as *mut StgTRecHeader;
    }

    if !found {
        let mut ne = null_mut::<TRecEntry>();
        ne = get_new_entry(cap, trec);
        (*ne).tvar = tvar;
        (*ne).expected_value = expected_value;
        (*ne).new_value = expected_value;
    }
}

unsafe fn entry_is_update(mut e: *mut TRecEntry) -> StgBool {
    let mut result: StgBool = 0;
    result = ((*e).expected_value != (*e).new_value) as i32 as StgBool;

    return result;
}

unsafe fn revert_ownership(
    mut cap: *mut Capability,
    mut trec: *mut StgTRecHeader,
    mut revert_all: StgBool,
) {
}

unsafe fn validate_trec_optimistic(
    mut cap: *mut Capability,
    mut trec: *mut StgTRecHeader,
) -> StgBool {
    let mut result: StgBool = 0;

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(
            c"STM: cap %d, trec %p : validate_trec_optimistic".as_ptr(),
            (*cap).no,
            trec,
        );
    }

    if shake() != 0 {
        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
            trace_(
                c"STM: %p : shake, pretending trec is invalid when it may not be".as_ptr(),
                trec,
            );
        }

        return false;
    }

    result = !((*trec).state as u32 == TREC_CONDEMNED as i32 as u32) as i32 as StgBool;

    if result != 0 {
        let mut __t = trec;
        let mut __c = (*__t).current_chunk;
        let mut __limit: StgWord = (*__c).next_entry_idx;

        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
            trace_(
                c"STM: %p : FOR_EACH_ENTRY, current_chunk=%p limit=%ld".as_ptr(),
                __t,
                __c,
                __limit,
            );
        }

        's_87: while __c
            != &raw mut stg_END_STM_CHUNK_LIST_closure as *mut c_void as *mut StgTRecChunk
        {
            let mut __i: StgWord = 0;
            __i = 0;

            while __i < __limit {
                let mut e: *mut TRecEntry = (&raw mut (*__c).entries as *mut TRecEntry)
                    .offset(__i as isize)
                    as *mut TRecEntry;

                let mut s = null_mut::<StgTVar>();
                s = (*e).tvar;

                let mut current = (*s).current_value;

                if current != (*e).expected_value
                    && GET_INFO(UNTAG_CLOSURE(current)) != &raw const stg_TREC_HEADER_info
                {
                    if 0 != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
                        trace_(c"STM: %p : failed optimistic validate %p".as_ptr(), trec, s);
                    }

                    result = 0;
                    break 's_87;
                } else {
                    __i = __i.wrapping_add(1);
                }
            }

            __c = (*__c).prev_chunk as *mut StgTRecChunk;
            __limit = TREC_CHUNK_NUM_ENTRIES as StgWord;
        }
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(
            c"STM: %p : validate_trec_optimistic, result: %d".as_ptr(),
            trec,
            result,
        );
    }

    return result;
}

unsafe fn validate_and_acquire_ownership(
    mut cap: *mut Capability,
    mut trec: *mut StgTRecHeader,
    mut acquire_all: i32,
    mut retain_ownership: i32,
) -> StgBool {
    let mut result: StgBool = 0;

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(
            c"STM: cap %d, trec %p : validate_and_acquire_ownership, all: %d, retrain: %d".as_ptr(),
            (*cap).no,
            trec,
            acquire_all,
            retain_ownership,
        );
    }

    if shake() != 0 {
        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
            trace_(
                c"STM: %p : shake, pretending trec is invalid when it may not be".as_ptr(),
                trec,
            );
        }

        return false;
    }

    result = !((*trec).state as u32 == TREC_CONDEMNED as i32 as u32) as i32 as StgBool;

    if result != 0 {
        let mut __t = trec;
        let mut __c = (*__t).current_chunk;
        let mut __limit: StgWord = (*__c).next_entry_idx;

        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
            trace_(
                c"STM: %p : FOR_EACH_ENTRY, current_chunk=%p limit=%ld".as_ptr(),
                __t,
                __c,
                __limit,
            );
        }

        's_87: while __c
            != &raw mut stg_END_STM_CHUNK_LIST_closure as *mut c_void as *mut StgTRecChunk
        {
            let mut __i: StgWord = 0;
            __i = 0;

            while __i < __limit {
                let mut e: *mut TRecEntry = (&raw mut (*__c).entries as *mut TRecEntry)
                    .offset(__i as isize)
                    as *mut TRecEntry;

                let mut s = null_mut::<StgTVar>();
                s = (*e).tvar;

                if acquire_all != 0 || entry_is_update(e) != 0 {
                    if 0 != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
                        trace_(c"STM: %p : trying to acquire %p".as_ptr(), trec, s);
                    }

                    if cond_lock_tvar(cap, trec, s, (*e).expected_value) == 0 {
                        if 0 != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
                            trace_(c"STM: %p : failed to acquire %p".as_ptr(), trec, s);
                        }

                        result = 0;
                        break 's_87;
                    }
                }

                __i = __i.wrapping_add(1);
            }

            __c = (*__c).prev_chunk as *mut StgTRecChunk;
            __limit = TREC_CHUNK_NUM_ENTRIES as StgWord;
        }
    }

    if result == 0 || retain_ownership == 0 {
        revert_ownership(cap, trec, acquire_all as StgBool);
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(
            c"STM: %p : validate_and_acquire_ownership, result: %d".as_ptr(),
            trec,
            result,
        );
    }

    return result;
}

unsafe fn check_read_only(mut trec: *mut StgTRecHeader) -> StgBool {
    let mut result = true;

    return result;
}

unsafe fn stmPreGCHook(mut cap: *mut Capability) {
    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(c"STM: stmPreGCHook".as_ptr());
    }

    (*cap).free_tvar_watch_queues =
        &raw mut stg_END_STM_WATCH_QUEUE_closure as *mut c_void as *mut StgTVarWatchQueue;
    (*cap).free_trec_chunks =
        &raw mut stg_END_STM_CHUNK_LIST_closure as *mut c_void as *mut StgTRecChunk;
    (*cap).free_trec_headers = &raw mut stg_NO_TREC_closure as *mut c_void as *mut StgTRecHeader;
}

const TOKEN_BATCH_SIZE: i32 = 1024;

unsafe fn getMaxCommits() -> StgInt64 {
    return 0;
}

unsafe fn getToken(mut cap: *mut Capability) {}

unsafe fn stmStartTransaction(
    mut cap: *mut Capability,
    mut outer: *mut StgTRecHeader,
) -> *mut StgTRecHeader {
    let mut t = null_mut::<StgTRecHeader>();

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(
            c"STM: %p : stmStartTransaction with %d tokens".as_ptr(),
            outer,
            (*cap).transaction_tokens,
        );
    }

    getToken(cap);
    t = alloc_stg_trec_header(cap, outer);

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(c"STM: %p : stmStartTransaction()=%p".as_ptr(), outer, t);
    }

    return t;
}

unsafe fn stmAbortTransaction(mut cap: *mut Capability, mut trec: *mut StgTRecHeader) {
    let mut et = null_mut::<StgTRecHeader>();

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(c"STM: %p : stmAbortTransaction".as_ptr(), trec);
    }

    et = (*trec).enclosing_trec as *mut StgTRecHeader;

    if et == &raw mut stg_NO_TREC_closure as *mut c_void as *mut StgTRecHeader {
        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
            trace_(c"STM: %p : aborting top-level transaction".as_ptr(), trec);
        }

        if (*trec).state as u32 == TREC_WAITING as i32 as u32 {
            if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
                trace_(
                    c"STM: %p : stmAbortTransaction aborting waiting transaction".as_ptr(),
                    trec,
                );
            }

            remove_watch_queue_entries_for_trec(cap, trec);
        }
    } else {
        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
            trace_(
                c"STM: %p : retaining read-set into parent %p".as_ptr(),
                trec,
                et,
            );
        }

        let mut __t = trec;
        let mut __c = (*__t).current_chunk;
        let mut __limit: StgWord = (*__c).next_entry_idx;

        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
            trace_(
                c"STM: %p : FOR_EACH_ENTRY, current_chunk=%p limit=%ld".as_ptr(),
                __t,
                __c,
                __limit,
            );
        }

        while __c != &raw mut stg_END_STM_CHUNK_LIST_closure as *mut c_void as *mut StgTRecChunk {
            let mut __i: StgWord = 0;
            __i = 0;

            while __i < __limit {
                let mut e: *mut TRecEntry = (&raw mut (*__c).entries as *mut TRecEntry)
                    .offset(__i as isize)
                    as *mut TRecEntry;

                let mut s = (*e).tvar;
                merge_read_into(cap, et, s, (*e).expected_value);
                __i = __i.wrapping_add(1);
            }

            __c = (*__c).prev_chunk as *mut StgTRecChunk;
            __limit = TREC_CHUNK_NUM_ENTRIES as StgWord;
        }
    }

    (*trec).state = TREC_ABORTED;

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(c"STM: %p : stmAbortTransaction done".as_ptr(), trec);
    }
}

unsafe fn stmFreeAbortedTRec(mut cap: *mut Capability, mut trec: *mut StgTRecHeader) {
    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(c"STM: %p : stmFreeAbortedTRec".as_ptr(), trec);
    }

    free_stg_trec_header(cap, trec);

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(c"STM: %p : stmFreeAbortedTRec done".as_ptr(), trec);
    }
}

unsafe fn stmCondemnTransaction(mut cap: *mut Capability, mut trec: *mut StgTRecHeader) {
    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(c"STM: %p : stmCondemnTransaction".as_ptr(), trec);
    }

    if (*trec).state as u32 == TREC_WAITING as i32 as u32 {
        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
            trace_(
                c"STM: %p : stmCondemnTransaction condemning waiting transaction".as_ptr(),
                trec,
            );
        }

        remove_watch_queue_entries_for_trec(cap, trec);
    }

    (*trec).state = TREC_CONDEMNED;

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(c"STM: %p : stmCondemnTransaction done".as_ptr(), trec);
    }
}

unsafe fn stmValidateNestOfTransactions(
    mut cap: *mut Capability,
    mut trec: *mut StgTRecHeader,
    mut optimistically: StgBool,
) -> StgBool {
    let mut t = null_mut::<StgTRecHeader>();

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(
            c"STM: %p : stmValidateNestOfTransactions, %b".as_ptr(),
            trec,
            optimistically,
        );
    }

    t = trec;

    let mut result = true;

    while t != &raw mut stg_NO_TREC_closure as *mut c_void as *mut StgTRecHeader {
        if optimistically != 0 {
            result &= validate_trec_optimistic(cap, t);
        } else {
            result &= validate_and_acquire_ownership(cap, t, true, false);
        }

        t = (*t).enclosing_trec as *mut StgTRecHeader;
    }

    if result == 0 && (*trec).state as u32 != TREC_WAITING as i32 as u32 {
        (*trec).state = TREC_CONDEMNED;
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(
            c"STM: %p : stmValidateNestOfTransactions()=%d".as_ptr(),
            trec,
            result,
        );
    }

    return result;
}

unsafe fn get_entry_for(
    mut trec: *mut StgTRecHeader,
    mut tvar: *mut StgTVar,
    mut r#in: *mut *mut StgTRecHeader,
) -> *mut TRecEntry {
    let mut result = null_mut::<TRecEntry>();

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(c"STM: %p : get_entry_for TVar %p".as_ptr(), trec, tvar);
    }

    loop {
        let mut __t = trec;
        let mut __c = (*__t).current_chunk;
        let mut __limit: StgWord = (*__c).next_entry_idx;

        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
            trace_(
                c"STM: %p : FOR_EACH_ENTRY, current_chunk=%p limit=%ld".as_ptr(),
                __t,
                __c,
                __limit,
            );
        }

        's_60: while __c
            != &raw mut stg_END_STM_CHUNK_LIST_closure as *mut c_void as *mut StgTRecChunk
        {
            let mut __i: StgWord = 0;
            __i = 0;

            while __i < __limit {
                let mut e: *mut TRecEntry = (&raw mut (*__c).entries as *mut TRecEntry)
                    .offset(__i as isize)
                    as *mut TRecEntry;

                if (*e).tvar == tvar {
                    result = e;

                    if !r#in.is_null() {
                        *r#in = trec;
                    }

                    break 's_60;
                } else {
                    __i = __i.wrapping_add(1);
                }
            }

            __c = (*__c).prev_chunk as *mut StgTRecChunk;
            __limit = TREC_CHUNK_NUM_ENTRIES as StgWord;
        }

        trec = (*trec).enclosing_trec as *mut StgTRecHeader;

        if !(result.is_null()
            && trec != &raw mut stg_NO_TREC_closure as *mut c_void as *mut StgTRecHeader)
        {
            break;
        }
    }

    return result;
}

unsafe fn stmCommitTransaction(mut cap: *mut Capability, mut trec: *mut StgTRecHeader) -> StgBool {
    let mut max_commits_at_start = getMaxCommits();

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(c"STM: %p : stmCommitTransaction()".as_ptr(), trec);
    }

    let mut result =
        validate_and_acquire_ownership(cap, trec, (config_use_read_phase == 0) as i32, true) != 0;

    if result {
        if config_use_read_phase != 0 {
            let mut max_commits_at_end: StgInt64 = 0;
            let mut max_concurrent_commits: StgInt64 = 0;

            if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
                trace_(c"STM: %p : doing read check".as_ptr(), trec);
            }

            result = check_read_only(trec) != 0;

            if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
                trace_(
                    c"STM: %p : read-check %s".as_ptr(),
                    trec,
                    if result as i32 != 0 {
                        c"succeeded".as_ptr()
                    } else {
                        c"failed".as_ptr()
                    },
                );
            }

            max_commits_at_end = getMaxCommits();
            max_concurrent_commits = max_commits_at_end - max_commits_at_start
                + getNumCapabilities().wrapping_mul(TOKEN_BATCH_SIZE as u32) as StgInt64;

            if max_concurrent_commits >> 32 > 0 || shake() != 0 {
                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
                    trace_(c"STM: STM - Max commit number exceeded".as_ptr());
                }

                result = false;
            }
        }

        if result {
            let mut __t = trec;
            let mut __c = (*__t).current_chunk;
            let mut __limit: StgWord = (*__c).next_entry_idx;

            if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
                trace_(
                    c"STM: %p : FOR_EACH_ENTRY, current_chunk=%p limit=%ld".as_ptr(),
                    __t,
                    __c,
                    __limit,
                );
            }

            while __c != &raw mut stg_END_STM_CHUNK_LIST_closure as *mut c_void as *mut StgTRecChunk
            {
                let mut __i: StgWord = 0;
                __i = 0;

                while __i < __limit {
                    let mut e: *mut TRecEntry = (&raw mut (*__c).entries as *mut TRecEntry)
                        .offset(__i as isize)
                        as *mut TRecEntry;

                    let mut s = null_mut::<StgTVar>();
                    s = (*e).tvar;

                    if config_use_read_phase == 0 || (*e).new_value != (*e).expected_value {
                        if 0 != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
                            trace_(
                                c"STM: %p : writing %p to %p, waking waiters".as_ptr(),
                                trec,
                                (*e).new_value,
                                s,
                            );
                        }

                        unpark_waiters_on(cap, s);
                        unlock_tvar(cap, trec, s, (*e).new_value, 1);
                    }

                    __i = __i.wrapping_add(1);
                }

                __c = (*__c).prev_chunk as *mut StgTRecChunk;
                __limit = TREC_CHUNK_NUM_ENTRIES as StgWord;
            }
        } else {
            revert_ownership(cap, trec, false);
        }
    }

    free_stg_trec_header(cap, trec);

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(
            c"STM: %p : stmCommitTransaction()=%d".as_ptr(),
            trec,
            result as i32,
        );
    }

    return result as StgBool;
}

unsafe fn stmCommitNestedTransaction(
    mut cap: *mut Capability,
    mut trec: *mut StgTRecHeader,
) -> StgBool {
    let mut et = null_mut::<StgTRecHeader>();

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(
            c"STM: %p : stmCommitNestedTransaction() into %p".as_ptr(),
            trec,
            (*trec).enclosing_trec,
        );
    }

    et = (*trec).enclosing_trec as *mut StgTRecHeader;

    let mut result =
        validate_and_acquire_ownership(cap, trec, (config_use_read_phase == 0) as i32, true) != 0;

    if result {
        if config_use_read_phase != 0 {
            if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
                trace_(c"STM: %p : doing read check".as_ptr(), trec);
            }

            result = check_read_only(trec) != 0;
        }

        if result {
            if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
                trace_(c"STM: %p : read-check succeeded".as_ptr(), trec);
            }

            let mut __t = trec;
            let mut __c = (*__t).current_chunk;
            let mut __limit: StgWord = (*__c).next_entry_idx;

            if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
                trace_(
                    c"STM: %p : FOR_EACH_ENTRY, current_chunk=%p limit=%ld".as_ptr(),
                    __t,
                    __c,
                    __limit,
                );
            }

            while __c != &raw mut stg_END_STM_CHUNK_LIST_closure as *mut c_void as *mut StgTRecChunk
            {
                let mut __i: StgWord = 0;
                __i = 0;

                while __i < __limit {
                    let mut e: *mut TRecEntry = (&raw mut (*__c).entries as *mut TRecEntry)
                        .offset(__i as isize)
                        as *mut TRecEntry;

                    let mut s = null_mut::<StgTVar>();
                    s = (*e).tvar;

                    if entry_is_update(e) != 0 {
                        unlock_tvar(cap, trec, s, (*e).expected_value, 0);
                    }

                    merge_update_into(cap, et, s, (*e).expected_value, (*e).new_value);
                    __i = __i.wrapping_add(1);
                }

                __c = (*__c).prev_chunk as *mut StgTRecChunk;
                __limit = TREC_CHUNK_NUM_ENTRIES as StgWord;
            }
        } else {
            revert_ownership(cap, trec, false);
        }
    }

    free_stg_trec_header(cap, trec);

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(
            c"STM: %p : stmCommitNestedTransaction()=%d".as_ptr(),
            trec,
            result as i32,
        );
    }

    return result as StgBool;
}

unsafe fn stmWait(
    mut cap: *mut Capability,
    mut tso: *mut StgTSO,
    mut trec: *mut StgTRecHeader,
) -> StgBool {
    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(c"STM: %p : stmWait(%p)".as_ptr(), trec, tso);
    }

    let mut result = validate_and_acquire_ownership(cap, trec, true, true) != 0;

    if result {
        build_watch_queue_entries_for_trec(cap, tso, trec);
        park_tso(tso);
        (*trec).state = TREC_WAITING;
    } else {
        free_stg_trec_header(cap, trec);
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(
            c"STM: %p : stmWait(%p)=%d".as_ptr(),
            trec,
            tso,
            result as i32,
        );
    }

    return result as StgBool;
}

unsafe fn stmWaitUnlock(mut cap: *mut Capability, mut trec: *mut StgTRecHeader) {
    revert_ownership(cap, trec, true);
}

unsafe fn stmReWait(mut cap: *mut Capability, mut tso: *mut StgTSO) -> StgBool {
    let mut trec = (*tso).trec as *mut StgTRecHeader;

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(c"STM: %p : stmReWait".as_ptr(), trec);
    }

    let mut result = validate_and_acquire_ownership(cap, trec, true, true) != 0;

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(
            c"STM: %p : validation %s".as_ptr(),
            trec,
            if result as i32 != 0 {
                c"succeeded".as_ptr()
            } else {
                c"failed".as_ptr()
            },
        );
    }

    if result {
        park_tso(tso);
        revert_ownership(cap, trec, true);
    } else {
        if (*trec).state as u32 != TREC_CONDEMNED as i32 as u32 {
            remove_watch_queue_entries_for_trec(cap, trec);
        }

        free_stg_trec_header(cap, trec);
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(c"STM: %p : stmReWait()=%d".as_ptr(), trec, result as i32);
    }

    return result as StgBool;
}

unsafe fn read_current_value(
    mut trec: *mut StgTRecHeader,
    mut tvar: *mut StgTVar,
) -> *mut StgClosure {
    let mut result = null_mut::<StgClosure>();
    result = (*tvar).current_value;

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(
            c"STM: %p : read_current_value(%p)=%p".as_ptr(),
            trec,
            tvar,
            result,
        );
    }

    return result;
}

unsafe fn stmReadTVar(
    mut cap: *mut Capability,
    mut trec: *mut StgTRecHeader,
    mut tvar: *mut StgTVar,
) -> *mut StgClosure {
    let mut entry_in = null_mut::<StgTRecHeader>();
    let mut result = null_mut::<StgClosure>();
    let mut entry = null_mut::<TRecEntry>();

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(c"STM: %p : stmReadTVar(%p)".as_ptr(), trec, tvar);
    }

    entry = get_entry_for(trec, tvar, &raw mut entry_in);

    if !entry.is_null() {
        if entry_in == trec {
            result = (*entry).new_value;
        } else {
            let mut new_entry = get_new_entry(cap, trec);
            (*new_entry).tvar = tvar;
            (*new_entry).expected_value = (*entry).expected_value;
            (*new_entry).new_value = (*entry).new_value;
            result = (*new_entry).new_value;
        }
    } else {
        let mut current_value = read_current_value(trec, tvar);
        let mut new_entry_0 = get_new_entry(cap, trec);
        (*new_entry_0).tvar = tvar;
        (*new_entry_0).expected_value = current_value;
        (*new_entry_0).new_value = current_value;
        result = current_value;
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(c"STM: %p : stmReadTVar(%p)=%p".as_ptr(), trec, tvar, result);
    }

    return result;
}

unsafe fn stmWriteTVar(
    mut cap: *mut Capability,
    mut trec: *mut StgTRecHeader,
    mut tvar: *mut StgTVar,
    mut new_value: *mut StgClosure,
) {
    let mut entry_in = null_mut::<StgTRecHeader>();
    let mut entry = null_mut::<TRecEntry>();

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(
            c"STM: %p : stmWriteTVar(%p, %p)".as_ptr(),
            trec,
            tvar,
            new_value,
        );
    }

    entry = get_entry_for(trec, tvar, &raw mut entry_in);

    if !entry.is_null() {
        if entry_in == trec {
            (*entry).new_value = new_value;
        } else {
            let mut new_entry = get_new_entry(cap, trec);
            (*new_entry).tvar = tvar;
            (*new_entry).expected_value = (*entry).expected_value;
            (*new_entry).new_value = new_value;
        }
    } else {
        let mut current_value = read_current_value(trec, tvar);
        let mut new_entry_0 = get_new_entry(cap, trec);
        (*new_entry_0).tvar = tvar;
        (*new_entry_0).expected_value = current_value;
        (*new_entry_0).new_value = new_value;
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
        trace_(c"STM: %p : stmWriteTVar done".as_ptr(), trec);
    }
}
