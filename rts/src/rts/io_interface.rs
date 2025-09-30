use crate::prelude::*;

#[cfg(test)]
mod tests;

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_setIOManagerControlFd"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn setIOManagerControlFd(cap_no: u32, fd: c_int) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::setIOManagerControlFd(cap_no, fd)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("setIOManagerControlFd")
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_setTimerManagerControlFd"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn setTimerManagerControlFd(fd: c_int) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::setTimerManagerControlFd(fd)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("setTimerManagerControlFd")
}

/// - GHC_PLACES: {libraries, testsuite}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_setIOManagerWakeupFd"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn setIOManagerWakeupFd(fd: c_int) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::setIOManagerWakeupFd(fd)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("setIOManagerWakeupFd")
}
