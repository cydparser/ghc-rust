use crate::ffi::rts::messages::{barf, errorBelch};
use crate::ffi::rts::types::StgClosure;
use crate::ffi::rts::{EXIT_HEAPOVERFLOW, EXIT_INTERRUPTED, EXIT_KILLED};
use crate::ffi::rts_api::{
    HaskellObj, NoStatus, RtsConfig, hs_init_ghc, rts_evalLazyIO, rts_getSchedStatus, rts_lock,
    rts_unlock, shutdownHaskellAndExit,
};
use crate::interpreter::{interp_shutdown, interp_startup};
use crate::prelude::*;

#[ffi(compiler, driver, testsuite, utils)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hs_main(
    mut argc: c_int,
    mut argv: *mut *mut c_char,
    mut main_closure: *mut StgClosure,
    mut rts_config: RtsConfig,
) -> ! {
    let mut exit_status: i32 = 0;
    let mut status = NoStatus;
    hs_init_ghc(&raw mut argc, &raw mut argv, rts_config);
    interp_startup();

    let mut cap = rts_lock();
    rts_evalLazyIO(
        &raw mut cap,
        main_closure as HaskellObj,
        null_mut::<HaskellObj>(),
    );
    status = rts_getSchedStatus(cap);
    rts_unlock(cap);

    match status as u32 {
        2 => {
            errorBelch(c"main thread exited (uncaught exception)".as_ptr());
            exit_status = EXIT_KILLED;
        }
        3 => {
            errorBelch(c"interrupted".as_ptr());
            exit_status = EXIT_INTERRUPTED;
        }
        4 => {
            exit_status = EXIT_HEAPOVERFLOW;
        }
        1 => {
            exit_status = EXIT_SUCCESS;
        }
        _ => {
            barf(c"main thread completed with invalid status".as_ptr());
        }
    }

    interp_shutdown();
    shutdownHaskellAndExit(exit_status, 0);
}
