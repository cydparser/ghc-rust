/*!
= General message generation functions

All messages should go through here.  We can't guarantee that
stdout/stderr will be available - e.g. in a Windows program there
is no console for generating messages, so they have to either go to
to the debug console, or pop up message boxes.
*/

use crate::eventlog::event_log::flushAllCapsEventsBufs;
use crate::ffi::ghcversion::__GLASGOW_HASKELL_FULL_VERSION__;
use crate::ffi::rts::{EXIT_INTERNAL_ERROR, stg_exit};
use crate::prelude::*;
use crate::rts_flags::{RtsFlags, TRACE_EVENTLOG};
use printf_compat as printf;
use std::ffi::VaList;
use std::io::Write as _;
use std::{env, io, process};

#[cfg(test)]
mod tests;

#[ffi]
pub type RtsMsgFunction = unsafe extern "C" fn(*const c_char, VaList);

#[ffi]
pub type RtsMsgFunctionRetLen = unsafe extern "C" fn(*const c_char, VaList) -> c_int;

#[ffi]
pub static mut fatalInternalErrorFn: Option<unsafe extern "C" fn(*const c_char, VaList) -> !> =
    Some(rtsFatalInternalErrorFn);

#[ffi]
pub static mut debugMsgFn: Option<RtsMsgFunctionRetLen> = Some(rtsDebugMsgFn);

#[ffi]
pub static mut errorMsgFn: Option<RtsMsgFunction> = Some(rtsErrorMsgFn);

#[ffi]
pub static mut sysErrorMsgFn: Option<RtsMsgFunction> = Some(rtsSysErrorMsgFn);

#[ffi(compiler, ghc_lib, libraries, testsuite, utils)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn barf(s: *const c_char, args: ...) -> ! {
    vbarf(s, args)
}

#[ffi(utils)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vbarf(s: *const c_char, ap: VaList) -> ! {
    fatalInternalErrorFn.expect("non-null fatalInternalErrorFn")(s, ap);
    #[expect(unreachable_code)]
    // Just in case fatalInternalErrorFn() returns.
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
pub extern "C" fn _assertFail(filename: *const c_char, linenum: c_uint) -> ! {
    unsafe {
        barf(
            c"ASSERTION FAILED: file %s, line %u\n".as_ptr(),
            filename,
            linenum,
        )
    }
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
    sysErrorMsgFn.expect("non-null sysErrorMsgFn")(s, args)
}

unsafe fn vsysErrorBelch(s: *const c_char, ap: VaList) {
    sysErrorMsgFn.expect("non-null sysErrorMsgFn")(s, ap)
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn debugBelch(s: *const c_char, args: ...) {
    debugMsgFn.expect("non-null debugMsgFn")(s, args);
}

pub(crate) unsafe fn vdebugBelch(s: *const c_char, ap: VaList) -> i32 {
    debugMsgFn.expect("non-null debugMsgFn")(s, ap)
}

fn isGUIApp() -> bool {
    #[cfg(windows)]
    {
        use windows_sys::Win32::System::Diagnostics::Debug;
        use windows_sys::Win32::System::Diagnostics::Debug::IMAGE_SUBSYSTEM_WINDOWS_GUI;
        use windows_sys::Win32::System::LibraryLoader::GetModuleHandleA;
        use windows_sys::Win32::System::SystemServices::{IMAGE_DOS_HEADER, IMAGE_DOS_SIGNATURE};

        todo!("test isGUIApp on Windows");

        let pDOSHeader: *const IMAGE_DOS_HEADER = unsafe { GetModuleHandleA(null()).cast() };

        if (*pDOSHeader).e_magic != IMAGE_DOS_SIGNATURE {
            return false;
        }

        #[cfg(target_pointer_width = "32")]
        type PIMAGE_NT_HEADERS = *const Debug::IMAGE_NT_HEADERS32;
        #[cfg(target_pointer_width = "64")]
        type PIMAGE_NT_HEADERS = *const Debug::IMAGE_NT_HEADERS64;

        let pPEHeader: PIMAGE_NT_HEADERS = unsafe {
            (&raw const pDOSHeader)
                .cast::<u8>()
                .add(pDOSHeader.e_lfanew.into())
                .cast()
        };

        if unsafe { (*pPEHeader).Signature != IMAGE_NT_SIGNATURE } {
            return false;
        }

        return unsafe { (*pPEHeader).OptionalHeader.Subsystem == IMAGE_SUBSYSTEM_WINDOWS_GUI };
    }

    false
}

unsafe extern "C" fn rtsFatalInternalErrorFn(s: *const c_char, mut ap: VaList) -> ! {
    if isGUIApp() {
        #[cfg(windows)]
        {
            const BUFSIZE: usize = 512;

            let message: [u8; BUFSIZE];
            let title: [u8; BUFSIZE];

            printf::format(
                "%s: internal error",
                prog_name.unwrap_or(c"".as_ptr()),
                printf::io_write(&mut title),
            );

            printf::format(s, ap, printf::io_write(&mut message));

            use windows_sys::Win32::Foundation::HWND;
            use windows_sys::Win32::UI::WindowsAndMessaging::{
                MB_ICONERROR, MB_OK, MB_TASKMODAL, MessageBoxA,
            };

            let hwnd: HWND = null_mut();

            MessageBoxA(
                hwnd,
                &raw const message,
                &raw const title,
                MB_OK | MB_ICONERROR | MB_TASKMODAL,
            );
        }
    } else {
        let mut handle = io::stderr().lock();

        if let Some(prog_name) = env::args_os().next() {
            write!(handle, "{}: ", prog_name.to_string_lossy());
        }
        write!(handle, "internal error: ");

        printf::format(s, ap, printf::output::io_write(&mut handle));

        todo!("if USE_LIBDW");

        writeln!(
            handle,
            "\n    (GHC version {} for {})",
            __GLASGOW_HASKELL_FULL_VERSION__.as_ptr(),
            HostPlatform_TYPE,
        );

        writeln!(
            handle,
            "    Please report this as a GHC bug:  https://www.haskell.org/ghc/reportabug"
        );

        handle.flush();

        if RtsFlags.TraceFlags.tracing == TRACE_EVENTLOG {
            flushAllCapsEventsBufs();
        }
    }

    process::abort()
}

unsafe extern "C" fn rtsErrorMsgFn(s: *const c_char, mut ap: VaList) {
    let mut handle = io::stderr().lock();

    if let Some(prog_name) = env::args_os().next() {
        write!(handle, "{}: ", prog_name.to_string_lossy());
    }

    todo!("vfprintf(__stderrp, s, ap)");
    writeln!(handle);
}

unsafe extern "C" fn rtsSysErrorMsgFn(s: *const c_char, mut ap: VaList) {
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

unsafe extern "C" fn rtsDebugMsgFn(s: *const c_char, mut ap: VaList) -> i32 {
    let mut handle = io::stderr().lock();
    let r = vfprintf(__stderrp, s, ap);
    _ = handle.flush();

    r
}

pub(crate) extern "C" fn rtsBadAlignmentBarf() -> ! {
    unsafe { barf(c"Encountered incorrectly aligned pointer. This can't be good.".as_ptr()) }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub extern "C" fn rtsOutOfBoundsAccess() -> ! {
    unsafe { barf(c"Encountered out of bounds array access.".as_ptr()) }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub extern "C" fn rtsMemcpyRangeOverlap() -> ! {
    unsafe {
        barf(c"Encountered overlapping source/destination ranges in a memcpy-using op.".as_ptr())
    }
}
