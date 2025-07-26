use crate::prelude::*;
use crate::rts::storage::tso::StgTSO;

pub mod adjustor;
pub mod block_signals;
pub mod capability;
pub mod config;
pub mod constants;
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
pub mod profiling;
pub mod signals;
pub mod spin_lock;
pub mod stable_name;
pub mod stable_ptr;
pub mod static_ptr_table;
pub mod threads;
pub mod ticky;
pub mod time;
pub mod timer;
pub mod tty;
pub mod types;
pub mod utils;

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

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn _assertFail(filename: *const c_char, linenum: c_uint) /* -> ! */
{
    unsafe { sys::_assertFail(filename, linenum) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn _warnFail(filename: *const c_char, linenum: c_uint) /* -> ! */
{
    unsafe { sys::_warnFail(filename, linenum) }
}

static mut prog_argv: *mut *mut c_char = null_mut();

static mut prog_argc: c_int = 0;

static mut prog_name: *mut c_char = null_mut();

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_reportStackOverflow"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn reportStackOverflow(tso: *mut StgTSO) {
    unsafe { sys::reportStackOverflow(tso as *mut sys::StgTSO) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_reportHeapOverflow"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn reportHeapOverflow() {
    unsafe { sys::reportHeapOverflow() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_exit"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn stg_exit(n: c_int) /* -> ! */
{
    unsafe { sys::stg_exit(n) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_sig_install"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn stg_sig_install(arg1: c_int, arg2: c_int, arg3: *mut c_void) -> c_int {
    unsafe { sys::stg_sig_install(arg1, arg2, arg3) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_isProfiled"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_isProfiled() -> c_int {
    unsafe { sys::rts_isProfiled() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_isDynamic"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_isDynamic() -> c_int {
    unsafe { sys::rts_isDynamic() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_isThreaded"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_isThreaded() -> c_int {
    unsafe { sys::rts_isThreaded() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_isDebugged"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_isDebugged() -> c_int {
    unsafe { sys::rts_isDebugged() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_isTracing"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_isTracing() -> c_int {
    unsafe { sys::rts_isTracing() }
}
