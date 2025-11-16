use crate::prelude::*;

#[cfg(test)]
mod tests;

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
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
#[ffi]
#[unsafe(no_mangle)]
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
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn setIOManagerWakeupFd(fd: c_int) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::setIOManagerWakeupFd(fd)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("setIOManagerWakeupFd")
}
