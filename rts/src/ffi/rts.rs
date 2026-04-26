pub use crate::adjustor::{createAdjustor, freeHaskellFunctionPtr};
pub use crate::posix::signals::stg_sig_install;
use crate::prelude::*;
pub use crate::rts_messages::{_assertFail, rtsMemcpyRangeOverlap, rtsOutOfBoundsAccess};
pub use crate::rts_startup::stg_exit;
pub use crate::rts_utils::{
    reportHeapOverflow, reportStackOverflow, rts_isDebugged, rts_isDynamic, rts_isProfiled,
    rts_isThreaded, rts_isTracing,
};

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
pub mod spin_lock;
pub mod stable_name;
pub mod stable_ptr;
pub mod static_ptr_table;
pub mod storage;
pub mod threads;
pub mod ticky;
pub mod time;
pub mod timer;
pub mod tty;
pub mod utils;

#[cfg(test)]
mod tests;

#[ffi(compiler)]
pub const IN_STG_CODE: u32 = 0;

#[ffi(ghc_lib, libraries)]
pub const _REENTRANT: u32 = 1;

pub(crate) const ASSERTS_ENABLED: u32 = 1;

#[ffi(ghc_lib)]
pub const EXIT_INTERNAL_ERROR: i32 = 254;

pub(crate) const EXIT_DEADLOCK: i32 = 253;

pub(crate) const EXIT_INTERRUPTED: i32 = 252;

pub(crate) const EXIT_HEAPOVERFLOW: i32 = 251;

pub(crate) const EXIT_KILLED: i32 = 250;

pub(crate) const DEBUG_IS_ON: u32 = 1;
