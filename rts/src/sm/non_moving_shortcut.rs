use crate::ffi::rts::messages::barf;
use crate::ffi::rts::storage::closure_macros::{
    GET_CLOSURE_TAG, INFO_PTR_TO_STRUCT, UNTAG_CLOSURE, get_itbl, get_itbl_acquire,
};
use crate::ffi::rts::storage::closure_types::THUNK_SELECTOR;
use crate::ffi::rts::storage::closures::{StgClosure_, StgInd, StgSelector};
use crate::ffi::rts::types::{StgClosure, StgInfoTable};
use crate::ffi::stg::misc_closures::{
    stg_BLOCKING_QUEUE_CLEAN_info, stg_BLOCKING_QUEUE_DIRTY_info, stg_IND_info, stg_TSO_info,
    stg_WHITEHOLE_info,
};
use crate::ffi::stg::smp::cas;
use crate::ffi::stg::types::{StgHalfWord, StgVolatilePtr, StgWord};
use crate::prelude::*;
use crate::sm::non_moving::isNonmovingClosure;
use crate::sm::non_moving_mark::{MarkQueue, markQueuePushClosure};
use crate::smp_closure_ops::{lockClosure, unlockClosure};

const MAX_THUNK_SELECTOR_DEPTH: c_int = 16 as c_int;

unsafe fn update_selector_chain(
    mut chain: *mut StgClosure,
    mut origin: *mut *mut StgClosure,
    p0: *mut StgSelector,
    val: *mut StgClosure,
) {
    while !chain.is_null() {
        let mut next = *(&raw mut (*chain).payload as *mut *mut StgClosure_)
            .offset(0 as c_int as isize) as *mut StgClosure;

        let ref mut fresh7 = (*(chain as *mut StgInd)).indirectee;
        *fresh7 = val;
        unlockClosure(chain, &raw const stg_IND_info);
        chain = next;
    }

    if !origin.is_null() && p0 as *mut StgClosure != val {
        cas(origin as StgVolatilePtr, p0 as StgWord, val as StgWord);
    }
}

