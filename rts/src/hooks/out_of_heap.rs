use crate::ffi::hs_ffi::HsBool;
use crate::ffi::rts::messages::errorBelch;
use crate::ffi::rts_api::RtsOptsAll;
use crate::ffi::stg::W_;
use crate::prelude::*;
use crate::rts_flags::rtsConfig;

unsafe fn OutOfHeapHook(mut request_size: W_, mut heap_size: W_) {
    if heap_size > 0 as W_ {
        errorBelch(b"Heap exhausted;\0" as *const u8 as *const c_char);

        errorBelch(
            b"Current maximum heap size is %llu bytes (%llu MB).\0" as *const u8 as *const c_char,
            heap_size,
            heap_size.wrapping_div((1024 as c_int * 1024 as c_int) as W_),
        );

        if rtsConfig.rts_opts_suggestions == r#true as HsBool {
            if rtsConfig.rts_opts_enabled as c_uint == RtsOptsAll as c_int as c_uint {
                errorBelch(b"Use `+RTS -M<size>' to increase it.\0" as *const u8 as *const c_char);
            } else {
                errorBelch(
                    b"Relink with -rtsopts and use `+RTS -M<size>' to increase it.\0" as *const u8
                        as *const c_char,
                );
            }
        }
    } else {
        errorBelch(b"Out of memory\n\0" as *const u8 as *const c_char);
    };
}
