use crate::ffi::rts::constants::{
    LDV_SHIFT, LDV_STATE_CREATE, MAX_CHARLIKE, MAX_INTLIKE, MIN_CHARLIKE, MIN_INTLIKE,
};
use crate::ffi::rts::prof::ccs::{CCS_SYSTEM, CostCentreStack, era, user_era};
use crate::ffi::rts::rts_to_hs_iface::ghc_hs_iface;
use crate::ffi::rts::storage::closure_macros::{
    doingErasProfiling, doingLDVProfiling, doingRetainerProfiling,
};
use crate::ffi::rts::storage::closures::{
    C2RustUnnamed, StgHeader, StgIntCharlikeClosure, StgProfHeader,
};
use crate::ffi::rts::types::StgClosure;
use crate::ffi::stg::types::StgWord;
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut stg_INTLIKE_closure: [StgIntCharlikeClosure; 272] = [StgIntCharlikeClosure {
    header: StgHeader {
        info: null::<StgInfoTable>(),
        prof: StgProfHeader {
            ccs: null_mut::<CostCentreStack>(),
            hp: C2RustUnnamed { trav: 0 },
        },
    },
    data: 0,
}; 272];

static mut stg_CHARLIKE_closure: [StgIntCharlikeClosure; 256] = [StgIntCharlikeClosure {
    header: StgHeader {
        info: null::<StgInfoTable>(),
        prof: StgProfHeader {
            ccs: null_mut::<CostCentreStack>(),
            hp: C2RustUnnamed { trav: 0 },
        },
    },
    data: 0,
}; 256];

unsafe fn initBuiltinClosures() {
    let mut i = MIN_INTLIKE;

    while i <= MAX_INTLIKE {
        let mut c: *mut StgIntCharlikeClosure =
            (&raw mut stg_INTLIKE_closure as *mut StgIntCharlikeClosure)
                .offset((i - MIN_INTLIKE) as isize) as *mut StgIntCharlikeClosure;

        let ref mut fresh12 = (*(c as *mut StgClosure)).header.prof.ccs;
        *fresh12 = &raw mut CCS_SYSTEM as *mut CostCentreStack;

        if doingLDVProfiling() {
            if doingLDVProfiling() {
                (*(c as *mut StgClosure)).header.prof.hp.ldvw =
                    (era as StgWord) << LDV_SHIFT | LDV_STATE_CREATE as StgWord;
            }
        } else if doingRetainerProfiling() {
            (*(c as *mut StgClosure)).header.prof.hp.trav = 0;
        } else if doingErasProfiling() {
            (*(c as *mut StgClosure)).header.prof.hp.era = user_era;
        }

        (&raw mut (*(c as *mut StgClosure)).header.info)
            .store((*ghc_hs_iface).Izh_con_info, Ordering::Relaxed);
        (*c).data = i as StgWord;
        i += 1;
    }

    let mut i_0 = MIN_CHARLIKE;

    while i_0 <= MAX_CHARLIKE {
        let mut c_0: *mut StgIntCharlikeClosure =
            (&raw mut stg_CHARLIKE_closure as *mut StgIntCharlikeClosure)
                .offset((i_0 - MIN_CHARLIKE) as isize) as *mut StgIntCharlikeClosure;

        let ref mut fresh13 = (*(c_0 as *mut StgClosure)).header.prof.ccs;
        *fresh13 = &raw mut CCS_SYSTEM as *mut CostCentreStack;

        if doingLDVProfiling() {
            if doingLDVProfiling() {
                (*(c_0 as *mut StgClosure)).header.prof.hp.ldvw =
                    (era as StgWord) << LDV_SHIFT | LDV_STATE_CREATE as StgWord;
            }
        } else if doingRetainerProfiling() {
            (*(c_0 as *mut StgClosure)).header.prof.hp.trav = 0;
        } else if doingErasProfiling() {
            (*(c_0 as *mut StgClosure)).header.prof.hp.era = user_era;
        }

        (&raw mut (*(c_0 as *mut StgClosure)).header.info)
            .store((*ghc_hs_iface).Czh_con_info, Ordering::Relaxed);
        (*c_0).data = i_0 as StgWord;
        i_0 += 1;
    }
}
