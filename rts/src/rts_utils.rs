use crate::ffi::ghcversion::__GLASGOW_HASKELL_FULL_VERSION__;
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::storage::block::BLOCK_SIZE;
use crate::ffi::rts::time::{TIME_RESOLUTION, Time};
use crate::ffi::rts::time::{ctime_r, nanosleep, time};
use crate::ffi::rts::types::StgTSO;
use crate::ffi::rts::{EXIT_HEAPOVERFLOW, EXIT_INTERNAL_ERROR, stg_exit};
use crate::ffi::rts_api::RtsConfig;
use crate::ffi::stg::W_;
use crate::ffi::stg::types::StgWord64;
use crate::ghcplatform::{HOST_ARCH, HOST_OS, HOST_VENDOR};
use crate::io_manager::{selectIOManager, showIOManager};
use crate::prelude::*;
use crate::rts_flags::rtsConfig;

#[cfg(test)]
mod tests;

unsafe fn stgMallocBytes(mut n: usize, mut msg: *mut c_char) -> *mut c_void {
    let mut space = malloc(n);

    if space.is_null() {
        if n == 0 {
            return NULL;
        }

        rtsConfig.mallocFailHook.expect("non-null function pointer")(n as W_, msg);
        stg_exit(EXIT_INTERNAL_ERROR);
    }

    return space;
}

unsafe fn stgReallocBytes(mut p: *mut c_void, mut n: usize, mut msg: *mut c_char) -> *mut c_void {
    let mut space = null_mut::<c_void>();
    space = realloc(p, n);

    if space.is_null() {
        rtsConfig.mallocFailHook.expect("non-null function pointer")(n as W_, msg);
        stg_exit(EXIT_INTERNAL_ERROR);
    }

    return space;
}

unsafe fn stgCallocBytes(mut count: usize, mut size: usize, mut msg: *mut c_char) -> *mut c_void {
    let mut space = null_mut::<c_void>();
    space = calloc(count, size);

    if space.is_null() {
        rtsConfig.mallocFailHook.expect("non-null function pointer")(
            (count as W_).wrapping_mul(size as W_),
            msg,
        );

        stg_exit(EXIT_INTERNAL_ERROR);
    }

    return space;
}

unsafe fn stgStrndup(mut s: *const c_char, mut n: usize) -> *mut c_char {
    let mut l = strnlen(s, n);
    let mut d = stgMallocBytes(l.wrapping_add(1 as usize), c"stgStrndup".as_ptr()) as *mut c_char;

    if d.is_null() {
        return null_mut::<c_char>();
    }

    memcpy(d as *mut c_void, s as *const c_void, l);
    *d.offset(l as isize) = 0;

    return d;
}

unsafe fn stgFree(mut p: *mut c_void) {
    free(p);
}

unsafe fn stgMallocAlignedBytes(
    mut n: usize,
    mut align: usize,
    mut msg: *mut c_char,
) -> *mut c_void {
    let mut space = null_mut::<c_void>();

    if posix_memalign(&raw mut space, align, n) != 0 {
        space = NULL;
    }

    if space.is_null() {
        if n == 0 {
            return NULL;
        }

        rtsConfig.mallocFailHook.expect("non-null function pointer")(n as W_, msg);
        stg_exit(EXIT_INTERNAL_ERROR);
    }

    return space;
}

unsafe fn stgFreeAligned(mut p: *mut c_void) {
    free(p);
}

#[ffi(compiler, ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn reportStackOverflow(mut tso: *mut StgTSO) {
    rtsConfig
        .stackOverflowHook
        .expect("non-null function pointer")(
        ((*tso).tot_stack_size as usize).wrapping_mul(size_of::<W_>() as usize) as W_,
    );
}

#[ffi(compiler, ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn reportHeapOverflow() {
    rtsConfig.outOfHeapHook.expect("non-null function pointer")(
        0,
        (RtsFlags.GcFlags.maxHeapSize as W_).wrapping_mul(BLOCK_SIZE as W_),
    );
}

unsafe fn exitHeapOverflow() -> ! {
    reportHeapOverflow();
    stg_exit(EXIT_HEAPOVERFLOW);
}

unsafe fn rtsSleep(mut t: Time) -> i32 {
    let mut req = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    req.tv_sec = (t / TIME_RESOLUTION as Time) as i64;
    req.tv_nsec = (t - (req.tv_sec * 1000000000) as Time) as i64;

    let mut ret: i32 = 0;

    loop {
        ret = nanosleep(&raw mut req, &raw mut req);

        if !(ret == -1 && *__error() == EINTR) {
            break;
        }
    }

    return ret;
}

