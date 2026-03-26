use crate::ffi::hs_ffi::hs_exit;
use crate::ffi::rts::storage::m_block::{freeMBlocks, getMBlocks, releaseFreeMemory};
use crate::ffi::rts_api::{RtsOptsAll, defaultRtsConfig, hs_init_ghc};
use crate::prelude::*;

static mut MAXALLOC: c_int = 16 as c_int;

static mut ARRSIZE: c_int = 64 as c_int;

static mut LOOPS: c_int = 1000 as c_int;

static mut SEED: c_int = 0xf00f00 as c_int;

unsafe fn main_0(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut b: c_int = 0;
    let vla = ARRSIZE as usize;
    let mut a: Vec<*mut c_void> = ::std::vec::from_elem(null_mut::<c_void>(), vla);
    let vla_0 = ARRSIZE as usize;
    let mut sizes: Vec<uint32_t> = ::std::vec::from_elem(0, vla_0);
    srand(SEED as c_uint);

    let mut conf = defaultRtsConfig;
    conf.rts_opts_enabled = RtsOptsAll;
    hs_init_ghc(&raw mut argc, &raw mut argv, conf);
    i = 0 as c_int;

    while i < LOOPS {
        j = 0 as c_int;

        while j < ARRSIZE {
            if i > 0 as c_int {
                freeMBlocks(
                    *a.as_mut_ptr().offset(j as isize),
                    *sizes.as_mut_ptr().offset(j as isize),
                );
            }

            b = rand() % MAXALLOC + 1 as c_int;

            let ref mut fresh5 = *a.as_mut_ptr().offset(j as isize);
            *fresh5 = getMBlocks(b as uint32_t);
            *sizes.as_mut_ptr().offset(j as isize) = b as uint32_t;
            j += 1;
        }

        i += 1;
    }

    releaseFreeMemory();
    j = 0 as c_int;

    while j < ARRSIZE {
        freeMBlocks(
            *a.as_mut_ptr().offset(j as isize),
            *sizes.as_mut_ptr().offset(j as isize),
        );

        j += 1;
    }

    releaseFreeMemory();
    i = 0 as c_int;

    while i < LOOPS {
        j = 0 as c_int;

        while j < ARRSIZE {
            b = rand() % MAXALLOC + 1 as c_int;

            let ref mut fresh6 = *a.as_mut_ptr().offset(j as isize);
            *fresh6 = getMBlocks(b as uint32_t);
            *sizes.as_mut_ptr().offset(j as isize) = b as uint32_t;
            j += 1;
        }

        j = ARRSIZE - 1 as c_int;

        while j >= 0 as c_int {
            freeMBlocks(
                *a.as_mut_ptr().offset(j as isize),
                *sizes.as_mut_ptr().offset(j as isize),
            );

            j -= 1;
        }

        i += 1;
    }

    releaseFreeMemory();
    hs_exit();
    exit(0 as c_int);
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
