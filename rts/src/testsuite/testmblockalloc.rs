use crate::ffi::hs_ffi::hs_exit;
use crate::ffi::rts::storage::m_block::{freeMBlocks, getMBlocks, releaseFreeMemory};
use crate::ffi::rts_api::{RtsOptsAll, defaultRtsConfig, hs_init_ghc};
use crate::prelude::*;

static mut MAXALLOC: i32 = 16;

static mut ARRSIZE: i32 = 64;

static mut LOOPS: i32 = 1000;

static mut SEED: i32 = 0xf00f00;

unsafe fn main_0(mut argc: i32, mut argv: *mut *mut c_char) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut b: i32 = 0;
    let vla = ARRSIZE as usize;
    let mut a: Vec<*mut c_void> = ::std::vec::from_elem(null_mut::<c_void>(), vla);
    let vla_0 = ARRSIZE as usize;
    let mut sizes: Vec<u32> = ::std::vec::from_elem(0, vla_0);
    srand(SEED as u32);

    let mut conf = defaultRtsConfig;
    conf.rts_opts_enabled = RtsOptsAll;
    hs_init_ghc(&raw mut argc, &raw mut argv, conf);
    i = 0;

    while i < LOOPS {
        j = 0;

        while j < ARRSIZE {
            if i > 0 {
                freeMBlocks(
                    *a.as_mut_ptr().offset(j as isize),
                    *sizes.as_mut_ptr().offset(j as isize),
                );
            }

            b = rand() % MAXALLOC + 1;

            let ref mut fresh5 = *a.as_mut_ptr().offset(j as isize);
            *fresh5 = getMBlocks(b as u32);
            *sizes.as_mut_ptr().offset(j as isize) = b as u32;
            j += 1;
        }

        i += 1;
    }

    releaseFreeMemory();
    j = 0;

    while j < ARRSIZE {
        freeMBlocks(
            *a.as_mut_ptr().offset(j as isize),
            *sizes.as_mut_ptr().offset(j as isize),
        );

        j += 1;
    }

    releaseFreeMemory();
    i = 0;

    while i < LOOPS {
        j = 0;

        while j < ARRSIZE {
            b = rand() % MAXALLOC + 1;

            let ref mut fresh6 = *a.as_mut_ptr().offset(j as isize);
            *fresh6 = getMBlocks(b as u32);
            *sizes.as_mut_ptr().offset(j as isize) = b as u32;
            j += 1;
        }

        j = ARRSIZE - 1;

        while j >= 0 {
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
    exit(0);
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
