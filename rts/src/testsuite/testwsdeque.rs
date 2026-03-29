use crate::ffi::rts::messages::{barf, debugBelch};
use crate::ffi::rts::os_threads::{OSThreadId, createOSThread};
use crate::ffi::stg::types::StgWord;
use crate::prelude::*;
use crate::ws_deque::{WSDeque, newWSDeque, popWSDeque, pushWSDeque, stealWSDeque};

const SCRATCH_SIZE: i32 = 1024 * 1024;

const THREADS: i32 = 3;

const POP: i32 = 2;

static mut q: *mut WSDeque = null_mut::<WSDeque>();

static mut scratch: [StgWord; 1048576] = [0; 1048576];

static mut done: StgWord = 0;

static mut ids: [OSThreadId; 3] = [null_mut::<_opaque_pthread_t>(); 3];

unsafe fn work(mut p: *mut c_void, mut n: u32) {
    let mut val: StgWord = 0;
    val = *(p as *mut StgWord);

    if val != 0 {
        fflush(__stdoutp);
        fflush(__stderrp);
        barf(c"FAIL: %p %u %llu".as_ptr(), p, n, val);
    }

    *(p as *mut StgWord) = n.wrapping_add(10 as u32) as StgWord;
}

unsafe fn thief(mut info: *mut c_void) -> *mut c_void {
    let mut p = null_mut::<c_void>();
    let mut n: StgWord = 0;
    let mut count: u32 = 0;
    n = info as StgWord;

    while done == 0 {
        p = stealWSDeque(q);

        if !p.is_null() {
            work(p, n.wrapping_add(1 as StgWord) as u32);
            count = count.wrapping_add(1);
        }
    }

    debugBelch(c"thread %ld finished, stole %d".as_ptr(), n, count);

    return NULL;
}

unsafe fn main_0(mut argc: i32, mut argv: *mut *mut c_char) -> i32 {
    let mut n: i32 = 0;
    let mut count: u32 = 0;
    let mut p = null_mut::<c_void>();
    q = newWSDeque(1024);
    done = 0;
    n = 0;

    while n < SCRATCH_SIZE {
        scratch[n as usize] = 0;
        n += 1;
    }

    n = 0;

    while n < THREADS {
        createOSThread(
            (&raw mut ids as *mut OSThreadId).offset(n as isize) as *mut OSThreadId,
            c"thief".as_ptr(),
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

    n = 0;

    while n < SCRATCH_SIZE {
        if n % POP != 0 {
            p = popWSDeque(q);

            if !p.is_null() {
                work(p, 0);
                count = count.wrapping_add(1);
            }
        }

        pushWSDeque(
            q,
            (&raw mut scratch as *mut StgWord).offset(n as isize) as *mut StgWord as *mut c_void,
        );

        n += 1;
    }

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
