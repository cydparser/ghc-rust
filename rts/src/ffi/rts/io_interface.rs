use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn setIOManagerControlFd(cap_no: u32, fd: c_int) {
    sys! {
        setIOManagerControlFd(cap_no, fd)
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn setTimerManagerControlFd(fd: c_int) {
    sys! {
        setTimerManagerControlFd(fd)
    }
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn setIOManagerWakeupFd(fd: c_int) {
    sys! {
        setIOManagerWakeupFd(fd)
    }
}
