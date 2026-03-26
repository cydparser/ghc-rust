use crate::ffi::hs_ffi::{hs_exit, hs_init};
use crate::ffi::rts::ipe::registerInfoProvList;
use crate::ffi::rts_api::{rts_lock, rts_unlock};
use crate::prelude::*;

unsafe fn main_0(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    hs_init(&raw mut argc, &raw mut argv);

    let mut cap = rts_lock();
    let mut list1 = makeAnyProvEntries(cap, 0 as c_int, 10 as c_int);
    let mut list2 = makeAnyProvEntries(cap, 0 as c_int, 10 as c_int);
    registerInfoProvList(list1);
    registerInfoProvList(list2);
    dumpIPEToEventLog();
    rts_unlock(cap);
    hs_exit();

    return 0;
}

fn main() {
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args()
        .map(|arg| {
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();

    let mut args_ptrs: Vec<*mut c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut c_char)
        .chain(::core::iter::once(null_mut()))
        .collect();

    unsafe {
        ::std::process::exit(main_0(
            (args_ptrs.len() - 1) as c_int,
            args_ptrs.as_mut_ptr() as *mut *mut c_char,
        ) as i32)
    }
}
