use crate::ffi::hs_ffi::hs_restoreConsoleCP;
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::linker::pathchar;
use crate::ffi::rts::messages::errorBelch;
use crate::ffi::rts::{_assertFail, stg_exit};
use crate::linker_internals::resolveSymbolAddr;
use crate::prelude::*;

const EH_UNWINDING: c_int = 0x2 as c_int;

static mut __hs_handle: PVOID = NULL;

static mut oldTopFilter: LPTOP_LEVEL_EXCEPTION_FILTER = None;

unsafe fn __hs_exception_handler(mut exception_data: *mut _EXCEPTION_POINTERS) -> c_long {
    let mut action = EXCEPTION_CONTINUE_SEARCH as c_long;
    let mut exit_code = EXIT_FAILURE;
    let mut what: ULONG_PTR = 0;
    fprintf(__stderrp, b"\n\0" as *const u8 as *const c_char);

    if !exception_data.is_null()
        && !(*exception_data).ExceptionRecord.is_null()
        && (*(*exception_data).ExceptionRecord).ExceptionFlags & EH_UNWINDING as DWORD == 0 as DWORD
    {
        match (*(*exception_data).ExceptionRecord).ExceptionCode {
            EXCEPTION_FLT_DIVIDE_BY_ZERO | EXCEPTION_INT_DIVIDE_BY_ZERO => {
                fprintf(
                    __stderrp,
                    b"divide by zero\n\0" as *const u8 as *const c_char,
                );

                action = EXCEPTION_CONTINUE_EXECUTION as c_long;
                exit_code = SIGFPE;
            }
            EXCEPTION_STACK_OVERFLOW => {
                fprintf(
                    __stderrp,
                    b"C stack overflow in generated code\n\0" as *const u8 as *const c_char,
                );

                action = EXCEPTION_CONTINUE_EXECUTION as c_long;
            }
            EXCEPTION_ACCESS_VIOLATION => {
                if (*(*exception_data).ExceptionRecord).NumberParameters < 2 as DWORD {
                    fprintf(
                        __stderrp,
                        b"Access violation in generated code. Empty exception record.\0"
                            as *const u8 as *const c_char,
                    );
                } else {
                    what = (*(*exception_data).ExceptionRecord).ExceptionInformation
                        [0 as c_int as usize];

                    fprintf(
                        __stderrp,
                        b"Access violation in generated code when %s 0x%lx\n\0" as *const u8
                            as *const c_char,
                        if what == 0 as ULONG_PTR {
                            b"reading\0" as *const u8 as *const c_char
                        } else if what == 1 as ULONG_PTR {
                            b"writing\0" as *const u8 as *const c_char
                        } else if what == 8 as ULONG_PTR {
                            b"executing data at\0" as *const u8 as *const c_char
                        } else {
                            b"?\0" as *const u8 as *const c_char
                        },
                        (*(*exception_data).ExceptionRecord).ExceptionInformation
                            [1 as c_int as usize] as uintptr_t,
                    );
                }

                action = EXCEPTION_CONTINUE_EXECUTION as c_long;
                exit_code = SIGSEGV;
            }
            _ => {}
        }

        if EXCEPTION_CONTINUE_EXECUTION as c_long == action {
            fflush(__stderrp);
            hs_restoreConsoleCP();
            generateStack(exception_data as *mut EXCEPTION_POINTERS);
            generateDump(exception_data as *mut EXCEPTION_POINTERS);
            stg_exit(exit_code);
        }
    }

    return action;
}

unsafe fn __hs_exception_filter(mut exception_data: *mut _EXCEPTION_POINTERS) -> c_long {
    let mut result = EXCEPTION_CONTINUE_EXECUTION as c_long;

    if oldTopFilter.is_some() {
        result = Some(oldTopFilter.expect("non-null function pointer"))
            .expect("non-null function pointer")(exception_data) as c_long;

        if EXCEPTION_CONTINUE_SEARCH as c_long == result {
            result = EXCEPTION_CONTINUE_EXECUTION as c_long;
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
            b"\n TODO(rust): __hs_handle = AddVectoredContinueHandler(CALL_LAST, __hs_exception_handler);\n\0"
                as *const u8 as *const c_char,
        );

        if !__hs_handle.is_null() as c_int as c_long != 0 {
        } else {
            _assertFail(
                b"/Users/cyd/src/ghc/rts/win32/veh_excn.c\0" as *const u8 as *const c_char,
                196 as c_uint,
            );
        }

        fprintf(
            __stderrp,
            b"\n TODO(rust): oldTopFilter = SetUnhandledExceptionFilter (__hs_exception_filter);\n\0"
                as *const u8 as *const c_char,
        );
    } else {
        errorBelch(
            b"There is no need to call __register_hs_exception_handler() twice, VEH handlers are global per process.\0"
                as *const u8 as *const c_char,
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
            b"__unregister_hs_exception_handler() called without havingcalled __register_hs_exception_handler() first.\0"
                as *const u8 as *const c_char,
        );
    };
}

