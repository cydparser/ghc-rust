use crate::ffi::rts::linker::pathchar;
use crate::ffi::rts::messages::errorBelch;
use crate::ffi::rts::{_assertFail, stg_exit};
use crate::hs_ffi::hs_restoreConsoleCP;
use crate::linker_internals::resolveSymbolAddr;
use crate::prelude::*;
use crate::rts_flags::RtsFlags;

const EH_UNWINDING: i32 = 0x2;

static mut __hs_handle: PVOID = NULL;

static mut oldTopFilter: LPTOP_LEVEL_EXCEPTION_FILTER = None;

unsafe fn __hs_exception_handler(mut exception_data: *mut _EXCEPTION_POINTERS) -> i64 {
    let mut action = EXCEPTION_CONTINUE_SEARCH as i64;
    let mut exit_code = EXIT_FAILURE;
    let mut what: ULONG_PTR = 0;
    fprintf(__stderrp, c"\n".as_ptr());

    if !exception_data.is_null()
        && !(*exception_data).ExceptionRecord.is_null()
        && (*(*exception_data).ExceptionRecord).ExceptionFlags & EH_UNWINDING as DWORD == 0
    {
        match (*(*exception_data).ExceptionRecord).ExceptionCode {
            EXCEPTION_FLT_DIVIDE_BY_ZERO | EXCEPTION_INT_DIVIDE_BY_ZERO => {
                fprintf(__stderrp, c"divide by zero\n".as_ptr());
                action = EXCEPTION_CONTINUE_EXECUTION as i64;
                exit_code = SIGFPE;
            }
            EXCEPTION_STACK_OVERFLOW => {
                fprintf(__stderrp, c"C stack overflow in generated code\n".as_ptr());
                action = EXCEPTION_CONTINUE_EXECUTION as i64;
            }
            EXCEPTION_ACCESS_VIOLATION => {
                if (*(*exception_data).ExceptionRecord).NumberParameters < 2 {
                    fprintf(
                        __stderrp,
                        c"Access violation in generated code. Empty exception record.".as_ptr(),
                    );
                } else {
                    what = (*(*exception_data).ExceptionRecord).ExceptionInformation[0];

                    fprintf(
                        __stderrp,
                        c"Access violation in generated code when %s 0x%lx\n".as_ptr(),
                        if what == 0 {
                            c"reading".as_ptr()
                        } else if what == 1 {
                            c"writing".as_ptr()
                        } else if what == 8 {
                            c"executing data at".as_ptr()
                        } else {
                            c"?".as_ptr()
                        },
                        (*(*exception_data).ExceptionRecord).ExceptionInformation[1] as usize,
                    );
                }

                action = EXCEPTION_CONTINUE_EXECUTION as i64;
                exit_code = SIGSEGV;
            }
            _ => {}
        }

        if EXCEPTION_CONTINUE_EXECUTION as i64 == action {
            fflush(__stderrp);
            hs_restoreConsoleCP();
            generateStack(exception_data as *mut EXCEPTION_POINTERS);
            generateDump(exception_data as *mut EXCEPTION_POINTERS);
            stg_exit(exit_code);
        }
    }

    return action;
}

unsafe fn __hs_exception_filter(mut exception_data: *mut _EXCEPTION_POINTERS) -> i64 {
    let mut result = EXCEPTION_CONTINUE_EXECUTION as i64;

    if let Some(filter) = oldTopFilter {
        result = filter(exception_data) as i64;

        if EXCEPTION_CONTINUE_SEARCH as i64 == result {
            result = EXCEPTION_CONTINUE_EXECUTION as i64;
        }
    }

    return result;
}

unsafe fn __register_hs_exception_handler() {
    if !RtsFlags.MiscFlags.install_seh_handlers {
        return;
    }

    if __hs_handle.is_null() {
        fprintf(
            __stderrp,
            c"\n TODO(rust): __hs_handle = AddVectoredContinueHandler(CALL_LAST, __hs_exception_handler);\n"
                .as_ptr(),
        );

        if !__hs_handle.is_null() as i32 as i64 != 0 {
        } else {
            _assertFail(c"/Users/cyd/src/ghc/rts/win32/veh_excn.c".as_ptr(), 196);
        }

        fprintf(
            __stderrp,
            c"\n TODO(rust): oldTopFilter = SetUnhandledExceptionFilter (__hs_exception_filter);\n"
                .as_ptr(),
        );
    } else {
        errorBelch(
            c"There is no need to call __register_hs_exception_handler() twice, VEH handlers are global per process."
                .as_ptr(),
        );
    };
}

