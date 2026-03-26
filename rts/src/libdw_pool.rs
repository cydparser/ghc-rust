use crate::ffi::rts::libdw::LibdwSession;
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[inline]
pub(crate) unsafe fn libdwPoolInit() {}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn libdwPoolTake() -> *mut LibdwSession {
    return null_mut::<LibdwSession>();
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn libdwPoolRelease(mut sess: *mut LibdwSession) {}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn libdwPoolClear() {}
