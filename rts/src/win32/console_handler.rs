use crate::ffi::rts::io_interface::sendIOManagerEvent;
use crate::ffi::rts::messages::errorBelch;
use crate::ffi::rts::signals::{STG_SIG_DFL, STG_SIG_HAN, STG_SIG_IGN};
use crate::ffi::rts::{EXIT_INTERRUPTED, stg_exit};
use crate::ffi::stg::types::{StgInt, StgStablePtr, StgWord8};
use crate::hs_ffi::HsWord32;
use crate::prelude::*;
use crate::schedule::{SCHED_INTERRUPTING, getSchedState, interruptStgRts};
use crate::win32::mio_manager::interruptIOManagerEvent;

#[cfg(test)]
mod tests;

static mut deliver_event: bool = true;

static mut console_handler: StgInt = STG_SIG_DFL as StgInt;

unsafe fn initUserSignals() {
    console_handler = STG_SIG_DFL as StgInt;
}

unsafe fn freeSignalHandlers() {}

unsafe fn finiUserSignals() {}

unsafe fn shutdown_handler(mut dwCtrlType: DWORD) -> BOOL {
    match dwCtrlType {
        2 => return false,
        0 | 1 => {
            if getSchedState() as u32 >= SCHED_INTERRUPTING as i32 as u32 {
                stg_exit(EXIT_INTERRUPTED);
            } else {
                interruptStgRts();
            }

            return true;
        }
        _ => return false,
    };
}

unsafe fn initDefaultHandlers() {
    if SetConsoleCtrlHandler(
        Some(shutdown_handler as unsafe extern "C" fn(DWORD) -> BOOL),
        true,
    ) == 0
    {
        errorBelch(c"warning: failed to install default console handler".as_ptr());
    }
}

unsafe fn resetDefaultHandlers() {
    if SetConsoleCtrlHandler(
        Some(shutdown_handler as unsafe extern "C" fn(DWORD) -> BOOL),
        false,
    ) == 0
    {
        errorBelch(c"warning: failed to uninstall default console handler".as_ptr());
    }
}

#[ffi(libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn blockUserSignals() {
    deliver_event = false;
}

#[ffi(libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn unblockUserSignals() {
    deliver_event = true;
}

unsafe fn awaitUserSignals() {}

unsafe fn generic_handler(mut dwCtrlType: DWORD) -> BOOL {
    match dwCtrlType {
        2 => return false,
        _ => {
            if !deliver_event {
                return true;
            }

            sendIOManagerEvent((dwCtrlType << 1 | 1) as StgWord8 as HsWord32);
            interruptIOManagerEvent();

            return true;
        }
    };
}

unsafe fn rts_InstallConsoleEvent(mut action: i32, mut handler: *mut StgStablePtr) -> i32 {
    let mut previous_hdlr: StgInt = console_handler;

    match action {
        STG_SIG_IGN => {
            console_handler = STG_SIG_IGN as StgInt;

            if SetConsoleCtrlHandler(None, true) == 0 {
                errorBelch(c"warning: unable to ignore console events".as_ptr());
            }
        }
        STG_SIG_DFL => {
            console_handler = STG_SIG_IGN as StgInt;

            if SetConsoleCtrlHandler(None, false) == 0 {
                errorBelch(c"warning: unable to restore default console event handling".as_ptr());
            }
        }
        STG_SIG_HAN => {
            console_handler = STG_SIG_HAN as StgInt;

            if previous_hdlr < 0 || previous_hdlr == STG_SIG_HAN as StgInt {
                if SetConsoleCtrlHandler(
                    Some(generic_handler as unsafe extern "C" fn(DWORD) -> BOOL),
                    true,
                ) == 0
                {
                    errorBelch(c"warning: unable to install console event handler".as_ptr());
                }
            }
        }
        _ => {}
    }

    if previous_hdlr == STG_SIG_DFL as StgInt
        || previous_hdlr == STG_SIG_IGN as StgInt
        || previous_hdlr == STG_SIG_HAN as StgInt
    {
        return previous_hdlr as i32;
    } else {
        if !handler.is_null() {
            *handler = previous_hdlr as StgStablePtr;
        }

        return STG_SIG_HAN;
    };
}

unsafe fn rts_ConsoleHandlerDone(mut ev: i32) {}
