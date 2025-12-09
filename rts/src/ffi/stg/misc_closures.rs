use crate::ffi::rts::storage::closures::{StgHeader, StgIntCharlikeClosure};
use crate::ffi::rts::storage::info_tables::{
    StgClosureInfo, StgFunInfoExtraRev_, StgFunInfoExtraRev___bindgen_ty_1, StgFunInfoTable,
    StgInfoTable, StgInfoTable_,
};
use crate::ffi::stg::types::StgFunPtr;
use crate::prelude::*;

#[cfg(test)]
mod tests;

const TODO_StgInfoTable: StgInfoTable = StgInfoTable_ {
    layout: StgClosureInfo { bitmap: 0 },
    type_: 0,
    srt: 0,
    code: __IncompleteArrayField::new(),
};

#[ffi(compiler, ghc_lib, testsuite)]
#[unsafe(no_mangle)]
pub static stg_upd_frame_info: StgInfoTable = TODO_StgInfoTable;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static stg_bh_upd_frame_info: StgInfoTable = TODO_StgInfoTable;

#[ffi(compiler, ghc_lib)]
#[unsafe(no_mangle)]
pub static stg_catch_frame_info: StgInfoTable = TODO_StgInfoTable;

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
pub static stg_catch_retry_frame_info: StgInfoTable = TODO_StgInfoTable;

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
pub static stg_atomically_frame_info: StgInfoTable = TODO_StgInfoTable;

const TODO_StgFunInfoTable: StgFunInfoTable = StgFunInfoTable {
    f: StgFunInfoExtraRev_ {
        slow_apply_offset: 0,
        b: StgFunInfoExtraRev___bindgen_ty_1 { bitmap: 0 },
        fun_type: 0,
        arity: 0,
    },
    i: TODO_StgInfoTable,
};

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
pub static stg_catch_stm_frame_info: StgInfoTable = TODO_StgInfoTable;

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
pub static stg_stack_underflow_frame_d_info: StgInfoTable = TODO_StgInfoTable;

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn stg_ctoi_t() -> StgFunPtr {
    sys! {
        stg_ctoi_t()
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static stg_ctoi_t3_info: StgInfoTable = TODO_StgInfoTable;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static stg_primcall_info: StgInfoTable = TODO_StgInfoTable;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static stg_IND_STATIC_info: StgInfoTable = TODO_StgInfoTable;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static __stg_EAGER_BLACKHOLE_info: StgInfoTable = TODO_StgInfoTable;

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
pub static stg_BCO_info: StgFunInfoTable = TODO_StgFunInfoTable;

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
pub static stg_STACK_info: StgInfoTable = TODO_StgInfoTable;

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
pub static stg_ARR_WORDS_info: StgInfoTable = TODO_StgInfoTable;

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
pub static stg_MUT_ARR_PTRS_FROZEN_CLEAN_info: StgInfoTable = TODO_StgInfoTable;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static stg_SRT_1_info: StgInfoTable = TODO_StgInfoTable;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static stg_SRT_16_info: StgInfoTable = TODO_StgInfoTable;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut stg_INTLIKE_closure: [StgIntCharlikeClosure; 272] = [const {
    StgIntCharlikeClosure {
        header: StgHeader { info: null() },
        data: 0,
    }
}; _];

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static stg_unpack_cstring_info: StgInfoTable = TODO_StgInfoTable;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static stg_unpack_cstring_utf8_info: StgInfoTable = TODO_StgInfoTable;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static stg_ap_pp_info: StgInfoTable = TODO_StgInfoTable;

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn stg_ap_n_fast() -> StgFunPtr {
    sys! {
        stg_ap_n_fast()
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn stg_ap_p_fast() -> StgFunPtr {
    sys! {
        stg_ap_p_fast()
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn stg_ap_pp_fast() -> StgFunPtr {
    sys! {
        stg_ap_pp_fast()
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn stg_ap_ppp_fast() -> StgFunPtr {
    sys! {
        stg_ap_ppp_fast()
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn stg_gc_noregs() -> StgFunPtr {
    sys! {
        stg_gc_noregs()
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
pub static stg_ret_p_info: StgInfoTable = TODO_StgInfoTable;

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
pub static stg_ret_n_info: StgInfoTable = TODO_StgInfoTable;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static stg_ret_t_info: StgInfoTable = TODO_StgInfoTable;

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn __stg_gc_enter_1() -> StgFunPtr {
    sys! {
        __stg_gc_enter_1()
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn stg_gc_unpt_r1() -> StgFunPtr {
    sys! {
        stg_gc_unpt_r1()
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn stg_gc_unbx_r1() -> StgFunPtr {
    sys! {
        stg_gc_unbx_r1()
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn stg_gc_f1() -> StgFunPtr {
    sys! {
        stg_gc_f1()
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn stg_gc_d1() -> StgFunPtr {
    sys! {
        stg_gc_d1()
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn stg_gc_l1() -> StgFunPtr {
    sys! {
        stg_gc_l1()
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn stg_gc_pp() -> StgFunPtr {
    sys! {
        stg_gc_pp()
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn stg_gc_ppp() -> StgFunPtr {
    sys! {
        stg_gc_ppp()
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn stg_gc_pppp() -> StgFunPtr {
    sys! {
        stg_gc_pppp()
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn __stg_gc_fun() -> StgFunPtr {
    sys! {
        __stg_gc_fun()
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
pub static stg_stop_thread_info: StgInfoTable = TODO_StgInfoTable;

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn StgReturn() -> StgFunPtr {
    sys! {
        StgReturn()
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn stg_paniczh() -> StgFunPtr {
    sys! {
        stg_paniczh()
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn stg_absentErrorzh() -> StgFunPtr {
    sys! {
        stg_absentErrorzh()
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn stg_getThreadAllocationCounterzh() -> StgFunPtr {
    sys! {
        stg_getThreadAllocationCounterzh()
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn stg_getOtherThreadAllocationCounterzh() -> StgFunPtr {
    sys! {
        stg_getOtherThreadAllocationCounterzh()
    }
}