unsafe fn generateDump(mut pExceptionPointers: *mut EXCEPTION_POINTERS) {
    if !RtsFlags.MiscFlags.generate_dump_file {
        return;
    }

    let mut szPath: [WCHAR; 260] = [0; 260];
    let mut szFileName: [WCHAR; 260] = [0; 260];
    let szAppName = &raw const transmute::<[u8; 16], [c_int; 4]>(*b"g\0\0\0h\0\0\0c\0\0\0\0\0\0\0")
        as *const WCHAR;

    let szVersion = &raw const transmute::<[u8; 4], [c_int; 1]>(*b"\0\0\0\0") as *const WCHAR;
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
        &raw mut szFileName as *mut wchar_t,
        MAX_PATH as size_t,
        &raw const transmute::<
            [u8; 192],
            [c_int; 48],
        >(
            *b"%\0\0\0l\0\0\0s\0\0\0%\0\0\0l\0\0\0s\0\0\0%\0\0\0l\0\0\0s\0\0\0-\0\0\0%\0\0\x000\0\0\x004\0\0\0d\0\0\0%\0\0\x000\0\0\x002\0\0\0d\0\0\0%\0\0\x000\0\0\x002\0\0\0d\0\0\0-\0\0\0%\0\0\x000\0\0\x002\0\0\0d\0\0\0%\0\0\x000\0\0\x002\0\0\0d\0\0\0%\0\0\x000\0\0\x002\0\0\0d\0\0\0-\0\0\0%\0\0\0l\0\0\0d\0\0\0-\0\0\0%\0\0\0l\0\0\0d\0\0\0.\0\0\0d\0\0\0m\0\0\0p\0\0\0\0\0\0\0",
        ) as *const wchar_t,
        &raw mut szPath as *mut WCHAR,
        szAppName,
        szVersion,
        stLocalTime.wYear as c_int,
        stLocalTime.wMonth as c_int,
        stLocalTime.wDay as c_int,
        stLocalTime.wHour as c_int,
        stLocalTime.wMinute as c_int,
        stLocalTime.wSecond as c_int,
        GetCurrentProcessId(),
        GetCurrentThreadId(),
    );

    hDumpFile = CreateFileW(
        &raw mut szFileName as *mut WCHAR as LPCWSTR,
        GENERIC_READ | GENERIC_WRITE as DWORD,
        (FILE_SHARE_WRITE | FILE_SHARE_READ) as DWORD,
        null_mut::<_SECURITY_ATTRIBUTES>(),
        CREATE_ALWAYS as DWORD,
        0 as DWORD,
        null_mut::<c_void>(),
    );

    ExpParam.ThreadId = GetCurrentThreadId();
    ExpParam.ExceptionPointers = pExceptionPointers as PEXCEPTION_POINTERS;
    ExpParam.ClientPointers = TRUE as WINBOOL;

    MiniDumpWriteDump(
        GetCurrentProcess(),
        GetCurrentProcessId(),
        hDumpFile,
        (MiniDumpNormal as c_int
            | MiniDumpWithDataSegs as c_int
            | MiniDumpWithThreadInfo as c_int
            | MiniDumpWithCodeSegs as c_int) as MINIDUMP_TYPE,
        &raw mut ExpParam,
        null_mut::<_MINIDUMP_USER_STREAM_INFORMATION>(),
        null_mut::<_MINIDUMP_CALLBACK_INFORMATION>(),
    );

    fprintf(
        __stderrp,
        b"Crash dump created. Dump written to:\n\t%ls\0" as *const u8 as *const c_char,
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
            Offset: 0 as DWORD64,
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
        b"\n Attempting to reconstruct a stack trace...\n\n\0" as *const u8 as *const c_char,
    );

    if SymInitialize(GetCurrentProcess(), null::<CHAR>(), r#true) == 0 {
        fprintf(
            __stderrp,
            b"  \nNOTE: Symbols could not be loaded. Addresses may be unresolved.\n\n\0"
                as *const u8 as *const c_char,
        );
    }

    let mut max_frames = 35 as c_int;

    fprintf(
        __stderrp,
        b"   Frame\tCode address\n\0" as *const u8 as *const c_char,
    );

    let mut lastBp = 0 as DWORD64;

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
        && max_frames > 0 as c_int
    {
        if stackFrame.AddrPC.Offset == 0 as DWORD64 {
            fprintf(__stderrp, b"Null address\n\0" as *const u8 as *const c_char);
            break;
        } else {
            let mut buffer: [wchar_t; 1024] = [0; 1024];
            let mut topSp: uintptr_t = 0 as uintptr_t;

            fprintf(
                __stderrp,
                b" * 0x%lx\t%ls\n\0" as *const u8 as *const c_char,
                stackFrame.AddrFrame.Offset as uintptr_t,
                resolveSymbolAddr(
                    &raw mut buffer as *mut pathchar,
                    1024 as c_int,
                    stackFrame.AddrPC.Offset as intptr_t as *mut c_void,
                    &raw mut topSp,
                ),
            );

            if lastBp >= stackFrame.AddrFrame.Offset {
                fprintf(
                    __stderrp,
                    b"Stack frame out of sequence...\n\0" as *const u8 as *const c_char,
                );

                break;
            } else {
                lastBp = stackFrame.AddrFrame.Offset;
                max_frames -= 1;

                if max_frames == 0 as c_int {
                    fprintf(
                        __stderrp,
                        b"\n   ... (maximum recursion depth reached.)\n\0" as *const u8
                            as *const c_char,
                    );
                }
            }
        }
    }

    fprintf(__stderrp, b"\n\0" as *const u8 as *const c_char);
    fflush(__stderrp);
}
