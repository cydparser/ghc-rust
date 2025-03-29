#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use core::ffi;
use std::mem::transmute;

#[cfg(feature = "tracing")]
use tracing::instrument;
pub mod types;

pub mod time;

pub mod config;

pub mod constants;

pub mod flags;

pub mod os_threads;

pub mod spin_lock;

pub mod messages;

pub mod threads;

pub mod non_moving;

pub mod foreign_exports;

pub mod exec_page;

pub mod parallel;

pub mod signals;

pub mod block_signals;

pub mod hpc;

pub mod adjustor;

pub mod file_lock;

pub mod get_time;

pub mod globals;

pub mod io_interface;

pub mod linker;

pub mod ticky;

pub mod timer;

pub mod stable_ptr;

pub mod stable_name;

pub mod tty;

pub mod utils;

pub mod prim_float;

pub mod main;

pub mod profiling;

pub mod ipe;

pub mod static_ptr_table;

pub mod libdw;

pub mod libdw_pool;

#[cfg(test)]
mod tests;

pub const IN_STG_CODE: u32 = 0;

pub const _REENTRANT: u32 = 1;

pub(crate) const FMT_SizeT: &[u8; 3] = b"zu\0";

pub(crate) const FMT_HexSizeT: &[u8; 3] = b"zx\0";

pub const EXIT_INTERNAL_ERROR: u32 = 254;

pub(crate) const EXIT_DEADLOCK: u32 = 253;

pub(crate) const EXIT_INTERRUPTED: u32 = 252;

pub(crate) const EXIT_HEAPOVERFLOW: u32 = 251;

pub(crate) const EXIT_KILLED: u32 = 250;

pub(crate) const DEBUG_IS_ON: u32 = 0;

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn _assertFail(filename: *const ffi::c_char, linenum: ffi::c_uint) {
    unsafe { sys::_assertFail(filename, linenum) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn _warnFail(filename: *const ffi::c_char, linenum: ffi::c_uint) {
    unsafe { sys::_warnFail(filename, linenum) }
}

static mut prog_argv: *mut *mut ffi::c_char = unsafe { sys::prog_argv };

static mut prog_argc: ffi::c_int = unsafe { sys::prog_argc };

static mut prog_name: *mut ffi::c_char = unsafe { sys::prog_name };

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn reportStackOverflow(tso: *mut types::StgTSO) {
    unsafe { sys::reportStackOverflow(&mut tso.into()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn reportHeapOverflow() {
    unsafe { transmute(sys::reportHeapOverflow()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn stg_exit(n: ffi::c_int) {
    unsafe { sys::stg_exit(n) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn stg_sig_install(
    arg1: ffi::c_int,
    arg2: ffi::c_int,
    arg3: *mut ffi::c_void,
) -> ffi::c_int {
    unsafe { sys::stg_sig_install(arg1, arg2, arg3) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_isProfiled() -> ffi::c_int {
    unsafe { sys::rts_isProfiled() }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_isDynamic() -> ffi::c_int {
    unsafe { sys::rts_isDynamic() }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_isThreaded() -> ffi::c_int {
    unsafe { sys::rts_isThreaded() }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_isDebugged() -> ffi::c_int {
    unsafe { sys::rts_isDebugged() }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_isTracing() -> ffi::c_int {
    unsafe { sys::rts_isTracing() }
}
