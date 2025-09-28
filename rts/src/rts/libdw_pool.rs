use crate::prelude::*;
use crate::rts::libdw::LibdwSession;

#[cfg(test)]
mod tests;

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_libdwPoolTake"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn libdwPoolTake() -> *mut LibdwSession {
    unsafe { sys::libdwPoolTake() as *mut LibdwSession }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_libdwPoolRelease"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn libdwPoolRelease(sess: *mut LibdwSession) {
    unsafe { sys::libdwPoolRelease(sess as *mut sys::LibdwSession) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_libdwPoolClear"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn libdwPoolClear() {
    unsafe { sys::libdwPoolClear() }
}