unsafe fn __unregister_hs_exception_handler() {
    if !RtsFlags.MiscFlags.install_seh_handlers {
        return;
    }

    if !__hs_handle.is_null() {
        RemoveVectoredContinueHandler(__hs_handle);
        __hs_handle = NULL as PVOID;
    } else {
        errorBelch(
            c"__unregister_hs_exception_handler() called without havingcalled __register_hs_exception_handler() first."
                .as_ptr(),
        );
    };
}

unsafe fn generateDump(mut pExceptionPointers: *mut EXCEPTION_POINTERS) {
    if !RtsFlags.MiscFlags.generate_dump_file {
        return;
    }

    let mut szPath: [WCHAR; 260] = [0; 260];
    let mut szFileName: [WCHAR; 260] = [0; 260];

    let szAppName = &raw const transmute::<[u8; 16], [i32; 4]>(*b"g\0\0\0h\0\0\0c\0\0\0\0\0\0\0")
        as *const WCHAR;

    let szVersion = &raw const transmute::<[u8; 4], [i32; 1]>(*b"\0\0\0\0") as *const WCHAR;

    let mut dwBufferSize = MAX_PATH as DWORD;
    let mut hDumpFile = null_mut::<c_void>();

    let mut stLocalTime = _SYSTEMTIME {
        wYear: 0,
        wMonth: 0,
        wDayOfWeek: 0,
        wDay: 0,
        wHour: 0,
        wMinute: 0,
        wSecond: 0,
        wMilliseconds: 0,
    };

    let mut ExpParam = _MINIDUMP_EXCEPTION_INFORMATION {
        ThreadId: 0,
        ExceptionPointers: null_mut::<_EXCEPTION_POINTERS>(),
        ClientPointers: 0,
    };

    GetLocalTime(&raw mut stLocalTime);
    GetTempPathW(dwBufferSize, &raw mut szPath as LPWSTR);

    swprintf(
        &raw mut szFileName as *mut char,
        MAX_PATH as usize,
        &raw const transmute::<
            [u8; 192],
            [i32; 48],
        >(
            *b"%\0\0\0l\0\0\0s\0\0\0%\0\0\0l\0\0\0s\0\0\0%\0\0\0l\0\0\0s\0\0\0-\0\0\0%\0\0\x000\0\0\x004\0\0\0d\0\0\0%\0\0\x000\0\0\x002\0\0\0d\0\0\0%\0\0\x000\0\0\x002\0\0\0d\0\0\0-\0\0\0%\0\0\x000\0\0\x002\0\0\0d\0\0\0%\0\0\x000\0\0\x002\0\0\0d\0\0\0%\0\0\x000\0\0\x002\0\0\0d\0\0\0-\0\0\0%\0\0\0l\0\0\0d\0\0\0-\0\0\0%\0\0\0l\0\0\0d\0\0\0.\0\0\0d\0\0\0m\0\0\0p\0\0\0\0\0\0\0",
        ) as *const char,
        &raw mut szPath as *mut WCHAR,
        szAppName,
        szVersion,
        stLocalTime.wYear as i32,
        stLocalTime.wMonth as i32,
        stLocalTime.wDay as i32,
        stLocalTime.wHour as i32,
        stLocalTime.wMinute as i32,
        stLocalTime.wSecond as i32,
        GetCurrentProcessId(),
        GetCurrentThreadId(),
    );

    hDumpFile = CreateFileW(
        &raw mut szFileName as *mut WCHAR as LPCWSTR,
        GENERIC_READ | GENERIC_WRITE as DWORD,
        (FILE_SHARE_WRITE | FILE_SHARE_READ) as DWORD,
        null_mut::<_SECURITY_ATTRIBUTES>(),
        CREATE_ALWAYS as DWORD,
        0,
        null_mut::<c_void>(),
    );

    ExpParam.ThreadId = GetCurrentThreadId();
    ExpParam.ExceptionPointers = pExceptionPointers as PEXCEPTION_POINTERS;
    ExpParam.ClientPointers = TRUE as WINBOOL;

    MiniDumpWriteDump(
        GetCurrentProcess(),
        GetCurrentProcessId(),
        hDumpFile,
        (MiniDumpNormal as i32
            | MiniDumpWithDataSegs as i32
            | MiniDumpWithThreadInfo as i32
            | MiniDumpWithCodeSegs as i32) as MINIDUMP_TYPE,
        &raw mut ExpParam,
        null_mut::<_MINIDUMP_USER_STREAM_INFORMATION>(),
        null_mut::<_MINIDUMP_CALLBACK_INFORMATION>(),
    );

    fprintf(
        __stderrp,
        c"Crash dump created. Dump written to:\n\t%ls".as_ptr(),
        &raw mut szFileName as *mut WCHAR,
    );
}

