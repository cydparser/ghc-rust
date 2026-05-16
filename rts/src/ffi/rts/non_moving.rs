use crate::prelude::*;
pub use crate::sm::non_moving_mark::{
    nonmoving_write_barrier_enabled, updateRemembSetPushClosure_, updateRemembSetPushThunk_,
};
use crate::stg::types::StgFunPtr;

#[cfg(test)]
mod tests;

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn stg_copyArray_barrier() -> StgFunPtr {
    sys! {
        stg_copyArray_barrier()
    }
}
