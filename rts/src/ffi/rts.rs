use crate::ffi::rts::storage::tso::StgTSO;
use crate::prelude::*;

pub mod adjustor;
pub mod block_signals;
pub mod config;
pub mod constants;
pub mod event_log_writer;
pub mod exec_page;
pub mod file_lock;
pub mod flags;
pub mod foreign_exports;
pub mod get_time;
pub mod globals;
pub mod hpc;
pub mod io_interface;
pub mod ipe;
pub mod libdw;
pub mod libdw_pool;
pub mod linker;
pub mod main;
pub mod messages;
pub mod non_moving;
pub mod os_threads;
pub mod parallel;
pub mod prim_float;
pub mod prof;
pub mod profiling;
pub mod rts_to_hs_iface;
pub mod signals;
pub mod stable_name;
pub mod stable_ptr;
pub mod static_ptr_table;
pub mod storage;
pub mod task;
pub mod threads;
pub mod ticky;
pub mod time;
pub mod timer;
pub mod tsan_utils;
pub mod tty;
pub mod utils;

#[cfg(test)]
mod tests;

#[ffi(compiler)]
pub const IN_STG_CODE: u32 = 0;

#[ffi(ghc_lib, libraries)]
pub const _REENTRANT: u32 = 1;

#[ffi(ghc_lib)]
pub const EXIT_INTERNAL_ERROR: u32 = 254;

pub(crate) const EXIT_DEADLOCK: u32 = 253;

pub(crate) const EXIT_INTERRUPTED: u32 = 252;

pub(crate) const EXIT_HEAPOVERFLOW: u32 = 251;

pub(crate) const EXIT_KILLED: u32 = 250;

pub(crate) const DEBUG_IS_ON: u32 = 0;

#[ffi(utils)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _assertFail(filename: *const c_char, linenum: c_uint) -> ! {
    before_exit("_assertFail");
    sys! {
        _assertFail(filename, linenum)
    }
}

#[ffi(compiler, ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn reportStackOverflow(tso: *mut StgTSO) {
    sys! {
        reportStackOverflow(tso as * mut sys::StgTSO)
    }
}

#[ffi(compiler, ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn reportHeapOverflow() {
    sys! {
        reportHeapOverflow()
    }
}

#[ffi(ghc_lib, utils)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stg_exit(n: c_int) -> ! {
    before_exit("stg_exit");
    sys! {
        stg_exit(n)
    }
}

#[ffi(ghc_lib, libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn stg_sig_install(arg1: c_int, arg2: c_int, arg3: *mut c_void) -> c_int {
    sys! {
        stg_sig_install(arg1, arg2, arg3)
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_isProfiled() -> c_int {
    sys! {
        rts_isProfiled()
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_isDynamic() -> c_int {
    sys! {
        rts_isDynamic()
    }
}

#[ffi(compiler, ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_isThreaded() -> c_int {
    sys! {
        rts_isThreaded()
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_isDebugged() -> c_int {
    sys! {
        rts_isDebugged()
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_isTracing() -> c_int {
    sys! {
        rts_isTracing()
    }
}
