use crate::prelude::*;

#[cfg(test)]
mod tests;

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn __hscore_get_saved_termios(fd: c_int) -> *mut c_void {
    #[cfg(feature = "sys")]
    unsafe {
        sys::__hscore_get_saved_termios(fd)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("__hscore_get_saved_termios")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn __hscore_set_saved_termios(fd: c_int, ts: *mut c_void) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::__hscore_set_saved_termios(fd, ts)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("__hscore_set_saved_termios")
}
