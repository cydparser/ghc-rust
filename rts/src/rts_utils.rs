use std::alloc::{self, Layout};

use crate::config::RtsConfig;
use crate::ffi::ghcversion::__GLASGOW_HASKELL_FULL_VERSION__;
use crate::ffi::rts::storage::block::BLOCK_SIZE;
use crate::ffi::rts::storage::tso::StgTSO;
use crate::ffi::rts::{EXIT_HEAPOVERFLOW, EXIT_INTERNAL_ERROR, stg_exit};
use crate::ffi::stg::W_;
use crate::ghcplatform::{HOST_ARCH, HOST_OS, HOST_VENDOR};
use crate::io_manager::{selectIOManager, showIOManager};
use crate::prelude::*;
use crate::rts_flags::{RtsFlags, get_rts_config};
use crate::stg::types::StgWord64;
use crate::ticky::PrintTickyInfo;

#[cfg(test)]
mod tests;

pub(crate) unsafe fn stgMallocBytes(n: usize, msg: *const c_char) -> *mut c_void {
    let space = libc::malloc(n);

    if space.is_null() {
        if n == 0 {
            return null_mut();
        }

        alloc_fail(n, msg)
    }

    if RtsFlags.DebugFlags.zero_on_gc {
        memset(space, 0xbb, n);
    }

    return space;
}

pub(crate) unsafe fn stgReallocBytes(p: *mut c_void, n: usize, msg: *const c_char) -> *mut c_void {
    let space = libc::realloc(p, n);

    if space.is_null() {
        alloc_fail(n, msg)
    }

    return space;
}

pub(crate) unsafe fn stgCallocBytes(count: usize, size: usize, msg: *const c_char) -> *mut c_void {
    let space = libc::calloc(count, size);

    if space.is_null() {
        alloc_fail(count * size, msg)
    }

    return space;
}

pub(crate) unsafe fn stgStrndup(s: *const c_char, n: usize) -> *const c_char {
    let l = libc::strnlen(s, n);
    let d = stgMallocBytes(l.wrapping_add(1 as usize), c"stgStrndup".as_ptr()) as *mut c_char;

    if d.is_null() {
        return null_mut::<c_char>();
    }

    libc::memcpy(d as *mut c_void, s as *const c_void, l);
    *d.offset(l as isize) = 0;

    return d;
}

pub(crate) unsafe fn stgFree(p: *mut c_void) {
    libc::free(p);
}

pub(crate) unsafe fn stg_alloc<T>(msg: &CStr) -> *mut T {
    stg_alloc_layout(Layout::new::<T>(), msg).cast()
}

pub(crate) unsafe fn stg_alloc_layout(layout: Layout, msg: &CStr) -> *mut u8 {
    let size = layout.size();

    if size == 0 {
        return null_mut();
    }

    let space = alloc::alloc(layout);

    if space.is_null() {
        alloc_fail(size, msg.as_ptr())
    }

    if RtsFlags.DebugFlags.zero_on_gc {
        space.write_bytes(0xbb, size);
    }

    space
}

pub(crate) unsafe fn stg_free<T>(ptr: *mut T) {
    stg_free_layout(ptr.cast(), Layout::new::<T>());
}

pub(crate) unsafe fn stg_free_layout(ptr: *mut u8, layout: Layout) {
    alloc::dealloc(ptr, layout);
}

#[ffi(compiler, ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn reportStackOverflow(mut tso: *mut StgTSO) {
    get_rts_config()
        .stackOverflowHook
        .expect("non-null stackOverflowHook")(
        (*tso).tot_stack_size as usize * size_of::<usize>()
    );

    if RtsFlags.TickyFlags.showTickyStats {
        PrintTickyInfo();
    }
}

#[ffi(compiler, ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn reportHeapOverflow() {
    get_rts_config()
        .outOfHeapHook
        .expect("non-null outOfHeapHook")(
        0, (RtsFlags.GcFlags.maxHeapSize * BLOCK_SIZE) as usize
    );
}

