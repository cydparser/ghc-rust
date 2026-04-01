use crate::ffi::rts::messages::{barf, debugBelch};
use crate::ffi::rts::os_threads::{OSThreadId, createOSThread};
use crate::ffi::stg::types::{StgInt, StgWord};
use crate::prelude::*;
use crate::ws_deque::cas_top;
use crate::ws_deque::{WSDeque, looksEmptyWSDeque, newWSDeque, popWSDeque, pushWSDeque};

const SCRATCH_SIZE: i32 = 1024 * 1024;

const THREADS: i32 = 3;

const POP: i32 = 2;

static mut q: *mut WSDeque = null_mut::<WSDeque>();

static mut scratch: [StgWord; 1048576] = [0; 1048576];

static mut done: StgWord = 0;

static mut ids: [OSThreadId; 3] = [null_mut::<_opaque_pthread_t>(); 3];

static mut bufs: [i32; 3] = [0; 3];

static mut last_b: [[StgWord; 128]; 3] = [[0; 128]; 3];

static mut last_t: [[StgWord; 128]; 3] = [[0; 128]; 3];

static mut last_v: [[StgWord; 128]; 3] = [[0; 128]; 3];

unsafe fn myStealWSDeque_(mut q_0: *mut WSDeque, mut n: u32) -> *mut c_void {
    let mut stolen = null_mut::<c_void>();
    let mut t: StgWord = (&raw mut (*q_0).top).load(Ordering::Acquire) as StgWord;
    ::std::sync::atomic::fence(::std::sync::atomic::Ordering::SeqCst);

    let mut b: StgWord = (&raw mut (*q_0).bottom).load(Ordering::Acquire) as StgWord;
    let mut result = NULL;

    if t < b {
        result = ((*q_0)
            .elements
            .offset(t.wrapping_rem((*q_0).size as StgWord) as isize)
            as *mut *mut c_void)
            .load(Ordering::Relaxed);

        if !cas_top(q_0, t as StgInt, t.wrapping_add(1 as StgWord) as StgInt) {
            return NULL;
        }
    }

    return result;
}

unsafe fn myStealWSDeque(mut q_0: *mut WSDeque, mut n: u32) -> *mut c_void {
    let mut stolen = null_mut::<c_void>();

    loop {
        stolen = myStealWSDeque_(q_0, n);

        if !(stolen.is_null() && !looksEmptyWSDeque(q_0)) {
            break;
        }
    }

    return stolen;
}

unsafe fn dump() {
    let mut n: u32 = 0;
    let mut i: u32 = 0;
    n = 0;

    while n < THREADS as u32 {
        debugBelch(c"\nthread %d:\n".as_ptr(), n);
        i = bufs[n as usize] as u32;

        while i
            >= ({
                let mut _a = bufs[n as usize] - 20;
                let mut _b = 0;
                (if _a <= _b { _b as i32 } else { _a as i32 })
            }) as u32
        {
            debugBelch(
                c"%d: t=%ld b=%ld = %ld\n".as_ptr(),
                i,
                last_t[n as usize][i as usize],
                last_b[n as usize][i as usize],
                last_v[n as usize][i as usize],
            );

            i = i.wrapping_sub(1);
        }

        n = n.wrapping_add(1);
    }
}

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
        p = myStealWSDeque(q, n as u32);

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

    debugBelch(c"main thread finished, popped %d".as_ptr(), count);
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
