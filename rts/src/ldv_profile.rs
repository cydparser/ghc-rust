use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::messages::barf;
use crate::ffi::rts::prof::ccs::era;
use crate::ffi::rts::storage::block::{BF_PINNED, bdescr};
use crate::ffi::rts::storage::closure_macros::{
    INFO_PTR_TO_STRUCT, LDV_recordDead, closure_sizeW, get_itbl,
};
use crate::ffi::rts::storage::gc::generations;
use crate::ffi::rts::types::StgClosure;
use crate::ffi::stg::types::{StgHalfWord, StgWord};
use crate::prelude::*;

unsafe fn isInherentlyUsed(mut closure_type: StgHalfWord) -> bool {
    match closure_type {
        52 | 53 | 39 | 40 | 41 | 43 | 44 | 46 | 45 | 59 | 60 | 62 | 61 | 42 | 49 | 47 | 48 | 23
        | 50 | 51 | 54 => return r#true != 0,
        _ => return r#false != 0,
    };
}

#[inline]
unsafe fn processHeapClosureForDead(mut c: *const StgClosure) -> uint32_t {
    let mut size: uint32_t = 0;
    let mut info = null::<StgInfoTable>();
    info = get_itbl(c);
    info = (*c).header.info;

    if info as StgWord & 1 as StgWord != 0 as StgWord {
        return (*(c as *mut StgClosure)).header.prof.hp.ldvw as uint32_t;
    }

    info = INFO_PTR_TO_STRUCT(info);
    size = closure_sizeW(c);

    if isInherentlyUsed((*info).r#type) {
        return size;
    }

    match (*info).r#type {
        15 | 16 | 17 | 22 | 18 | 19 | 20 | 24 | 25 | 26 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
        | 10 | 11 | 12 | 13 | 64 | 38 | 37 | 27 => {
            LDV_recordDead(c, size);

            return size;
        }
        28 | 14 | 21 | 33 | 34 | 35 | 36 | 29 | 30 | 31 | 57 | 56 | 55 | 65 | 0 | 63 | _ => {
            barf(
                b"Invalid object in processHeapClosureForDead(): %d\0" as *const u8
                    as *const c_char,
                (*info).r#type,
            );
        }
    };
}

unsafe fn processHeapForDead(mut bd: *mut bdescr) {
    let mut p = null_mut::<StgWord>();

    while !bd.is_null() {
        p = (*bd).start;

        while p < (*bd).c2rust_unnamed.free {
            p = p.offset(processHeapClosureForDead(p as *mut StgClosure) as isize);

            while p < (*bd).c2rust_unnamed.free && *p == 0 {
                p = p.offset(1);
            }
        }

        bd = (*bd).link as *mut bdescr;
    }
}

unsafe fn processChainForDead(mut bd: *mut bdescr) {
    while !bd.is_null() {
        if (*bd).flags as c_int & BF_PINNED == 0 {
            processHeapClosureForDead((*bd).start as *mut StgClosure);
        }

        bd = (*bd).link as *mut bdescr;
    }
}

unsafe fn LdvCensusForDead(mut N: uint32_t) {
    let mut g: uint32_t = 0;

    if era == 0 as c_uint {
        return;
    }

    if RtsFlags.GcFlags.generations == 1 as uint32_t {
        barf(b"Lag/Drag/Void profiling not supported with -G1\0" as *const u8 as *const c_char);
    } else {
        g = 0 as uint32_t;

        while g <= N {
            processHeapForDead((*generations.offset(g as isize)).old_blocks);
            processChainForDead((*generations.offset(g as isize)).large_objects);
            g = g.wrapping_add(1);
        }
    };
}

unsafe fn LdvCensusKillAll() {
    LdvCensusForDead(RtsFlags.GcFlags.generations.wrapping_sub(1 as uint32_t));
}
