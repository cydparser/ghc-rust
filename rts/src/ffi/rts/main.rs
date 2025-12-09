use crate::ffi::rts::storage::closures::StgClosure;
use crate::ffi::rts_api::RtsConfig;
use crate::prelude::*;

#[ffi(compiler, driver, testsuite, utils)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hs_main(
    argc: c_int,
    argv: *mut *mut c_char,
    main_closure: *mut StgClosure,
    rts_config: RtsConfig,
) -> ! {
    before_exit("hs_main");
    sys! {
        hs_main(argc, argv, main_closure as * mut sys::StgClosure, transmute(rts_config))
    }
}
