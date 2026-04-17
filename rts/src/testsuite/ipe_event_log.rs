use crate::ffi::rts::ipe::registerInfoProvList;
use crate::hs_ffi::{hs_exit, hs_init};
use crate::prelude::*;
use crate::rts_api::{rts_lock, rts_unlock};

unsafe fn main_0(mut argc: i32, mut argv: *mut *mut c_char) -> i32 {
    hs_init(&raw mut argc, &raw mut argv);

    let mut cap = rts_lock();
    let mut list1 = makeAnyProvEntries(cap, 0, 10);
    let mut list2 = makeAnyProvEntries(cap, 0, 10);
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
            (args_ptrs.len() - 1) as i32,
            args_ptrs.as_mut_ptr() as *mut *mut c_char,
        ) as i32)
    }
}
