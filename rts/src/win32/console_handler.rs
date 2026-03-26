use crate::ffi::hs_ffi::HsWord32;
use crate::ffi::rts::io_interface::sendIOManagerEvent;
use crate::ffi::rts::messages::errorBelch;
use crate::ffi::rts::signals::{STG_SIG_DFL, STG_SIG_HAN, STG_SIG_IGN};
use crate::ffi::rts::{EXIT_INTERRUPTED, stg_exit};
use crate::ffi::stg::types::{StgInt, StgStablePtr, StgWord8};
use crate::prelude::*;
use crate::schedule::{SCHED_INTERRUPTING, getSchedState, interruptStgRts};
use crate::win32::mio_manager::interruptIOManagerEvent;

#[cfg(test)]
mod tests;

static mut deliver_event: bool = r#true != 0;

static mut console_handler: StgInt = STG_SIG_DFL as StgInt;

unsafe fn initUserSignals() {
    console_handler = STG_SIG_DFL as StgInt;
}

unsafe fn freeSignalHandlers() {}

unsafe fn finiUserSignals() {}

unsafe fn shutdown_handler(mut dwCtrlType: DWORD) -> BOOL {
    match dwCtrlType {
        2 => return r#false,
        0 | 1 => {
            if getSchedState() as c_uint >= SCHED_INTERRUPTING as c_int as c_uint {
                stg_exit(EXIT_INTERRUPTED);
            } else {
                interruptStgRts();
            }

            return r#true;
        }
        _ => return r#false,
    };
}

unsafe fn initDefaultHandlers() {
    if SetConsoleCtrlHandler(
        Some(shutdown_handler as unsafe extern "C" fn(DWORD) -> BOOL),
        r#true,
    ) == 0
    {
        errorBelch(
            b"warning: failed to install default console handler\0" as *const u8 as *const c_char,
        );
    }
}

unsafe fn resetDefaultHandlers() {
    if SetConsoleCtrlHandler(
        Some(shutdown_handler as unsafe extern "C" fn(DWORD) -> BOOL),
        r#false,
    ) == 0
    {
        errorBelch(
            b"warning: failed to uninstall default console handler\0" as *const u8 as *const c_char,
        );
    }
}

#[ffi(libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn blockUserSignals() {
    deliver_event = r#false != 0;
}

#[ffi(libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn unblockUserSignals() {
    deliver_event = r#true != 0;
}

unsafe fn awaitUserSignals() {}

unsafe fn generic_handler(mut dwCtrlType: DWORD) -> BOOL {
    match dwCtrlType {
        2 => return r#false,
        _ => {
            if !deliver_event {
                return r#true;
            }

            sendIOManagerEvent((dwCtrlType << 1 as c_int | 1 as DWORD) as StgWord8 as HsWord32);
            interruptIOManagerEvent();

            return r#true;
        }
    };
}

unsafe fn rts_InstallConsoleEvent(mut action: c_int, mut handler: *mut StgStablePtr) -> c_int {
    let mut previous_hdlr: StgInt = console_handler;

    match action {
        STG_SIG_IGN => {
            console_handler = STG_SIG_IGN as StgInt;

            if SetConsoleCtrlHandler(None, r#true) == 0 {
                errorBelch(
                    b"warning: unable to ignore console events\0" as *const u8 as *const c_char,
                );
            }
        }
        STG_SIG_DFL => {
            console_handler = STG_SIG_IGN as StgInt;

            if SetConsoleCtrlHandler(None, r#false) == 0 {
                errorBelch(
                    b"warning: unable to restore default console event handling\0" as *const u8
                        as *const c_char,
                );
            }
        }
        STG_SIG_HAN => {
            console_handler = STG_SIG_HAN as StgInt;

            if previous_hdlr < 0 as StgInt || previous_hdlr == STG_SIG_HAN as StgInt {
                if SetConsoleCtrlHandler(
                    Some(generic_handler as unsafe extern "C" fn(DWORD) -> BOOL),
                    r#true,
                ) == 0
                {
                    errorBelch(
                        b"warning: unable to install console event handler\0" as *const u8
                            as *const c_char,
                    );
                }
            }
        }
        _ => {}
    }

    if previous_hdlr == STG_SIG_DFL as StgInt
        || previous_hdlr == STG_SIG_IGN as StgInt
        || previous_hdlr == STG_SIG_HAN as StgInt
    {
        return previous_hdlr as c_int;
    } else {
        if !handler.is_null() {
            *handler = previous_hdlr as StgStablePtr;
        }

        return STG_SIG_HAN;
    };
}

unsafe fn rts_ConsoleHandlerDone(mut ev: c_int) {}
