use crate::prelude::*;
use crate::rts::libdw::LibdwSession;

#[cfg(test)]
mod tests;

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_libdwPoolTake"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn libdwPoolTake() -> *mut LibdwSession {
    #[cfg(feature = "sys")]
    unsafe {
        sys::libdwPoolTake() as *mut LibdwSession
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("libdwPoolTake")
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_libdwPoolRelease"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn libdwPoolRelease(sess: *mut LibdwSession) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::libdwPoolRelease(sess as *mut sys::LibdwSession)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("libdwPoolRelease")
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_libdwPoolClear"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn libdwPoolClear() {
    #[cfg(feature = "sys")]
    unsafe {
        sys::libdwPoolClear()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("libdwPoolClear")
}
