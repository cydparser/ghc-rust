use crate::ffi::hs_ffi::HsBool;
use crate::ffi::rts::messages::errorBelch;
use crate::ffi::rts_api::RtsOptsAll;
use crate::ffi::stg::W_;
use crate::prelude::*;
use crate::rts_flags::rtsConfig;

unsafe fn StackOverflowHook(mut stack_size: W_) {
    errorBelch(
        c"Stack space overflow: current size %llu bytes.".as_ptr(),
        stack_size,
    );

    if rtsConfig.rts_opts_suggestions == true {
        if rtsConfig.rts_opts_enabled as u32 == RtsOptsAll as i32 as u32 {
            errorBelch(c"Use `+RTS -Ksize -RTS' to increase it.".as_ptr());
        } else {
            errorBelch(c"Relink with -rtsopts and use `+RTS -Ksize -RTS' to increase it.".as_ptr());
        }
    }
}
