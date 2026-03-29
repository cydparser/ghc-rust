use crate::capability::getCapability;
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::messages::barf;
use crate::ffi::rts::storage::block::Bdescr;
use crate::ffi::rts::storage::closure_macros::INFO_PTR_TO_STRUCT;
use crate::ffi::rts::storage::closures::{_StgWeak, StgWeak};
use crate::ffi::rts::storage::gc::{g0, generation, generations, oldest_gen};
use crate::ffi::rts::storage::tso::StgTSO_;
use crate::ffi::rts::threads::getNumCapabilities;
use crate::ffi::rts::types::{StgClosure, StgTSO};
use crate::ffi::stg::misc_closures::{
    stg_DEAD_WEAK_info, stg_END_TSO_QUEUE_closure, stg_NO_FINALIZER_closure,
};
use crate::ffi::stg::types::StgPtr;
use crate::prelude::*;
use crate::sm::evac::evacuate;
use crate::sm::gc::{N, isAlive};
use crate::sm::gc_thread::gc_thread;
use crate::sm::gc_utils::recordMutableGen_GC;
use crate::sm::gct_decl::the_gc_thread;
use crate::trace::{DEBUG_RTS, trace_};

type WeakStage = u32;

const WeakDone: WeakStage = 2;

const WeakThreads: WeakStage = 1;

const WeakPtrs: WeakStage = 0;

static mut weak_stage: WeakStage = WeakPtrs;

