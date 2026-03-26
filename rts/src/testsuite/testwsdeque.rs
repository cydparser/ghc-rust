use crate::ffi::rts::messages::{barf, debugBelch};
use crate::ffi::rts::os_threads::{OSThreadId, createOSThread};
use crate::ffi::stg::types::StgWord;
use crate::prelude::*;
use crate::ws_deque::{WSDeque, newWSDeque, popWSDeque, pushWSDeque, stealWSDeque};

const SCRATCH_SIZE: c_int = 1024 as c_int * 1024 as c_int;

const THREADS: c_int = 3 as c_int;

const POP: c_int = 2 as c_int;

static mut q: *mut WSDeque = null::<WSDeque>() as *mut WSDeque;

static mut scratch: [StgWord; 1048576] = [0; 1048576];

static mut done: StgWord = 0;

static mut ids: [OSThreadId; 3] = [null::<_opaque_pthread_t>() as *mut _opaque_pthread_t; 3];

unsafe fn work(mut p: *mut c_void, mut n: uint32_t) {
    let mut val: StgWord = 0;
    val = *(p as *mut StgWord);

    if val != 0 as StgWord {
        fflush(__stdoutp);
        fflush(__stderrp);

        barf(
            b"FAIL: %p %u %llu\0" as *const u8 as *const c_char,
            p,
            n,
            val,
        );
    }

    *(p as *mut StgWord) = n.wrapping_add(10 as uint32_t) as StgWord;
}

unsafe fn thief(mut info: *mut c_void) -> *mut c_void {
    let mut p = null_mut::<c_void>();
    let mut n: StgWord = 0;
    let mut count: uint32_t = 0 as uint32_t;
    n = info as StgWord;

    while done == 0 {
        p = stealWSDeque(q);

        if !p.is_null() {
            work(p, n.wrapping_add(1 as StgWord) as uint32_t);
            count = count.wrapping_add(1);
        }
    }

    debugBelch(
        b"thread %ld finished, stole %d\0" as *const u8 as *const c_char,
        n,
        count,
    );

    return NULL;
}

unsafe fn main_0(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let mut n: c_int = 0;
    let mut count: uint32_t = 0 as uint32_t;
    let mut p = null_mut::<c_void>();
    q = newWSDeque(1024 as uint32_t);
    done = 0 as StgWord;
    n = 0 as c_int;

    while n < SCRATCH_SIZE {
        scratch[n as usize] = 0 as StgWord;
        n += 1;
    }

    n = 0 as c_int;

    while n < THREADS {
        createOSThread(
            (&raw mut ids as *mut OSThreadId).offset(n as isize) as *mut OSThreadId,
            b"thief\0" as *const u8 as *const c_char,
            transmute::<
                Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
                Option<OSThreadProc>,
            >(Some(
                thief as unsafe extern "C" fn(*mut c_void) -> *mut c_void,
            )),
            n as StgWord as *mut c_void,
        );

        n += 1;
    }

    n = 0 as c_int;

    while n < SCRATCH_SIZE {
        if n % POP != 0 {
            p = popWSDeque(q);

            if !p.is_null() {
                work(p, 0 as uint32_t);
                count = count.wrapping_add(1);
            }
        }

        pushWSDeque(
            q,
            (&raw mut scratch as *mut StgWord).offset(n as isize) as *mut StgWord as *mut c_void,
        );

        n += 1;
    }

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
