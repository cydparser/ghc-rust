use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use libc::{clockid_t, pid_t, pthread_cond_t, pthread_key_t, pthread_mutex_t, pthread_t};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use std::mem::transmute;
#[cfg(feature = "tracing")]
use tracing::instrument;
#[cfg(test)]
mod tests;

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_main(
    argc: ::core::ffi::c_int,
    argv: *mut *mut ::core::ffi::c_char,
    main_closure: *mut StgClosure,
    rts_config: RtsConfig,
) -> ! {
    unsafe {
        transmute(sys::hs_main(
            argc.into(),
            &mut &mut argv.into(),
            &mut main_closure.into(),
            rts_config.into(),
        ))
    }
}
