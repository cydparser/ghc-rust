use crate::ffi::rts::_assertFail;
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
use crate::rts_flags::RtsFlags;

unsafe fn isInherentlyUsed(mut closure_type: StgHalfWord) -> bool {
    match closure_type {
        52 | 53 | 39 | 40 | 41 | 43 | 44 | 46 | 45 | 59 | 60 | 62 | 61 | 42 | 49 | 47 | 48 | 23
        | 50 | 51 | 54 => return true,
        _ => return false,
    };
}

unsafe fn processHeapClosureForDead(mut c: *const StgClosure) -> u32 {
    let mut size: u32 = 0;
    let mut info = null::<StgInfoTable>();
    info = get_itbl(c);
    info = (*c).header.info;

    if info as StgWord & 1 != 0 {
        return (*(c as *mut StgClosure)).header.prof.hp.ldvw as u32;
    }

    info = INFO_PTR_TO_STRUCT(info);

    if (((*(c as *mut StgClosure)).header.prof.hp.ldvw & 0xfffffffc0000000) >> 30 <= era as StgWord
        && ((*(c as *mut StgClosure)).header.prof.hp.ldvw & 0xfffffffc0000000) >> 30 > 0)
        as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/LdvProfile.c".as_ptr(), 79);
    }

    if ((*(c as *mut StgClosure)).header.prof.hp.ldvw & 0x1000000000000000 == 0
        || (*(c as *mut StgClosure)).header.prof.hp.ldvw & 0x3fffffff <= era as StgWord
            && (*(c as *mut StgClosure)).header.prof.hp.ldvw & 0x3fffffff > 0) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/LdvProfile.c".as_ptr(), 84);
    }

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
                c"Invalid object in processHeapClosureForDead(): %d".as_ptr(),
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

        if (p == (*bd).c2rust_unnamed.free) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/LdvProfile.c".as_ptr(), 183);
        }

        bd = (*bd).link as *mut bdescr;
    }
}

unsafe fn processChainForDead(mut bd: *mut bdescr) {
    while !bd.is_null() {
        if (*bd).flags as i32 & BF_PINNED == 0 {
            processHeapClosureForDead((*bd).start as *mut StgClosure);
        }

        bd = (*bd).link as *mut bdescr;
    }
}

unsafe fn LdvCensusForDead(mut N: u32) {
    let mut g: u32 = 0;

    if era == 0 {
        return;
    }

    if RtsFlags.GcFlags.generations == 1 {
        barf(c"Lag/Drag/Void profiling not supported with -G1".as_ptr());
    } else {
        g = 0;

        while g <= N {
            processHeapForDead((*generations.offset(g as isize)).old_blocks);
            processChainForDead((*generations.offset(g as isize)).large_objects);
            g = g.wrapping_add(1);
        }
    };
}

unsafe fn LdvCensusKillAll() {
    LdvCensusForDead(RtsFlags.GcFlags.generations.wrapping_sub(1 as u32));
}
