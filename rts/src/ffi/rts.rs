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

pub(crate) const IN_STG_CODE: u32 = 0;

/// - GHC_PLACES: {libraries}
pub const _REENTRANT: u32 = 1;

/// - GHC_PLACES: {libraries}
pub const EXIT_INTERNAL_ERROR: u32 = 254;

pub(crate) const EXIT_DEADLOCK: u32 = 253;

pub(crate) const EXIT_INTERRUPTED: u32 = 252;

pub(crate) const EXIT_HEAPOVERFLOW: u32 = 251;

pub(crate) const EXIT_KILLED: u32 = 250;

pub(crate) const DEBUG_IS_ON: u32 = 0;

/// - GHC_PLACES: {utils}
#[ffi]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _assertFail(filename: *const c_char, linenum: c_uint) -> ! {
    #[cfg(feature = "sys")]
    unsafe {
        sys::_assertFail(filename, linenum)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("_assertFail")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn reportStackOverflow(tso: *mut StgTSO) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::reportStackOverflow(tso as *mut sys::StgTSO)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("reportStackOverflow")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn reportHeapOverflow() {
    #[cfg(feature = "sys")]
    unsafe {
        sys::reportHeapOverflow()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("reportHeapOverflow")
}

/// - GHC_PLACES: {libraries, utils}
#[ffi]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stg_exit(n: c_int) -> ! {
    #[cfg(feature = "sys")]
    unsafe {
        sys::stg_exit(n)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("stg_exit")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn stg_sig_install(arg1: c_int, arg2: c_int, arg3: *mut c_void) -> c_int {
    #[cfg(feature = "sys")]
    unsafe {
        sys::stg_sig_install(arg1, arg2, arg3)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("stg_sig_install")
}

/// - GHC_PLACES: {compiler}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_isProfiled() -> c_int {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_isProfiled()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_isProfiled")
}

/// - GHC_PLACES: {compiler}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_isDynamic() -> c_int {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_isDynamic()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_isDynamic")
}

/// - GHC_PLACES: {compiler, libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_isThreaded() -> c_int {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_isThreaded()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_isThreaded")
}

/// - GHC_PLACES: {compiler}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_isDebugged() -> c_int {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_isDebugged()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_isDebugged")
}

/// - GHC_PLACES: {compiler}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_isTracing() -> c_int {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_isTracing()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_isTracing")
}
