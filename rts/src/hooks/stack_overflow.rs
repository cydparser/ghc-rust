use crate::ffi::hs_ffi::HsBool;
use crate::ffi::rts::messages::errorBelch;
use crate::ffi::rts_api::RtsOptsAll;
use crate::ffi::stg::W_;
use crate::prelude::*;
use crate::rts_flags::rtsConfig;

unsafe fn StackOverflowHook(mut stack_size: W_) {
    errorBelch(
        b"Stack space overflow: current size %llu bytes.\0" as *const u8 as *const c_char,
        stack_size,
    );

    if rtsConfig.rts_opts_suggestions == r#true as HsBool {
        if rtsConfig.rts_opts_enabled as c_uint == RtsOptsAll as c_int as c_uint {
            errorBelch(b"Use `+RTS -Ksize -RTS' to increase it.\0" as *const u8 as *const c_char);
        } else {
            errorBelch(
                b"Relink with -rtsopts and use `+RTS -Ksize -RTS' to increase it.\0" as *const u8
                    as *const c_char,
            );
        }
    }
}
