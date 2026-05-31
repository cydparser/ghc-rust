/*!
= General message generation functions

All messages should go through here.  We can't guarantee that
stdout/stderr will be available - e.g. in a Windows program there
is no console for generating messages, so they have to either go to
to the debug console, or pop up message boxes.
*/

use std::ffi::VaList;
use std::io::Write as _;
use std::{io, process};

use errno::errno;
use printf_compat as printf;
#[cfg(windows)]
use windows_sys::Win32::Foundation::HWND;
#[cfg(windows)]
use windows_sys::Win32::UI::WindowsAndMessaging::{MB_ICONERROR, MB_OK, MB_TASKMODAL, MessageBoxA};

use crate::event_log::flushAllCapsEventsBufs;
use crate::ffi::ghcversion::__GLASGOW_HASKELL_FULL_VERSION__;
use crate::ffi::rts::{EXIT_INTERNAL_ERROR, stg_exit};
use crate::prelude::*;
use crate::rts_flags::{RtsFlags, TRACE_EVENTLOG, get_prog_name};

#[cfg(test)]
mod tests;

#[ffi]
pub type RtsMsgFunction = unsafe extern "C" fn(*const c_char, VaList);

#[ffi]
pub type RtsMsgFunctionRetLen = unsafe extern "C" fn(*const c_char, VaList) -> c_int;

#[ffi]
#[unsafe(no_mangle)]
pub static mut fatalInternalErrorFn: Option<unsafe extern "C" fn(*const c_char, VaList) -> !> =
    Some(rtsFatalInternalErrorFn);

#[ffi]
#[unsafe(no_mangle)]
pub static mut debugMsgFn: Option<RtsMsgFunctionRetLen> = Some(rtsDebugMsgFn);

#[ffi]
#[unsafe(no_mangle)]
pub static mut errorMsgFn: Option<RtsMsgFunction> = Some(rtsErrorMsgFn);

#[ffi]
#[unsafe(no_mangle)]
pub static mut sysErrorMsgFn: Option<RtsMsgFunction> = Some(rtsSysErrorMsgFn);

#[inline(always)]
fn get_handle() -> impl io::Write {
    cfg_select! {
        test => self::testing::get_handle(),
        _ => io::stderr().lock(),
    }
}

#[cfg(test)]
mod testing {
    use std::io;

    pub(super) fn get_handle() -> Box<dyn io::Write> {
        todo!("Add ability to capture output in a `Vec<u8>`")
    }
}

#[ffi(compiler, ghc_lib, libraries, testsuite, utils)]
#[unsafe(no_mangle)]
#[cold]
pub unsafe extern "C" fn barf(s: *const c_char, args: ...) -> ! {
    vbarf(s, args)
}

#[ffi(utils)]
#[unsafe(no_mangle)]
#[cold]
pub unsafe extern "C" fn vbarf(s: *const c_char, ap: VaList) -> ! {
    fatalInternalErrorFn.expect("non-null fatalInternalErrorFn")(s, ap);
    #[expect(unreachable_code)]
    // Just in case fatalInternalErrorFn() returns.
    stg_exit(EXIT_INTERNAL_ERROR)
}

#[cold]
pub(crate) unsafe fn sbarf(s: *const c_char) -> ! {
    barf(c"%s".as_ptr(), s)
}

macro_rules! rts_assert {
    ($cond:expr) => {
        if !($cond) {
            $crate::rts_messages::_assertFail(concat!(file!(), "\0").as_ptr().cast(), line!())
        }
    };
}

pub(crate) use rts_assert;

macro_rules! rts_debug_assert {
    ($cond:expr) => {
        if cfg!(debug_assertions) {
            $crate::rts_messages::rts_assert!($cond)
        }
    };
}

pub(crate) use rts_debug_assert;

#[ffi(utils)]
#[unsafe(no_mangle)]
#[cold]
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
#[cold]
pub unsafe extern "C" fn errorBelch(s: *const c_char, args: ...) {
    errorMsgFn.expect("non-null errorMsgFn")(s, args);
}

#[cold]
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

#[cold]
fn _warnFail(filename: *const c_char, linenum: u32) {
    unsafe {
        errorBelch(
            c"ASSERTION FAILED: file %s, line %u\n".as_ptr(),
            filename,
            linenum,
        );
    }
}

#[cold]
unsafe fn verrorBelch(s: *const c_char, ap: VaList) {
    errorMsgFn.expect("non-null errorMsgFn")(s, ap);
}

#[cold]
pub(crate) unsafe extern "C" fn sysErrorBelch(s: *const c_char, args: ...) {
    sysErrorMsgFn.expect("non-null sysErrorMsgFn")(s, args)
}

#[cold]
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

const BUFSIZE: usize = 512;

