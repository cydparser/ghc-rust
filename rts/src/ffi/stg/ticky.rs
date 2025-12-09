use crate::ffi::stg::types::StgInt;
use crate::prelude::*;

#[cfg(test)]
mod tests;

pub(crate) const TICKY_BIN_COUNT: u32 = 9;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut ENT_STATIC_THK_SINGLE_ctr: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut ENT_DYN_THK_SINGLE_ctr: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut ENT_STATIC_THK_MANY_ctr: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut ENT_DYN_THK_MANY_ctr: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut ENT_STATIC_FUN_DIRECT_ctr: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut ENT_DYN_FUN_DIRECT_ctr: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut ENT_DYN_CON_ctr: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut ENT_LNE_ctr: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut UNKNOWN_CALL_ctr: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut VERY_SLOW_CALL_ctr: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut KNOWN_CALL_ctr: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut KNOWN_CALL_TOO_FEW_ARGS_ctr: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut KNOWN_CALL_EXTRA_ARGS_ctr: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut UPDF_OMITTED_ctr: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut UPDF_PUSHED_ctr: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut ALLOC_HEAP_ctr: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut ALLOC_HEAP_tot: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut HEAP_CHK_ctr: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut STK_CHK_ctr: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut ALLOC_FUN_ctr: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut ALLOC_FUN_gds: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut UPD_CAF_BH_UPDATABLE_ctr: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut UPD_CAF_BH_SINGLE_ENTRY_ctr: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut ALLOC_UP_THK_ctr: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut ALLOC_SE_THK_ctr: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut ALLOC_THK_gds: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut ALLOC_THK_slp: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut ALLOC_CON_ctr: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut ALLOC_CON_gds: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut ALLOC_PRIM_ctr: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut ALLOC_PRIM_adm: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut ALLOC_PRIM_gds: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut ALLOC_PRIM_slp: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut ALLOC_PAP_ctr: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut ALLOC_PAP_gds: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut ALLOC_PAP_slp: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut RET_NEW_ctr: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut RET_OLD_ctr: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut RET_UNBOXED_TUP_ctr: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut TAG_UNTAGGED_pred: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut TAG_UNTAGGED_miss: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut TAG_TAGGED_pred: StgInt = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut RET_NEW_hst: [StgInt; 9usize] = [0; _];

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut RET_OLD_hst: [StgInt; 9usize] = [0; _];

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut RET_UNBOXED_TUP_hst: [StgInt; 9usize] = [0; _];
