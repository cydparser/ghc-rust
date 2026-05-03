use crate::eventlog::event_log::flushAllCapsEventsBufs;
use crate::ffi::ghcversion::__GLASGOW_HASKELL_FULL_VERSION__;
use crate::ffi::rts::{EXIT_INTERNAL_ERROR, stg_exit};
use crate::prelude::*;
use crate::rts_flags::{RtsFlags, TRACE_EVENTLOG};
use std::ffi::VaList;
use std::io::Write as _;
use std::{env, io};

#[cfg(test)]
mod tests;

pub type RtsMsgFunction = unsafe extern "C" fn(*const c_char, VaList) -> ();

pub type RtsMsgFunctionRetLen = unsafe extern "C" fn(*const c_char, VaList) -> c_int;

static mut fatalInternalErrorFn: Option<RtsMsgFunction> =
    unsafe { Some(rtsFatalInternalErrorFn as unsafe extern "C" fn(*const c_char, VaList) -> !) };

static mut debugMsgFn: Option<RtsMsgFunctionRetLen> =
    unsafe { Some(rtsDebugMsgFn as unsafe extern "C" fn(*const c_char, VaList) -> c_int) };

static mut errorMsgFn: Option<RtsMsgFunction> =
    unsafe { Some(rtsErrorMsgFn as unsafe extern "C" fn(*const c_char, VaList) -> ()) };

static mut sysErrorMsgFn: Option<RtsMsgFunction> =
    unsafe { Some(rtsSysErrorMsgFn as unsafe extern "C" fn(*const c_char, VaList) -> ()) };

#[ffi(compiler, ghc_lib, libraries, testsuite, utils)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn barf(s: *const c_char, args: ...) -> ! {
    vbarf(s, args)
}

#[ffi(utils)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vbarf(s: *const c_char, ap: VaList) -> ! {
    fatalInternalErrorFn.expect("non-null fatalInternalErrorFn")(s, ap);
    stg_exit(EXIT_INTERNAL_ERROR)
}

macro_rules! rts_assert {
    ($cond:expr) => {
        if !($cond) {
            $crate::rts_messages::_assertFail(concat!(file!(), "\0").as_ptr().cast(), line!())
        }
    };
}

pub(crate) use rts_assert;

#[ffi(utils)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _assertFail(filename: *const c_char, linenum: c_uint) -> ! {
    barf(
        c"ASSERTION FAILED: file %s, line %u\n".as_ptr(),
        filename,
        linenum,
    )
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn errorBelch(s: *const c_char, args: ...) {
    errorMsgFn.expect("non-null errorMsgFn")(s, args);
}

pub(crate) fn errorBelch0(s: &CStr) {
    unsafe { errorBelch(s.as_ptr()) }
}

macro_rules! rts_assert_warn {
    ($cond:expr) => {
        if !($cond) {
            $crate::rts_messages::_warnFail(concat!(file!(), "\0").as_ptr().cast(), line!())
        }
    };
}

pub(crate) use rts_assert_warn;

fn _warnFail(filename: *const c_char, linenum: u32) {
    unsafe {
        errorBelch(
            c"ASSERTION FAILED: file %s, line %u\n".as_ptr(),
            filename,
            linenum,
        );
    }
}

unsafe fn verrorBelch(s: *const c_char, ap: VaList) {
    errorMsgFn.expect("non-null errorMsgFn")(s, ap);
}

unsafe extern "C" fn sysErrorBelch(s: *const c_char, args: ...) {
    sysErrorMsgFn.expect("non-null sysErrorMsgFn")(s, args);
}

unsafe fn vsysErrorBelch(s: *const c_char, ap: VaList) {
    sysErrorMsgFn.expect("non-null sysErrorMsgFn")(s, ap);
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn debugBelch(s: *const c_char, args: ...) {
    debugMsgFn.expect("non-null debugMsgFn")(s, args);
}

unsafe fn vdebugBelch(s: *const c_char, ap: VaList) -> i32 {
    debugMsgFn.expect("non-null debugMsgFn")(s, ap)
}

fn isGUIApp() -> bool {
    cfg_select! {
        windows => {
            todo!()
        }
        _ => false,
    }
}

unsafe fn rtsFatalInternalErrorFn(s: *const c_char, mut ap: VaList) -> ! {
    let mut args_os = env::args_os();

    let handle = io::stderr().lock();

    if let Some(prog_name) = args_os.next() {
        write!(handle, "{} : ", prog_name.to_string_lossy());
    }
    write!(handle, "internal error: ");

    todo!("printf");

    writeln!(
        handle,
        "    (GHC version {} for {})",
        __GLASGOW_HASKELL_FULL_VERSION__.as_ptr(),
        xstr(HostPlatform_TYPE),
    );

    writeln!(
        handle,
        "    Please report this as a GHC bug:  https://www.haskell.org/ghc/reportabug"
    );

    let _ = handle.flush();

    if RtsFlags.TraceFlags.tracing == TRACE_EVENTLOG {
        flushAllCapsEventsBufs();
    }

    abort()
}

unsafe fn rtsErrorMsgFn(s: *const c_char, mut ap: VaList) {
    if !prog_name.is_null() {
        fprintf(__stderrp, c"%s: ".as_ptr(), prog_name);
    }

    vfprintf(__stderrp, s, ap);
    fprintf(__stderrp, c"\n".as_ptr());
}

unsafe fn rtsSysErrorMsgFn(s: *const c_char, mut ap: VaList) {
    let mut syserr = null_mut::<c_char>();
    syserr = strerror(*__error());

    if !prog_argv.is_null() && !prog_name.is_null() {
        fprintf(__stderrp, c"%s: ".as_ptr(), prog_name);
    }

    vfprintf(__stderrp, s, ap);

    if !syserr.is_null() {
        fprintf(__stderrp, c": %s\n".as_ptr(), syserr);
    } else {
        fprintf(__stderrp, c"\n".as_ptr());
    };
}

unsafe fn rtsDebugMsgFn(s: *const c_char, mut ap: VaList) -> i32 {
    let mut r: i32 = 0;
    r = vfprintf(__stderrp, s, ap);
    fflush(__stderrp);

    return r;
}

unsafe fn rtsBadAlignmentBarf() -> ! {
    barf(c"Encountered incorrectly aligned pointer. This can't be good.".as_ptr())
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rtsOutOfBoundsAccess() -> ! {
    barf(c"Encountered out of bounds array access.".as_ptr())
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rtsMemcpyRangeOverlap() -> ! {
    barf(c"Encountered overlapping source/destination ranges in a memcpy-using op.".as_ptr())
}