unsafe fn generateStack(mut pExceptionPointers: *mut EXCEPTION_POINTERS) {
    if !RtsFlags.MiscFlags.generate_stack_trace {
        return;
    }

    let mut context = (*pExceptionPointers).ContextRecord;

    let mut stackFrame = _tagSTACKFRAME64 {
        AddrPC: _tagADDRESS64 {
            Offset: 0,
            Segment: 0,
            Mode: AddrMode1616,
        },
        AddrReturn: _tagADDRESS64 {
            Offset: 0,
            Segment: 0,
            Mode: AddrMode1616,
        },
        AddrFrame: _tagADDRESS64 {
            Offset: 0,
            Segment: 0,
            Mode: AddrMode1616,
        },
        AddrStack: _tagADDRESS64 {
            Offset: 0,
            Segment: 0,
            Mode: AddrMode1616,
        },
        AddrBStore: _tagADDRESS64 {
            Offset: 0,
            Segment: 0,
            Mode: AddrMode1616,
        },
        FuncTableEntry: null_mut::<c_void>(),
        Params: [0; 4],
        Far: 0,
        Virtual: 0,
        Reserved: [0; 3],
        KdHelp: _KDHELP64 {
            Thread: 0,
            ThCallbackStack: 0,
            ThCallbackBStore: 0,
            NextCallback: 0,
            FramePointer: 0,
            KiCallUserMode: 0,
            KeUserCallbackDispatcher: 0,
            SystemRangeStart: 0,
            KiUserExceptionDispatcher: 0,
            StackBase: 0,
            StackLimit: 0,
            BuildVersion: 0,
            RetpolineStubFunctionTableSize: 0,
            RetpolineStubFunctionTable: 0,
            RetpolineStubOffset: 0,
            RetpolineStubSize: 0,
            Reserved0: [0; 2],
        },
    };

    let mut machineType: DWORD = 0;
    machineType = IMAGE_FILE_MACHINE_ARM64 as DWORD;
    stackFrame.AddrPC.Offset = (*context).Pc;
    stackFrame.AddrPC.Mode = AddrModeFlat;
    stackFrame.AddrFrame.Offset = (*context).c2rust_unnamed.c2rust_unnamed.Fp;
    stackFrame.AddrFrame.Mode = AddrModeFlat;
    stackFrame.AddrStack.Offset = (*context).Sp;
    stackFrame.AddrStack.Mode = AddrModeFlat;
    fprintf(
        __stderrp,
        c"\n Attempting to reconstruct a stack trace...\n\n".as_ptr(),
    );

    if SymInitialize(GetCurrentProcess(), null::<CHAR>(), true) == 0 {
        fprintf(
            __stderrp,
            c"  \nNOTE: Symbols could not be loaded. Addresses may be unresolved.\n\n".as_ptr(),
        );
    }

    let mut max_frames = 35;
    fprintf(__stderrp, c"   Frame\tCode address\n".as_ptr());

    let mut lastBp = 0;

    while StackWalk64(
        machineType,
        GetCurrentProcess(),
        GetCurrentThread(),
        &raw mut stackFrame,
        context as PVOID,
        None,
        Some(SymFunctionTableAccess64 as unsafe extern "C" fn(HANDLE, DWORD64) -> PVOID),
        Some(SymGetModuleBase64 as unsafe extern "C" fn(HANDLE, DWORD64) -> DWORD64),
        None,
    ) != 0
        && max_frames > 0
    {
        if stackFrame.AddrPC.Offset == 0 {
            fprintf(__stderrp, c"Null address\n".as_ptr());
            break;
        } else {
            let mut buffer: [char; 1024] = [0; 1024];
            let mut topSp: usize = 0;

            fprintf(
                __stderrp,
                c" * 0x%lx\t%ls\n".as_ptr(),
                stackFrame.AddrFrame.Offset as usize,
                resolveSymbolAddr(
                    &raw mut buffer as *mut pathchar,
                    1024,
                    stackFrame.AddrPC.Offset as isize as *mut c_void,
                    &raw mut topSp,
                ),
            );

            if lastBp >= stackFrame.AddrFrame.Offset {
                fprintf(__stderrp, c"Stack frame out of sequence...\n".as_ptr());
                break;
            } else {
                lastBp = stackFrame.AddrFrame.Offset;
                max_frames -= 1;

                if max_frames == 0 {
                    fprintf(
                        __stderrp,
                        c"\n   ... (maximum recursion depth reached.)\n".as_ptr(),
                    );
                }
            }
        }
    }

    fprintf(__stderrp, c"\n".as_ptr());
    fflush(__stderrp);
}
