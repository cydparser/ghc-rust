use crate::ffi::hs_ffi::{HsInt, hs_exit, hs_init};
use crate::ffi::rts::ipe::{InfoProv_, InfoProvEnt_, lookupIPE, registerInfoProvList};
use crate::ffi::rts_api::{rts_lock, rts_mkInt, rts_unlock};
use crate::prelude::*;

unsafe fn main_0(mut argc: i32, mut argv: *mut *mut c_char) -> i32 {
    hs_init(&raw mut argc, &raw mut argv);

    let mut cap = rts_lock();
    let mut one = rts_mkInt(cap, 1);
    let mut list1 = makeAnyProvEntries(cap, 0, 10);
    let mut list2 = makeAnyProvEntries(cap, 0, 10);
    registerInfoProvList(list1);
    registerInfoProvList(list2);

    let mut ipe = InfoProvEnt_ {
        info: null::<StgInfoTable>(),
        prov: InfoProv_ {
            table_name: null::<c_char>(),
            closure_desc: 0,
            ty_desc: null::<c_char>(),
            label: null::<c_char>(),
            unit_id: null::<c_char>(),
            module: null::<c_char>(),
            src_file: null::<c_char>(),
            src_span: null::<c_char>(),
        },
    };

    lookupIPE(*(*list1).tables.offset(0), &raw mut ipe);
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