unsafe fn exitHeapOverflow() -> ! {
    reportHeapOverflow();
    stg_exit(EXIT_HEAPOVERFLOW)
}

unsafe fn time_str() -> *mut c_char {
    static mut now: libc::time_t = 0;

    static mut nowstr: [c_char; 26] = [0; 26];

    if now == 0 {
        libc::time(&raw mut now);
        libc::ctime_r(&raw mut now, &raw mut nowstr as *mut c_char);

        libc::memmove(
            (&raw mut nowstr as *mut c_char).offset(16) as *mut c_void,
            (&raw mut nowstr as *mut c_char).offset(19) as *const c_void,
            7,
        );

        nowstr[21] = '\0' as i32 as c_char;
    }

    return &raw mut nowstr as *mut c_char;
}

unsafe fn showStgWord64(x: StgWord64, s: *mut c_char, with_commas: bool) -> *mut c_char {
    if with_commas {
        if x < 10_u64.pow(3) {
            libc::sprintf(s, c"%llu".as_ptr(), x);
        } else if x < 10_u64.pow(6) {
            libc::sprintf(
                s,
                c"%llu,%03llu".as_ptr(),
                x.wrapping_div(1000 as StgWord64),
                x.wrapping_rem(1000 as StgWord64),
            );
        } else if x < 10_u64.pow(9) {
            libc::sprintf(
                s,
                c"%llu,%03llu,%03llu".as_ptr(),
                (x as f64 / 1e6) as StgWord64,
                x.wrapping_div(1000 as StgWord64)
                    .wrapping_rem(1000 as StgWord64),
                x.wrapping_rem(1000 as StgWord64),
            );
        } else if x < 10_u64.pow(12) {
            libc::sprintf(
                s,
                c"%llu,%03llu,%03llu,%03llu".as_ptr(),
                x.wrapping_div(10_u64.pow(9)),
                x.wrapping_div(10_u64.pow(6))
                    .wrapping_rem(1000 as StgWord64),
                x.wrapping_div(10_u64.pow(3))
                    .wrapping_rem(1000 as StgWord64),
                x.wrapping_rem(1000 as StgWord64),
            );
        } else if x < 10_u64.pow(15) {
            libc::sprintf(
                s,
                c"%llu,%03llu,%03llu,%03llu,%03llu".as_ptr(),
                x.wrapping_div(10_u64.pow(12)),
                x.wrapping_div(10_u64.pow(9))
                    .wrapping_rem(1000 as StgWord64),
                x.wrapping_div(10_u64.pow(6))
                    .wrapping_rem(1000 as StgWord64),
                x.wrapping_div(10_u64.pow(3))
                    .wrapping_rem(1000 as StgWord64),
                x.wrapping_rem(1000 as StgWord64),
            );
        } else if x < 10_u64.pow(18) {
            libc::sprintf(
                s,
                c"%llu,%03llu,%03llu,%03llu,%03llu,%03llu".as_ptr(),
                x.wrapping_div(10_u64.pow(15)),
                x.wrapping_div(10_u64.pow(12))
                    .wrapping_rem(1000 as StgWord64),
                x.wrapping_div(10_u64.pow(9))
                    .wrapping_rem(1000 as StgWord64),
                x.wrapping_div(10_u64.pow(6))
                    .wrapping_rem(1000 as StgWord64),
                x.wrapping_div(10_u64.pow(3))
                    .wrapping_rem(1000 as StgWord64),
                x.wrapping_rem(1000 as StgWord64),
            );
        } else {
            libc::sprintf(
                s,
                c"%llu,%03llu,%03llu,%03llu,%03llu,%03llu,%03llu".as_ptr(),
                x.wrapping_div(10_u64.pow(18)),
                x.wrapping_div(10_u64.pow(15))
                    .wrapping_rem(1000 as StgWord64),
                x.wrapping_div(10_u64.pow(12))
                    .wrapping_rem(1000 as StgWord64),
                x.wrapping_div(10_u64.pow(9))
                    .wrapping_rem(1000 as StgWord64),
                x.wrapping_div(10_u64.pow(6))
                    .wrapping_rem(1000 as StgWord64),
                x.wrapping_div(10_u64.pow(3))
                    .wrapping_rem(1000 as StgWord64),
                x.wrapping_rem(1000 as StgWord64),
            );
        }
    } else {
        libc::sprintf(s, c"%llu".as_ptr(), x);
    }

    return s;
}