fn isGUIApp() -> bool {
    #[cfg(windows)]
    {
        use windows_sys::Win32::System::Diagnostics::Debug;
        use windows_sys::Win32::System::Diagnostics::Debug::IMAGE_SUBSYSTEM_WINDOWS_GUI;
        use windows_sys::Win32::System::LibraryLoader::GetModuleHandleA;
        use windows_sys::Win32::System::SystemServices::{
            IMAGE_DOS_HEADER, IMAGE_DOS_SIGNATURE, IMAGE_NT_SIGNATURE,
        };

        todo!("test isGUIApp on Windows");

        let pDOSHeader: *const IMAGE_DOS_HEADER = unsafe { GetModuleHandleA(null()).cast() };

        if unsafe { *pDOSHeader.e_magic != IMAGE_DOS_SIGNATURE } {
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
    let prog_name = get_prog_name();

    if isGUIApp() {
        #[cfg(windows)]
        {
            let mut message: [u8; BUFSIZE];
            let mut title: [u8; BUFSIZE];

            write_printf(
                &mut title.as_mut_slice(),
                c"%s: internal error".as_ptr(),
                prog_name.as_ptr(),
            );

            write_printf(&mut message.as_mut_slice(), s, ap);

            let hwnd: HWND = null_mut();

            MessageBoxA(
                hwnd,
                message.as_ptr().cast(),
                title.as_ptr().cast(),
                MB_OK | MB_ICONERROR | MB_TASKMODAL,
            );
        }
    } else {
        let mut handle = get_handle();

        if !prog_name.is_empty() {
            write!(handle, "{}: ", prog_name.as_str());
        }
        write!(handle, "internal error: ");

        printf::format(s, ap, printf::output::io_write(&mut handle));

        todo!("if USE_LIBDW");

        writeln!(
            handle,
            "\n    (GHC version {} for {})",
            __GLASGOW_HASKELL_FULL_VERSION__, HostPlatform_TYPE,
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

unsafe extern "C" fn rtsErrorMsgFn(s: *const c_char, ap: VaList) {
    let mut handle = get_handle();

    let prog_name = get_prog_name();

    if isGUIApp() {
        #[cfg(windows)]
        {
            let mut buf: [u8; BUFSIZE];

            let r = write_printf(&mut buf.as_mut_slice(), s, ap);

            if r > 0 && r < BUFSIZE as i32 {
                let hwnd: HWND = null_mut();

                MessageBox(
                    hwnd,
                    buf.as_ptr(),
                    prog_name.as_ptr,
                    MB_OK | MB_ICONERROR | MB_TASKMODAL,
                );
            }
        }
    } else {
        if !prog_name.is_empty() {
            write!(handle, "{}: ", prog_name.as_str());
        }

        write_printf(&mut handle, s, ap);
        writeln!(handle);
    }
}

unsafe extern "C" fn rtsSysErrorMsgFn(s: *const c_char, ap: VaList) {
    let prog_name = get_prog_name();

    if isGUIApp() {
        #[cfg(windows)]
        {
            let mut buf: [u8; BUFSIZE];

            let r: usize = write_printf(&mut buf.as_mut_slice(), s, ap) as usize;

            if r > 0 && r < BUFSIZE {
                let syserr = errno();

                if syserr.0 != 0 {
                    // Ensure `buf` ends with a nul.
                    if write_printf(&mut buf[r..], c": %s".as_ptr(), syserr) as usize == BUFSIZE {
                        buf[BUFSIZE - 1] = 0;
                    }
                }

                let hwnd: HWND = null_mut();

                MessageBox(
                    hwnd,
                    buf.as_ptr(),
                    prog_name.as_ptr,
                    MB_OK | MB_ICONERROR | MB_TASKMODAL,
                );
            }
        }
    } else {
        let mut handle = get_handle();

        if !prog_name.is_empty() {
            write!(handle, "{}: ", prog_name.as_str());
        }

        write_printf(&mut handle, s, ap);

        let syserr = errno();

        if syserr.0 != 0 {
            writeln!(handle, ": {}", syserr);
        } else {
            writeln!(handle);
        };
    }
}

unsafe extern "C" fn rtsDebugMsgFn(s: *const c_char, ap: VaList) -> i32 {
    let mut r = 0;

    if isGUIApp() {
        #[cfg(windows)]
        {
            let mut buf: [u8; BUFSIZE];
            r = write_printf(&mut buf.as_mut_slice(), s, ap);

            if r > 0 && r < BUFSIZE as i32 {
                use windows_sys::Win32::System::Diagnostics::Debug::OutputDebugStringA;

                OutputDebugStringA(buf.as_ptr());
            }
        }
    } else {
        let mut handle = get_handle();

        r = write_printf(&mut handle, s, ap);
        _ = handle.flush();
    }

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

#[inline(always)]
pub(crate) unsafe extern "C" fn write_printf(
    w: &mut impl io::Write,
    format: *const c_char,
    ap: ...
) -> c_int {
    printf::format(format, ap, printf::output::io_write(w))
}
