use crate::ffi::rts::messages::sysErrorBelch;
use crate::ffi::rts::time::Time;
use crate::ffi::stg::W_;
use crate::ffi::stg::types::{StgWord32, StgWord64};
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[inline]
unsafe fn fileTimeToRtsTime(mut ft: FILETIME) -> Time {
    let mut t: Time = 0;
    t = (ft.dwHighDateTime as Time) << 32 as c_int | ft.dwLowDateTime as Time;
    t = t * 100 as Time;

    return t;
}

unsafe fn getProcessTimes(mut user: *mut Time, mut elapsed: *mut Time) {
    *user = getProcessCPUTime();
    *elapsed = getProcessElapsedTime();
}

unsafe fn getCurrentThreadCPUTime() -> Time {
    let mut creationTime = _FILETIME {
        dwLowDateTime: 0,
        dwHighDateTime: 0,
    };

    let mut exitTime = _FILETIME {
        dwLowDateTime: 0,
        dwHighDateTime: 0,
    };

    let mut userTime = _FILETIME {
        dwLowDateTime: 0,
        dwHighDateTime: 0,
    };

    let mut kernelTime = _FILETIME {
        dwLowDateTime: 0 as DWORD,
        dwHighDateTime: 0 as DWORD,
    };

    if GetThreadTimes(
        GetCurrentThread(),
        &raw mut creationTime,
        &raw mut exitTime,
        &raw mut kernelTime,
        &raw mut userTime,
    ) == 0
    {
        sysErrorBelch(
            b"getCurrentThreadCPUTime: Win32 error %lu\0" as *const u8 as *const c_char,
            GetLastError(),
        );

        return 0 as Time;
    }

    return fileTimeToRtsTime(userTime);
}

unsafe fn getProcessCPUTime() -> Time {
    let mut creationTime = _FILETIME {
        dwLowDateTime: 0,
        dwHighDateTime: 0,
    };

    let mut exitTime = _FILETIME {
        dwLowDateTime: 0,
        dwHighDateTime: 0,
    };

    let mut userTime = _FILETIME {
        dwLowDateTime: 0,
        dwHighDateTime: 0,
    };

    let mut kernelTime = _FILETIME {
        dwLowDateTime: 0 as DWORD,
        dwHighDateTime: 0 as DWORD,
    };

    if GetProcessTimes(
        GetCurrentProcess(),
        &raw mut creationTime,
        &raw mut exitTime,
        &raw mut kernelTime,
        &raw mut userTime,
    ) == 0
    {
        return 0 as Time;
    }

    return fileTimeToRtsTime(userTime);
}

static mut qpc_frequency: LARGE_INTEGER = _LARGE_INTEGER {
    QuadPart: 0 as c_int as LONGLONG,
};

unsafe fn initializeTimer() {
    let mut qpc_supported = QueryPerformanceFrequency(&raw mut qpc_frequency) as BOOL;

    if qpc_supported == 0 {
        qpc_frequency.QuadPart = 0 as LONGLONG;
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getMonotonicNSec() -> StgWord64 {
    if qpc_frequency.QuadPart != 0 {
        let mut system_time = _LARGE_INTEGER {
            c2rust_unnamed: C2RustUnnamed_5 {
                LowPart: 0,
                HighPart: 0,
            },
        };

        QueryPerformanceCounter(&raw mut system_time);

        let mut secs = system_time.QuadPart as c_double / qpc_frequency.QuadPart as c_double;

        return (secs * 1e9f64) as StgWord64;
    } else {
        let mut count = GetTickCount();

        return (count as StgWord64).wrapping_mul(1000000 as StgWord64);
    };
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getProcessElapsedTime() -> Time {
    return getMonotonicNSec() as Time;
}

unsafe fn getUnixEpochTime(mut sec: *mut StgWord64, mut nsec: *mut StgWord32) {
    let mut systime = _SYSTEMTIME {
        wYear: 0,
        wMonth: 0,
        wDayOfWeek: 0,
        wDay: 0,
        wHour: 0,
        wMinute: 0,
        wSecond: 0,
        wMilliseconds: 0,
    };

    let mut filetime = _FILETIME {
        dwLowDateTime: 0,
        dwHighDateTime: 0,
    };

    let mut unixtime = _ULARGE_INTEGER {
        c2rust_unnamed: C2RustUnnamed_7 {
            LowPart: 0,
            HighPart: 0,
        },
    };

    GetSystemTime(&raw mut systime);
    SystemTimeToFileTime(&raw mut systime, &raw mut filetime);
    unixtime.c2rust_unnamed.LowPart = filetime.dwLowDateTime;
    unixtime.c2rust_unnamed.HighPart = filetime.dwHighDateTime;
    unixtime.QuadPart = (unixtime.QuadPart as c_ulonglong)
        .wrapping_sub(116444736000000000 as c_ulonglong) as ULONGLONG;
    *sec = (unixtime.QuadPart as c_ulonglong).wrapping_div(10000000 as c_ulonglong) as StgWord64;
    *nsec = ((unixtime.QuadPart as c_ulonglong).wrapping_rem(10000000 as c_ulonglong) as c_ulong)
        .wrapping_mul(100 as c_ulong) as StgWord32;
}

unsafe fn getPageFaults() -> W_ {
    return 0 as W_;
}
