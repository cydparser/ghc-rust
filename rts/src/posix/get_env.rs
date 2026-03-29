use crate::prelude::*;

unsafe fn get_environ() -> *mut *mut c_char {
    return *_NSGetEnviron();
}

unsafe fn getProgEnvv(mut out_envc: *mut i32, mut out_envv: *mut *mut *mut c_char) {
    let mut envc: i32 = 0;
    let mut environ = get_environ();
    envc = 0;

    while !(*environ.offset(envc as isize)).is_null() {
        envc += 1;
    }

    *out_envc = envc;
    *out_envv = environ;
}

unsafe fn freeProgEnvv(mut envc: i32, mut envv: *mut *mut c_char) {}
