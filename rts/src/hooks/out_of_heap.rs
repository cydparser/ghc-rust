use crate::ffi::rts::messages::errorBelch;
use crate::ffi::stg::W_;
use crate::hs_ffi::HsBool;
use crate::prelude::*;
use crate::rts_api::RtsOptsAll;
use crate::rts_flags::rtsConfig;

unsafe fn OutOfHeapHook(mut request_size: W_, mut heap_size: W_) {
    if heap_size > 0 {
        errorBelch(c"Heap exhausted;".as_ptr());

        errorBelch(
            c"Current maximum heap size is %llu bytes (%llu MB).".as_ptr(),
            heap_size,
            heap_size.wrapping_div((1024 as i32 * 1024 as i32) as W_),
        );

        if rtsConfig.rts_opts_suggestions == true {
            if rtsConfig.rts_opts_enabled as u32 == RtsOptsAll as i32 as u32 {
                errorBelch(c"Use `+RTS -M<size>' to increase it.".as_ptr());
            } else {
                errorBelch(
                    c"Relink with -rtsopts and use `+RTS -M<size>' to increase it.".as_ptr(),
                );
            }
        }
    } else {
        errorBelch(c"Out of memory\n".as_ptr());
    };
}
