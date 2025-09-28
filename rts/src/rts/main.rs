use crate::prelude::*;
use crate::rts::storage::closures::StgClosure;
use crate::rts_api::RtsConfig;

/// - GHC_PLACES: {driver, testsuite, utils}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_main"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_main(
    argc: c_int,
    argv: *mut *mut c_char,
    main_closure: *mut StgClosure,
    rts_config: RtsConfig,
) -> ! {
    unsafe {
        sys::hs_main(
            argc,
            argv,
            main_closure as *mut sys::StgClosure,
            transmute(rts_config),
        )
    }
}
