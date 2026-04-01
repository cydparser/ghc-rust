use crate::prelude::*;

#[cfg(test)]
mod tests;

static mut saved_termios: [*mut c_void; 3] = [NULL, NULL, NULL];

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn __hscore_get_saved_termios(mut fd: c_int) -> *mut c_void {
    return if 0 <= fd
        && fd
            < (size_of::<[*mut c_void; 3]>() as usize)
                .wrapping_div(size_of::<*mut c_void>() as usize) as i32
    {
        saved_termios[fd as usize]
    } else {
        NULL
    };
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn __hscore_set_saved_termios(mut fd: c_int, mut ts: *mut c_void) {
    if 0 <= fd
        && fd
            < (size_of::<[*mut c_void; 3]>() as usize)
                .wrapping_div(size_of::<*mut c_void>() as usize) as i32
    {
        saved_termios[fd as usize] = ts;
    }
}

unsafe fn resetTerminalSettings() {
    let mut fd: i32 = 0;
    let mut sigset: sigset_t = 0;
    let mut old_sigset: sigset_t = 0;
    sigset = 0;
    sigset |= __sigbits(22) as sigset_t;
    sigprocmask(SIG_BLOCK, &raw mut sigset, &raw mut old_sigset);
    fd = 0;

    while fd <= 2 {
        let mut ts = __hscore_get_saved_termios(fd) as *mut termios;

        if !ts.is_null() {
            tcsetattr(fd, TCSANOW, ts);
        }

        fd += 1;
    }

    sigprocmask(SIG_SETMASK, &raw mut old_sigset, null_mut::<sigset_t>());
}
