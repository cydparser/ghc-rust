use crate::prelude::*;

#[cfg(test)]
mod tests;

/// - GHC_PLACES: {libraries}
#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust___hscore_get_saved_termios")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn __hscore_get_saved_termios(fd: c_int) -> *mut c_void {
    unsafe { sys::__hscore_get_saved_termios(fd) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust___hscore_set_saved_termios")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn __hscore_set_saved_termios(fd: c_int, ts: *mut c_void) {
    unsafe { sys::__hscore_set_saved_termios(fd, ts) }
}
