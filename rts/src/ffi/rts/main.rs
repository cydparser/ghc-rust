use crate::prelude::*;
use crate::rts::storage::closures::StgClosure;
use crate::rts_api::RtsConfig;

/// - GHC_PLACES: {driver, testsuite, utils}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_main(
    argc: c_int,
    argv: *mut *mut c_char,
    main_closure: *mut StgClosure,
    rts_config: RtsConfig,
) -> ! {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_main(
            argc,
            argv,
            main_closure as *mut sys::StgClosure,
            transmute(rts_config),
        )
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_main")
}
