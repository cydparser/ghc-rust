use crate::ffi::rts::constants::{MAX_CHARLIKE, MAX_INTLIKE, MIN_CHARLIKE, MIN_INTLIKE};
use crate::ffi::rts::rts_to_hs_iface::ghc_hs_iface;
use crate::ffi::rts::storage::closures::{StgHeader, StgIntCharlikeClosure};
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
    },
    data: 0,
}; 272];

static mut stg_CHARLIKE_closure: [StgIntCharlikeClosure; 256] = [StgIntCharlikeClosure {
    header: StgHeader {
        info: null::<StgInfoTable>(),
    },
    data: 0,
}; 256];

unsafe fn initBuiltinClosures() {
    let mut i = MIN_INTLIKE;

    while i <= MAX_INTLIKE {
        let mut c: *mut StgIntCharlikeClosure =
            (&raw mut stg_INTLIKE_closure as *mut StgIntCharlikeClosure)
                .offset((i - MIN_INTLIKE) as isize) as *mut StgIntCharlikeClosure;

        let ref mut fresh5 = (*(c as *mut StgClosure)).header.info;
        *fresh5 = (*ghc_hs_iface).Izh_con_info;
        (*c).data = i as StgWord;
        i += 1;
    }

    let mut i_0 = MIN_CHARLIKE;

    while i_0 <= MAX_CHARLIKE {
        let mut c_0: *mut StgIntCharlikeClosure =
            (&raw mut stg_CHARLIKE_closure as *mut StgIntCharlikeClosure)
                .offset((i_0 - MIN_CHARLIKE) as isize) as *mut StgIntCharlikeClosure;

        let ref mut fresh6 = (*(c_0 as *mut StgClosure)).header.info;
        *fresh6 = (*ghc_hs_iface).Czh_con_info;
        (*c_0).data = i_0 as StgWord;
        i_0 += 1;
    }
}
