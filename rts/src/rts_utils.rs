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

unsafe fn stgMallocBytes(mut n: size_t, mut msg: *mut c_char) -> *mut c_void {
    let mut space = malloc(n);

    if space.is_null() {
        if n == 0 as size_t {
            return NULL;
        }

        rtsConfig.mallocFailHook.expect("non-null function pointer")(n as W_, msg);
        stg_exit(EXIT_INTERNAL_ERROR);
    }

    return space;
}

unsafe fn stgReallocBytes(mut p: *mut c_void, mut n: size_t, mut msg: *mut c_char) -> *mut c_void {
    let mut space = null_mut::<c_void>();
    space = realloc(p, n);

    if space.is_null() {
        rtsConfig.mallocFailHook.expect("non-null function pointer")(n as W_, msg);
        stg_exit(EXIT_INTERNAL_ERROR);
    }

    return space;
}

unsafe fn stgCallocBytes(mut count: size_t, mut size: size_t, mut msg: *mut c_char) -> *mut c_void {
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

unsafe fn stgStrndup(mut s: *const c_char, mut n: size_t) -> *mut c_char {
    let mut l = strnlen(s, n);

    let mut d = stgMallocBytes(
        l.wrapping_add(1 as size_t),
        b"stgStrndup\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut c_char;

    if d.is_null() {
        return null_mut::<c_char>();
    }

    memcpy(d as *mut c_void, s as *const c_void, l);
    *d.offset(l as isize) = 0 as c_char;

    return d;
}

unsafe fn stgFree(mut p: *mut c_void) {
    free(p);
}

unsafe fn stgMallocAlignedBytes(
    mut n: size_t,
    mut align: size_t,
    mut msg: *mut c_char,
) -> *mut c_void {
    let mut space = null_mut::<c_void>();

    if posix_memalign(&raw mut space, align, n) != 0 {
        space = NULL;
    }

    if space.is_null() {
        if n == 0 as size_t {
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
        0 as W_,
        (RtsFlags.GcFlags.maxHeapSize as W_).wrapping_mul(BLOCK_SIZE as W_),
    );
}

unsafe fn exitHeapOverflow() -> ! {
    reportHeapOverflow();
    stg_exit(EXIT_HEAPOVERFLOW);
}

unsafe fn rtsSleep(mut t: Time) -> c_int {
    let mut req = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };

    req.tv_sec = (t / TIME_RESOLUTION as Time) as __darwin_time_t;
    req.tv_nsec = (t - (req.tv_sec * 1000000000 as __darwin_time_t) as Time) as c_long;

    let mut ret: c_int = 0;

    loop {
        ret = nanosleep(&raw mut req, &raw mut req);

        if !(ret == -(1 as c_int) && *__error() == EINTR) {
            break;
        }
    }

    return ret;
}

unsafe fn time_str() -> *mut c_char {
    static mut now: time_t = 0 as time_t;

    static mut nowstr: [c_char; 26] = [0; 26];

    if now == 0 as time_t {
        time(&raw mut now);
        ctime_r(&raw mut now, &raw mut nowstr as *mut c_char);

        memmove(
            (&raw mut nowstr as *mut c_char).offset(16 as c_int as isize) as *mut c_void,
            (&raw mut nowstr as *mut c_char).offset(19 as c_int as isize) as *const c_void,
            7 as size_t,
        );

        nowstr[21 as c_int as usize] = '\0' as i32 as c_char;
    }

    return &raw mut nowstr as *mut c_char;
}

unsafe fn showStgWord64(
    mut x: StgWord64,
    mut s: *mut c_char,
    mut with_commas: bool,
) -> *mut c_char {
    if with_commas {
        if x < 1e3f64 as StgWord64 {
            sprintf(s, b"%llu\0" as *const u8 as *const c_char, x);
        } else if x < 1e6f64 as StgWord64 {
            sprintf(
                s,
                b"%llu,%03llu\0" as *const u8 as *const c_char,
                x.wrapping_div(1000 as StgWord64),
                x.wrapping_rem(1000 as StgWord64),
            );
        } else if x < 1e9f64 as StgWord64 {
            sprintf(
                s,
                b"%llu,%03llu,%03llu\0" as *const u8 as *const c_char,
                (x as c_double / 1e6f64) as StgWord64,
                x.wrapping_div(1000 as StgWord64)
                    .wrapping_rem(1000 as StgWord64),
                x.wrapping_rem(1000 as StgWord64),
            );
        } else if x < 1e12f64 as StgWord64 {
            sprintf(
                s,
                b"%llu,%03llu,%03llu,%03llu\0" as *const u8 as *const c_char,
                x.wrapping_div(1e9f64 as StgWord64),
                x.wrapping_div(1e6f64 as StgWord64)
                    .wrapping_rem(1000 as StgWord64),
                x.wrapping_div(1e3f64 as StgWord64)
                    .wrapping_rem(1000 as StgWord64),
                x.wrapping_rem(1000 as StgWord64),
            );
        } else if x < 1e15f64 as StgWord64 {
            sprintf(
                s,
                b"%llu,%03llu,%03llu,%03llu,%03llu\0" as *const u8 as *const c_char,
                x.wrapping_div(1e12f64 as StgWord64),
                x.wrapping_div(1e9f64 as StgWord64)
                    .wrapping_rem(1000 as StgWord64),
                x.wrapping_div(1e6f64 as StgWord64)
                    .wrapping_rem(1000 as StgWord64),
                x.wrapping_div(1e3f64 as StgWord64)
                    .wrapping_rem(1000 as StgWord64),
                x.wrapping_rem(1000 as StgWord64),
            );
        } else if x < 1e18f64 as StgWord64 {
            sprintf(
                s,
                b"%llu,%03llu,%03llu,%03llu,%03llu,%03llu\0" as *const u8 as *const c_char,
                x.wrapping_div(1e15f64 as StgWord64),
                x.wrapping_div(1e12f64 as StgWord64)
                    .wrapping_rem(1000 as StgWord64),
                x.wrapping_div(1e9f64 as StgWord64)
                    .wrapping_rem(1000 as StgWord64),
                x.wrapping_div(1e6f64 as StgWord64)
                    .wrapping_rem(1000 as StgWord64),
                x.wrapping_div(1e3f64 as StgWord64)
                    .wrapping_rem(1000 as StgWord64),
                x.wrapping_rem(1000 as StgWord64),
            );
        } else {
            sprintf(
                s,
                b"%llu,%03llu,%03llu,%03llu,%03llu,%03llu,%03llu\0" as *const u8 as *const c_char,
                x.wrapping_div(1e18f64 as StgWord64),
                x.wrapping_div(1e15f64 as StgWord64)
                    .wrapping_rem(1000 as StgWord64),
                x.wrapping_div(1e12f64 as StgWord64)
                    .wrapping_rem(1000 as StgWord64),
                x.wrapping_div(1e9f64 as StgWord64)
                    .wrapping_rem(1000 as StgWord64),
                x.wrapping_div(1e6f64 as StgWord64)
                    .wrapping_rem(1000 as StgWord64),
                x.wrapping_div(1e3f64 as StgWord64)
                    .wrapping_rem(1000 as StgWord64),
                x.wrapping_rem(1000 as StgWord64),
            );
        }
    } else {
        sprintf(s, b"%llu\0" as *const u8 as *const c_char, x);
    }

    return s;
}

#[ffi(libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn genericRaise(mut sig: c_int) -> c_int {
    return raise(sig);
}

unsafe fn mkRtsInfoPair(mut key: *const c_char, mut val: *const c_char) {
    printf(
        b" ,(\"%s\", \"%s\")\n\0" as *const u8 as *const c_char,
        key,
        val,
    );
}

unsafe fn printRtsInfo(rts_config: RtsConfig) {
    printf(b" [(\"GHC RTS\", \"YES\")\n\0" as *const u8 as *const c_char);

    mkRtsInfoPair(
        b"GHC version\0" as *const u8 as *const c_char,
        __GLASGOW_HASKELL_FULL_VERSION__.as_ptr(),
    );

    mkRtsInfoPair(
        b"RTS way\0" as *const u8 as *const c_char,
        b"TODO RtsWay\0" as *const u8 as *const c_char,
    );

    mkRtsInfoPair(
        b"Host platform\0" as *const u8 as *const c_char,
        b"aarch64-apple-darwin\0" as *const u8 as *const c_char,
    );

    mkRtsInfoPair(
        b"Host architecture\0" as *const u8 as *const c_char,
        HOST_ARCH.as_ptr(),
    );

    mkRtsInfoPair(b"Host OS\0" as *const u8 as *const c_char, HOST_OS.as_ptr());

    mkRtsInfoPair(
        b"Host vendor\0" as *const u8 as *const c_char,
        HOST_VENDOR.as_ptr(),
    );

    mkRtsInfoPair(
        b"Word size\0" as *const u8 as *const c_char,
        b"64\0" as *const u8 as *const c_char,
    );

    mkRtsInfoPair(
        b"Compiler unregisterised\0" as *const u8 as *const c_char,
        b"NO\0" as *const u8 as *const c_char,
    );

    mkRtsInfoPair(
        b"Tables next to code\0" as *const u8 as *const c_char,
        b"YES\0" as *const u8 as *const c_char,
    );

    mkRtsInfoPair(
        b"Flag -with-rtsopts\0" as *const u8 as *const c_char,
        if !rts_config.rts_opts.is_null() {
            rts_config.rts_opts
        } else {
            b"\0" as *const u8 as *const c_char
        },
    );

    selectIOManager();

    mkRtsInfoPair(
        b"I/O manager default\0" as *const u8 as *const c_char,
        showIOManager(),
    );

    printf(b" ]\n\0" as *const u8 as *const c_char);
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_isProfiled() -> c_int {
    return 0 as c_int;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_isDynamic() -> c_int {
    return 0 as c_int;
}

#[ffi(compiler, ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_isThreaded() -> c_int {
    return 0 as c_int;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_isDebugged() -> c_int {
    return 0 as c_int;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_isTracing() -> c_int {
    return 1 as c_int;
}

unsafe fn checkFPUStack() {}

unsafe fn dropExtension(mut path: *mut c_char, mut extension: *const c_char) {
    let mut ext_len = strlen(extension) as c_int;
    let mut path_len = strlen(path) as c_int;

    if ext_len < path_len {
        let mut s: *mut c_char = path.offset((path_len - ext_len) as isize) as *mut c_char;

        if strcmp(s, extension) == 0 as c_int {
            *s = '\0' as i32 as c_char;
        }
    }
}
