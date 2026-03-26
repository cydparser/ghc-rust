use crate::prelude::*;
use crate::rts_utils::{stgFree, stgMallocBytes};

unsafe fn getProgEnvv(mut out_envc: *mut c_int, mut out_envv: *mut *mut *mut c_char) {
    let mut envc: c_int = 0;
    let mut i: c_int = 0;
    let mut env = null_mut::<c_char>();
    let mut envp = null_mut::<c_char>();
    let mut envv = null_mut::<*mut c_char>();
    env = GetEnvironmentStrings() as *mut c_char;
    envc = 0 as c_int;
    envp = env;

    while *envp as c_int != 0 as c_int {
        envc += 1;
        envp = envp.offset(strlen(envp).wrapping_add(1 as size_t) as isize);
    }

    envv = stgMallocBytes(
        (size_of::<*mut c_char>() as size_t).wrapping_mul((envc + 1 as c_int) as size_t),
        b"getProgEnvv\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut *mut c_char;

    i = 0 as c_int;
    envp = env;

    while *envp as c_int != 0 as c_int {
        let ref mut fresh28 = *envv.offset(i as isize);
        *fresh28 = envp;
        i += 1;
        envp = envp.offset(strlen(envp).wrapping_add(1 as size_t) as isize);
    }

    let ref mut fresh29 = *envv.offset(envc as isize);
    *fresh29 = env;
    *out_envc = envc;
    *out_envv = envv;
}

unsafe fn freeProgEnvv(mut envc: c_int, mut envv: *mut *mut c_char) {
    FreeEnvironmentStringsA(*envv.offset(envc as isize) as LPCH);
    stgFree(envv as *mut c_void);
}
