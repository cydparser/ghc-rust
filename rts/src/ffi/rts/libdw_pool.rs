use crate::ffi::rts::libdw::LibdwSession;
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn libdwPoolTake() -> *mut LibdwSession {
    sys! {
        libdwPoolTake().cast()
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn libdwPoolRelease(sess: *mut LibdwSession) {
    sys! {
        libdwPoolRelease(sess as * mut sys::LibdwSession)
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn libdwPoolClear() {
    sys! {
        libdwPoolClear()
    }
}