unsafe fn heapCheckFail() {}

#[ffi(libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn genericRaise(_sig: c_int) -> c_int {
    todo!("pthread_kill(pthread_self(), sig) is very unsafe")
}

unsafe fn mkRtsInfoPair(key: *const c_char, val: *const c_char) {
    libc::printf(c" ,(\"%s\", \"%s\")\n".as_ptr(), key, val);
}

pub(crate) unsafe fn printRtsInfo(rts_config: RtsConfig) {
    libc::printf(c" [(\"GHC RTS\", \"YES\")\n".as_ptr());
    mkRtsInfoPair(
        c"GHC version".as_ptr(),
        __GLASGOW_HASKELL_FULL_VERSION__.as_ptr(),
    );
    mkRtsInfoPair(c"RTS way".as_ptr(), c"TODO RtsWay".as_ptr());
    mkRtsInfoPair(c"Host platform".as_ptr(), c"aarch64-apple-darwin".as_ptr());
    mkRtsInfoPair(c"Host architecture".as_ptr(), HOST_ARCH.as_ptr());
    mkRtsInfoPair(c"Host OS".as_ptr(), HOST_OS.as_ptr());
    mkRtsInfoPair(c"Host vendor".as_ptr(), HOST_VENDOR.as_ptr());
    mkRtsInfoPair(c"Word size".as_ptr(), c"64".as_ptr());
    mkRtsInfoPair(c"Compiler unregisterised".as_ptr(), c"NO".as_ptr());
    mkRtsInfoPair(c"Tables next to code".as_ptr(), c"YES".as_ptr());

    mkRtsInfoPair(
        c"Flag -with-rtsopts".as_ptr(),
        if !rts_config.rts_opts.is_null() {
            rts_config.rts_opts
        } else {
            c"".as_ptr()
        },
    );

    selectIOManager();
    mkRtsInfoPair(c"I/O manager default".as_ptr(), showIOManager());
    libc::printf(c" ]\n".as_ptr());
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub extern "C" fn rts_isProfiled() -> c_int {
    cfg!(feature = "way_profiling") as c_int
}

/// TODO(rust)
#[ffi(compiler)]
#[unsafe(no_mangle)]
pub extern "C" fn rts_isDynamic() -> c_int {
    0
}

#[ffi(compiler, ghc_lib)]
#[unsafe(no_mangle)]
pub extern "C" fn rts_isThreaded() -> c_int {
    cfg!(feature = "way_threaded") as c_int
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub extern "C" fn rts_isDebugged() -> c_int {
    cfg!(feature = "way_debug") as c_int
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub extern "C" fn rts_isTracing() -> c_int {
    1 // TODO(rust)
}

/// TODO(rust): Implement the assembly.
#[cfg(target_arch = "x86")]
pub(crate) unsafe fn checkFPUStack() {
    let mut buf: [c_char; 108] = [0; _];
    asm!("FSAVE %0":"=m" (buf));

    if buf[8] != 255 || buf[9] != 255 {
        errorBelch("NONEMPTY FPU Stack, TAG = %x %x\n", buf[8], buf[9]);
        abort();
    }
}

#[cold]
fn alloc_fail(request_size: W_, msg: *const c_char) -> ! {
    get_rts_config()
        .mallocFailHook
        .expect("non-null mallocFailHook")(request_size, msg);

    stg_exit(EXIT_INTERNAL_ERROR)
}
