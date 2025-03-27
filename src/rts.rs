use crate::hs_ffi;
use crate::rts::prof::ccs;
use crate::rts::prof::heap;
use crate::rts::prof::ldv;
use crate::rts::storage::block;
use crate::rts::storage::closure_macros;
use crate::rts::storage::closure_types;
use crate::rts::storage::closures;
use crate::rts::storage::fun_types;
use crate::rts::storage::gc;
use crate::rts::storage::heap;
use crate::rts::storage::info_tables;
use crate::rts::storage::m_block;
use crate::rts::storage::tso;
use crate::rts_api;
use crate::stg;
use crate::stg::misc_closures;
use crate::stg::types;
use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use libc::{clockid_t, pid_t, pthread_cond_t, pthread_key_t, pthread_mutex_t, pthread_t};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
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
pub unsafe extern "C" fn _assertFail(
    filename: *const ::core::ffi::c_char,
    linenum: ::core::ffi::c_uint,
) -> ! {
    unsafe { transmute(sys::_assertFail(&filename.into(), linenum.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn _warnFail(filename: *const ::core::ffi::c_char, linenum: ::core::ffi::c_uint) {
    unsafe { transmute(sys::_warnFail(&filename.into(), linenum.into())) }
}

static mut prog_argv: *mut *mut ::core::ffi::c_char = sys::prog_argv;

static mut prog_argc: ::core::ffi::c_int = sys::prog_argc;

static mut prog_name: *mut ::core::ffi::c_char = sys::prog_name;

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn reportStackOverflow(tso: *mut StgTSO) {
    unsafe { transmute(sys::reportStackOverflow(&mut tso.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn reportHeapOverflow() {
    unsafe { transmute(sys::reportHeapOverflow()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn stg_exit(n: ::core::ffi::c_int) -> ! {
    unsafe { transmute(sys::stg_exit(n.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn stg_sig_install(
    arg1: ::core::ffi::c_int,
    arg2: ::core::ffi::c_int,
    arg3: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        transmute(sys::stg_sig_install(
            arg1.into(),
            arg2.into(),
            &mut arg3.into(),
        ))
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_isProfiled() -> ::core::ffi::c_int {
    unsafe { transmute(sys::rts_isProfiled()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_isDynamic() -> ::core::ffi::c_int {
    unsafe { transmute(sys::rts_isDynamic()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_isThreaded() -> ::core::ffi::c_int {
    unsafe { transmute(sys::rts_isThreaded()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_isDebugged() -> ::core::ffi::c_int {
    unsafe { transmute(sys::rts_isDebugged()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_isTracing() -> ::core::ffi::c_int {
    unsafe { transmute(sys::rts_isTracing()) }
}