unsafe fn time_str() -> *mut c_char {
    static mut now: time_t = 0;

    static mut nowstr: [c_char; 26] = [0; 26];

    if now == 0 {
        time(&raw mut now);
        ctime_r(&raw mut now, &raw mut nowstr as *mut c_char);

        memmove(
            (&raw mut nowstr as *mut c_char).offset(16) as *mut c_void,
            (&raw mut nowstr as *mut c_char).offset(19) as *const c_void,
            7,
        );

        nowstr[21] = '\0' as i32 as c_char;
    }

    return &raw mut nowstr as *mut c_char;
}

unsafe fn showStgWord64(
    mut x: StgWord64,
    mut s: *mut c_char,
    mut with_commas: bool,
) -> *mut c_char {
    if with_commas {
        if x < 1e3f64 {
            sprintf(s, c"%llu".as_ptr(), x);
        } else if x < 1e6f64 {
            sprintf(
                s,
                c"%llu,%03llu".as_ptr(),
                x.wrapping_div(1000 as StgWord64),
                x.wrapping_rem(1000 as StgWord64),
            );
        } else if x < 1e9f64 {
            sprintf(
                s,
                c"%llu,%03llu,%03llu".as_ptr(),
                (x as f64 / 1e6f64) as StgWord64,
                x.wrapping_div(1000 as StgWord64)
                    .wrapping_rem(1000 as StgWord64),
                x.wrapping_rem(1000 as StgWord64),
            );
        } else if x < 1e12f64 {
            sprintf(
                s,
                c"%llu,%03llu,%03llu,%03llu".as_ptr(),
                x.wrapping_div(1e9f64),
                x.wrapping_div(1e6f64).wrapping_rem(1000 as StgWord64),
                x.wrapping_div(1e3f64).wrapping_rem(1000 as StgWord64),
                x.wrapping_rem(1000 as StgWord64),
            );
        } else if x < 1e15f64 {
            sprintf(
                s,
                c"%llu,%03llu,%03llu,%03llu,%03llu".as_ptr(),
                x.wrapping_div(1e12f64),
                x.wrapping_div(1e9f64).wrapping_rem(1000 as StgWord64),
                x.wrapping_div(1e6f64).wrapping_rem(1000 as StgWord64),
                x.wrapping_div(1e3f64).wrapping_rem(1000 as StgWord64),
                x.wrapping_rem(1000 as StgWord64),
            );
        } else if x < 1e18f64 {
            sprintf(
                s,
                c"%llu,%03llu,%03llu,%03llu,%03llu,%03llu".as_ptr(),
                x.wrapping_div(1e15f64),
                x.wrapping_div(1e12f64).wrapping_rem(1000 as StgWord64),
                x.wrapping_div(1e9f64).wrapping_rem(1000 as StgWord64),
                x.wrapping_div(1e6f64).wrapping_rem(1000 as StgWord64),
                x.wrapping_div(1e3f64).wrapping_rem(1000 as StgWord64),
                x.wrapping_rem(1000 as StgWord64),
            );
        } else {
            sprintf(
                s,
                c"%llu,%03llu,%03llu,%03llu,%03llu,%03llu,%03llu".as_ptr(),
                x.wrapping_div(1e18f64),
                x.wrapping_div(1e15f64).wrapping_rem(1000 as StgWord64),
                x.wrapping_div(1e12f64).wrapping_rem(1000 as StgWord64),
                x.wrapping_div(1e9f64).wrapping_rem(1000 as StgWord64),
                x.wrapping_div(1e6f64).wrapping_rem(1000 as StgWord64),
                x.wrapping_div(1e3f64).wrapping_rem(1000 as StgWord64),
                x.wrapping_rem(1000 as StgWord64),
            );
        }
    } else {
        sprintf(s, c"%llu".as_ptr(), x);
    }

    return s;
}

#[ffi(libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn genericRaise(mut sig: i32) -> i32 {
    return raise(sig);
}

unsafe fn mkRtsInfoPair(mut key: *const c_char, mut val: *const c_char) {
    printf(c" ,(\"%s\", \"%s\")\n".as_ptr(), key, val);
}

unsafe fn printRtsInfo(rts_config: RtsConfig) {
    printf(c" [(\"GHC RTS\", \"YES\")\n".as_ptr());
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
    printf(c" ]\n".as_ptr());
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_isProfiled() -> i32 {
    return 0;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_isDynamic() -> i32 {
    return 0;
}

#[ffi(compiler, ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_isThreaded() -> i32 {
    return 0;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_isDebugged() -> i32 {
    return 0;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_isTracing() -> i32 {
    return 1;
}

unsafe fn checkFPUStack() {}

unsafe fn dropExtension(mut path: *mut c_char, mut extension: *const c_char) {
    let mut ext_len = strlen(extension) as i32;
    let mut path_len = strlen(path) as i32;

    if ext_len < path_len {
        let mut s: *mut c_char = path.offset((path_len - ext_len) as isize) as *mut c_char;

        if strcmp(s, extension) == 0 {
            *s = '\0' as i32 as c_char;
        }
    }
}
