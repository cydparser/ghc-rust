use crate::capability::Capability;
use crate::ffi::hs_ffi::{HsBool, HsStablePtr};
use crate::ffi::rts::storage::closures::{_StgMutArrPtrs, StgClosure};
use crate::ffi::rts::storage::tso::{StgTSO, StgThreadID};
use crate::ffi::stg::W_;
use crate::ffi::stg::regs::StgRegTable;
use crate::ffi::stg::types::StgPtr;
use crate::prelude::*;

use libc::pid_t;

#[cfg(test)]
mod tests;

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn createThread(cap: *mut Capability, stack_size: W_) -> *mut StgTSO {
    sys! {
        createThread(cap as * mut sys::Capability, stack_size).cast()
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn createGenThread(
    cap: *mut Capability,
    stack_size: W_,
    closure: *mut StgClosure,
) -> *mut StgTSO {
    sys! {
        createGenThread(cap as * mut sys::Capability, stack_size, closure as * mut
        sys::StgClosure).cast()
    }
}

#[ffi(compiler, libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn suspendThread(arg1: *mut StgRegTable, interruptible: bool) -> *mut c_void {
    sys! {
        suspendThread(arg1 as * mut sys::StgRegTable, interruptible)
    }
}

#[ffi(compiler, libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn resumeThread(arg1: *mut c_void) -> *mut StgRegTable {
    sys! {
        resumeThread(arg1).cast()
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn eq_thread(tso1: StgPtr, tso2: StgPtr) -> bool {
    sys! {
        eq_thread(tso1, tso2)
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn cmp_thread(tso1: StgPtr, tso2: StgPtr) -> c_int {
    sys! {
        cmp_thread(tso1, tso2)
    }
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getThreadId(tso: StgPtr) -> StgThreadID {
    sys! {
        rts_getThreadId(tso)
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_enableThreadAllocationLimit(tso: StgPtr) {
    sys! {
        rts_enableThreadAllocationLimit(tso)
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_disableThreadAllocationLimit(tso: StgPtr) {
    sys! {
        rts_disableThreadAllocationLimit(tso)
    }
}

#[ffi(compiler, ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn listThreads(cap: *mut Capability) -> *mut _StgMutArrPtrs {
    sys! {
        listThreads(cap as * mut sys::Capability).cast()
    }
}

#[ffi(libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn forkProcess(entry: *mut HsStablePtr) -> pid_t {
    sys! {
        forkProcess(entry)
    }
}

#[ffi(ghc_lib, libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rtsSupportsBoundThreads() -> HsBool {
    sys! {
        rtsSupportsBoundThreads()
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut n_capabilities: u32 = 0;

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
pub static mut enabled_capabilities: u32 = 0;

// #[ffi(compiler)]
// #[unsafe(no_mangle)]
// TODO(rust): pub static mut MainCapability: Capability = todo!();

#[ffi(compiler, ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn setNumCapabilities(new_: u32) {
    sys! {
        setNumCapabilities(new_)
    }
}
