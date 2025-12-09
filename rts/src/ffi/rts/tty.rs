use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn __hscore_get_saved_termios(fd: c_int) -> *mut c_void {
    sys! {
        __hscore_get_saved_termios(fd)
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn __hscore_set_saved_termios(fd: c_int, ts: *mut c_void) {
    sys! {
        __hscore_set_saved_termios(fd, ts)
    }
}