unsafe fn nonmoving_eval_thunk_selector_(
    mut queue: *mut MarkQueue,
    p0: *mut StgSelector,
    origin: *mut *mut StgClosure,
    mut depth: c_int,
) -> *mut StgClosure {
    let mut selectee_info_tbl: *const StgInfoTable = null::<StgInfoTable>();
    let mut val: *mut StgClosure = null_mut::<StgClosure>();
    let mut indirectee: *mut StgClosure = null_mut::<StgClosure>();
    markQueuePushClosure(queue, p0 as *mut StgClosure, null_mut::<*mut StgClosure>());

    let mut p = p0 as *mut StgClosure;
    let mut chain = null_mut::<StgClosure>();

    loop {
        let mut selector_info_ptr: *const StgInfoTable = lockClosure(p);
        let mut selector_info_tbl = INFO_PTR_TO_STRUCT(selector_info_ptr);

        if (*selector_info_tbl).r#type != THUNK_SELECTOR as StgHalfWord {
            unlockClosure(p, selector_info_ptr);
            update_selector_chain(chain, origin, p0, p);

            return p;
        }

        let mut field: uint32_t = (*selector_info_tbl).layout.selector_offset as uint32_t;
        let mut selectee = UNTAG_CLOSURE((*(p as *mut StgSelector)).selectee);

        loop {
            if !isNonmovingClosure(selectee) {
                unlockClosure(p, selector_info_ptr);
                update_selector_chain(chain, origin, p0, p);

                return p;
            }

            markQueuePushClosure(queue, selectee, null_mut::<*mut StgClosure>());
            selectee_info_tbl = get_itbl_acquire(selectee);

            match (*selectee_info_tbl).r#type {
                58 => {
                    unlockClosure(p, selector_info_ptr);
                    update_selector_chain(chain, origin, p0, p);

                    return p;
                }
                1 | 2 | 3 | 4 | 5 | 6 | 7 => {
                    val = UNTAG_CLOSURE(
                        *(&raw mut (*selectee).payload as *mut *mut StgClosure_)
                            .offset(field as isize) as *mut StgClosure,
                    );

                    break;
                }
                27 | 28 => {
                    let mut indirectee_0 = UNTAG_CLOSURE((*(selectee as *mut StgInd)).indirectee);

                    if isNonmovingClosure(indirectee_0) {
                        selectee = indirectee_0;
                    } else {
                        unlockClosure(p, selector_info_ptr);
                        update_selector_chain(chain, origin, p0, p);

                        return p;
                    }
                }
                38 => {
                    let mut indirectee_1 = (*(selectee as *mut StgInd)).indirectee;

                    if !isNonmovingClosure(UNTAG_CLOSURE(indirectee_1)) {
                        unlockClosure(p, selector_info_ptr);
                        update_selector_chain(chain, origin, p0, p);

                        return p;
                    }

                    if GET_CLOSURE_TAG(indirectee_1) == 0 as StgWord {
                        let mut i = (*indirectee_1).header.info;

                        if i == &raw const stg_TSO_info
                            || i == &raw const stg_WHITEHOLE_info
                            || i == &raw const stg_BLOCKING_QUEUE_CLEAN_info
                            || i == &raw const stg_BLOCKING_QUEUE_DIRTY_info
                        {
                            unlockClosure(p, selector_info_ptr);
                            update_selector_chain(chain, origin, p0, p);

                            return p;
                        }
                    }

                    selectee = UNTAG_CLOSURE(indirectee_1);
                }
                24 | 26 | 15 | 16 | 17 | 18 | 19 | 20 | 21 => {
                    unlockClosure(p, selector_info_ptr);
                    update_selector_chain(chain, origin, p0, p);

                    return p;
                }
                22 => {
                    if depth < MAX_THUNK_SELECTOR_DEPTH {
                        let mut new_selectee = UNTAG_CLOSURE(nonmoving_eval_thunk_selector_(
                            queue,
                            selectee as *mut StgSelector,
                            null_mut::<*mut StgClosure>(),
                            depth + 1 as c_int,
                        ));

                        if selectee == new_selectee {
                            unlockClosure(p, selector_info_ptr);
                            update_selector_chain(chain, origin, p0, p);

                            return p;
                        } else {
                            selectee = new_selectee;
                        }
                    } else {
                        unlockClosure(p, selector_info_ptr);
                        update_selector_chain(chain, origin, p0, p);

                        return p;
                    }
                }
                _ => {
                    barf(
                        b"nonmoving_eval_thunk_selector: strange selectee %d\0" as *const u8
                            as *const c_char,
                        (*selectee_info_tbl).r#type as c_int,
                    );
                }
            }
        }

        loop {
            if !isNonmovingClosure(val) {
                unlockClosure(p, selector_info_ptr);
                update_selector_chain(chain, origin, p0, p);

                return p;
            }

            match (*get_itbl(val)).r#type {
                27 | 28 => {
                    indirectee = UNTAG_CLOSURE((*(val as *mut StgInd)).indirectee);

                    if isNonmovingClosure(indirectee) {
                        val = UNTAG_CLOSURE((*(val as *mut StgInd)).indirectee);
                    } else {
                        unlockClosure(p, selector_info_ptr);
                        update_selector_chain(chain, origin, p0, p);

                        return p;
                    }
                }
                22 => {
                    let ref mut fresh5 = *(&raw mut (*p).payload as *mut *mut StgClosure_)
                        .offset(0 as c_int as isize);
                    *fresh5 = chain as *mut StgClosure_;
                    chain = p;
                    p = val;
                    break;
                }
                _ => {
                    let ref mut fresh6 = *(&raw mut (*p).payload as *mut *mut StgClosure_)
                        .offset(0 as c_int as isize);
                    *fresh6 = chain as *mut StgClosure_;
                    chain = p;
                    update_selector_chain(chain, origin, p0, val);

                    return val;
                }
            }
        }
    }
}

unsafe fn nonmoving_eval_thunk_selector(
    mut queue: *mut MarkQueue,
    mut p: *mut StgSelector,
    mut origin: *mut *mut StgClosure,
) {
    nonmoving_eval_thunk_selector_(queue, p, origin, 0 as c_int);
}
