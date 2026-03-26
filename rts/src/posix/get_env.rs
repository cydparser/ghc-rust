use crate::prelude::*;

unsafe fn get_environ() -> *mut *mut c_char {
    return *_NSGetEnviron();
}

unsafe fn getProgEnvv(mut out_envc: *mut c_int, mut out_envv: *mut *mut *mut c_char) {
    let mut envc: c_int = 0;
    let mut environ = get_environ();
    envc = 0 as c_int;

    while !(*environ.offset(envc as isize)).is_null() {
        envc += 1;
    }

    *out_envc = envc;
    *out_envv = environ;
}

unsafe fn freeProgEnvv(mut envc: c_int, mut envv: *mut *mut c_char) {}