unsafe fn initWeakForGC() {
    let mut oldest: u32 = N;

    if RtsFlags.GcFlags.useNonmoving as i32 != 0 && N == (*oldest_gen).no {
        oldest = (*oldest_gen).no.wrapping_sub(1 as u32);
    }

    let mut g: u32 = 0;

    while g <= oldest {
        let mut r#gen: *mut generation = generations.offset(g as isize) as *mut generation;
        (*r#gen).old_weak_ptr_list = (*r#gen).weak_ptr_list;
        (*r#gen).weak_ptr_list = null_mut::<StgWeak>();
        g = g.wrapping_add(1);
    }

    weak_stage = WeakThreads;
}

unsafe fn traverseWeakPtrList(
    mut dead_weak_ptr_list: *mut *mut StgWeak,
    mut resurrected_threads: *mut *mut StgTSO,
) -> bool {
    let mut flag = false;

    match weak_stage as u32 {
        2 => return false,
        1 => {
            let mut g: u32 = 0;
            g = 0;

            while g <= N {
                tidyThreadList(generations.offset(g as isize) as *mut generation);
                g = g.wrapping_add(1);
            }

            g = 0;

            while g <= N {
                if tidyWeakList(generations.offset(g as isize) as *mut generation) {
                    flag = true;
                }

                g = g.wrapping_add(1);
            }

            if flag {
                return true;
            }

            g = 0;

            while g <= N {
                if resurrectUnreachableThreads(
                    generations.offset(g as isize) as *mut generation,
                    resurrected_threads,
                ) {
                    flag = true;
                }

                g = g.wrapping_add(1);
            }

            weak_stage = WeakPtrs;

            if flag {
                return true;
            }
        }
        0 => {}
        _ => {
            barf(c"traverseWeakPtrList".as_ptr());
        }
    }

    let mut g_0: u32 = 0;
    g_0 = 0;

    while g_0 <= N {
        if tidyWeakList(generations.offset(g_0 as isize) as *mut generation) {
            flag = true;
        }

        g_0 = g_0.wrapping_add(1);
    }

    if flag as i32 == false {
        g_0 = 0;

        while g_0 <= N {
            collectDeadWeakPtrs(
                generations.offset(g_0 as isize) as *mut generation,
                dead_weak_ptr_list,
            );

            g_0 = g_0.wrapping_add(1);
        }

        weak_stage = WeakDone;
    }

    return true;
}

unsafe fn collectDeadWeakPtrs(
    mut r#gen: *mut generation,
    mut dead_weak_ptr_list: *mut *mut StgWeak,
) {
    let mut w = null_mut::<StgWeak>();
    let mut next_w = null_mut::<StgWeak>();
    w = (*r#gen).old_weak_ptr_list;

    while !w.is_null() {
        if (*w).cfinalizers != &raw mut stg_NO_FINALIZER_closure {
            evacuate(&raw mut (*w).value);
        }

        evacuate(&raw mut (*w).finalizer);
        next_w = (*w).link as *mut StgWeak;
        (*w).link = *dead_weak_ptr_list as *mut _StgWeak;
        *dead_weak_ptr_list = w;
        w = next_w;
    }
}

unsafe fn resurrectUnreachableThreads(
    mut r#gen: *mut generation,
    mut resurrected_threads: *mut *mut StgTSO,
) -> bool {
    let mut t = null_mut::<StgTSO>();
    let mut next = null_mut::<StgTSO>();
    let mut flag = false;
    t = (*r#gen).old_threads;

    while t != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
        next = (*t).global_link as *mut StgTSO;

        match (*t).what_next as i32 {
            ThreadKilled | ThreadComplete => {
                (*t).global_link = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
                    as *mut StgTSO_;
            }
            _ => {
                let mut tmp = t;
                evacuate(&raw mut tmp as *mut *mut StgClosure);
                (*tmp).global_link = *resurrected_threads as *mut StgTSO_;
                *resurrected_threads = tmp;
                flag = true;
            }
        }

        t = next;
    }

    (*r#gen).old_threads = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO;

    return flag;
}

unsafe fn tidyWeakList(mut r#gen: *mut generation) -> bool {
    if RtsFlags.GcFlags.useNonmoving as i32 != 0 && r#gen == oldest_gen {
        return false;
    }

    let mut w = null_mut::<StgWeak>();
    let mut last_w = null_mut::<*mut StgWeak>();
    let mut next_w = null_mut::<StgWeak>();
    let mut info = null::<StgInfoTable>();
    let mut new = null_mut::<StgClosure>();
    let mut flag = false;
    last_w = &raw mut (*r#gen).old_weak_ptr_list;
    w = (*r#gen).old_weak_ptr_list;

    while !w.is_null() {
        info = (*w).header.info;

        if info == &raw const stg_DEAD_WEAK_info {
            next_w = (*w).link as *mut StgWeak;
            *last_w = next_w;
        } else {
            info = INFO_PTR_TO_STRUCT(info);

            match (*info).r#type {
                49 => {
                    new = isAlive((*w).key);

                    if !new.is_null() {
                        let mut new_gen = null_mut::<generation>();
                        (*w).key = new;
                        new_gen = (*Bdescr(w as StgPtr)).r#gen as *mut generation;
                        (*(&raw mut the_gc_thread as *mut gc_thread)).evac_gen_no = (*new_gen).no;
                        (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac = false;
                        scavengeLiveWeak(w);

                        if (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac {
                            if DEBUG_RTS != 0 && RtsFlags.DebugFlags.weak as i64 != 0 {
                                trace_(c"putting weak pointer %p into mutable list".as_ptr(), w);
                            }

                            (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac = false;
                            recordMutableGen_GC(w as *mut StgClosure, (*new_gen).no);
                        }

                        *last_w = (*w).link as *mut StgWeak;
                        next_w = (*w).link as *mut StgWeak;
                        (*w).link = (*new_gen).weak_ptr_list as *mut _StgWeak;
                        (*new_gen).weak_ptr_list = w;
                        flag = true;

                        if (*r#gen).no != (*new_gen).no {
                            if DEBUG_RTS != 0 && RtsFlags.DebugFlags.weak as i64 != 0 {
                                trace_(
                                    c"moving weak pointer %p from %d to %d".as_ptr(),
                                    w,
                                    (*r#gen).no,
                                    (*new_gen).no,
                                );
                            }
                        }

                        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.weak as i64 != 0 {
                            trace_(
                                c"weak pointer still alive at %p -> %p".as_ptr(),
                                w,
                                (*w).key,
                            );
                        }
                    } else {
                        last_w = &raw mut (*w).link as *mut *mut StgWeak;
                        next_w = (*w).link as *mut StgWeak;
                    }
                }
                _ => {
                    barf(
                        c"tidyWeakList: not WEAK: %d, %p".as_ptr(),
                        (*info).r#type,
                        w,
                    );
                }
            }
        }

        w = next_w;
    }

    return flag;
}

unsafe fn tidyThreadList(mut r#gen: *mut generation) {
    let mut next = null_mut::<StgTSO>();
    let mut prev: *mut *mut StgTSO = &raw mut (*r#gen).old_threads;
    let mut t = (*r#gen).old_threads;

    while t != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
        let mut tmp = isAlive(t as *mut StgClosure) as *mut StgTSO;

        if !tmp.is_null() {
            t = tmp;
        }

        next = (*t).global_link as *mut StgTSO;

        if tmp.is_null() {
            prev = &raw mut (*t).global_link as *mut *mut StgTSO;
        } else {
            *prev = next;

            let mut new_gen = (*Bdescr(t as StgPtr)).r#gen as *mut generation;
            (*t).global_link = (*new_gen).threads as *mut StgTSO_;
            (*new_gen).threads = t;
        }

        t = next;
    }
}

unsafe fn collectFreshWeakPtrs() {
    let mut i: u32 = 0;
    i = 0;

    while i < getNumCapabilities() as u32 {
        let mut cap = getCapability(i);

        if !(*cap).weak_ptr_list_tl.is_null() {
            (*(*cap).weak_ptr_list_tl).link = (*g0).weak_ptr_list as *mut _StgWeak;
            (*g0).weak_ptr_list = (*cap).weak_ptr_list_hd;
            (*cap).weak_ptr_list_tl = null_mut::<StgWeak>();
            (*cap).weak_ptr_list_hd = null_mut::<StgWeak>();
        }

        i = i.wrapping_add(1);
    }
}

unsafe fn markWeakPtrList() {
    let mut g: u32 = 0;
    g = 0;

    while g <= N {
        let mut r#gen: *mut generation = generations.offset(g as isize) as *mut generation;

        let mut w = null_mut::<StgWeak>();
        let mut last_w = null_mut::<*mut StgWeak>();
        last_w = &raw mut (*r#gen).weak_ptr_list;
        w = (*r#gen).weak_ptr_list;

        while !w.is_null() {
            evacuate(last_w as *mut *mut StgClosure);
            w = *last_w;
            last_w = &raw mut (*w).link as *mut *mut StgWeak;
            w = (*w).link as *mut StgWeak;
        }

        g = g.wrapping_add(1);
    }
}

unsafe fn scavengeLiveWeak(mut w: *mut StgWeak) {
    evacuate(&raw mut (*w).value);
    evacuate(&raw mut (*w).key);
    evacuate(&raw mut (*w).finalizer);
    evacuate(&raw mut (*w).cfinalizers);
}
