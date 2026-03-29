use crate::prelude::*;
use crate::rts_utils::{stgFree, stgMallocBytes};

unsafe fn getProgEnvv(mut out_envc: *mut i32, mut out_envv: *mut *mut *mut c_char) {
    let mut envc: i32 = 0;
    let mut i: i32 = 0;
    let mut env = null_mut::<c_char>();
    let mut envp = null_mut::<c_char>();
    let mut envv = null_mut::<*mut c_char>();
    env = GetEnvironmentStrings() as *mut c_char;
    envc = 0;
    envp = env;

    while *envp as i32 != 0 {
        envc += 1;
        envp = envp.offset(strlen(envp).wrapping_add(1 as usize) as isize);
    }

    envv = stgMallocBytes(
        (size_of::<*mut c_char>() as usize).wrapping_mul((envc + 1 as i32) as usize),
        c"getProgEnvv".as_ptr(),
    ) as *mut *mut c_char;

    i = 0;
    envp = env;

    while *envp as i32 != 0 {
        let ref mut fresh28 = *envv.offset(i as isize);
        *fresh28 = envp;
        i += 1;
        envp = envp.offset(strlen(envp).wrapping_add(1 as usize) as isize);
    }

    let ref mut fresh29 = *envv.offset(envc as isize);
    *fresh29 = env;
    *out_envc = envc;
    *out_envv = envv;
}

unsafe fn freeProgEnvv(mut envc: i32, mut envv: *mut *mut c_char) {
    FreeEnvironmentStringsA(*envv.offset(envc as isize) as LPCH);
    stgFree(envv as *mut c_void);
}
