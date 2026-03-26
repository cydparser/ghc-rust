use crate::ffi::rts::messages::sysErrorBelch;
use crate::ffi::rts::stg_exit;
use crate::ffi::rts::time::{
    _CLOCK_MONOTONIC, _CLOCK_PROCESS_CPUTIME_ID, clock_gettime, clockid_t,
};
use crate::ffi::rts::time::{TIME_RESOLUTION, Time};
use crate::ffi::stg::W_;
use crate::ffi::stg::types::{StgWord32, StgWord64};
use crate::prelude::*;

#[cfg(test)]
mod tests;

static mut timer_scaling_factor_numer: uint64_t = 0 as uint64_t;

static mut timer_scaling_factor_denom: uint64_t = 0 as uint64_t;

unsafe fn initializeTimer() {
    let mut info = mach_timebase_info { numer: 0, denom: 0 };
    mach_timebase_info(&raw mut info);
    timer_scaling_factor_numer = info.numer as uint64_t;
    timer_scaling_factor_denom = info.denom as uint64_t;
}

unsafe fn getClockTime(mut clock: clockid_t) -> Time {
    let mut ts = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };

    let mut res = clock_gettime(clock, &raw mut ts);

    if res == 0 as c_int {
        return ts.tv_sec as Time * TIME_RESOLUTION as Time + ts.tv_nsec as Time;
    } else {
        sysErrorBelch(b"clock_gettime\0" as *const u8 as *const c_char);
        stg_exit(EXIT_FAILURE);
    };
}

unsafe fn getCurrentThreadCPUTime() -> Time {
    let mut info = thread_basic_info {
        user_time: time_value {
            seconds: 0,
            microseconds: 0,
        },
        system_time: time_value {
            seconds: 0,
            microseconds: 0,
        },
        cpu_usage: 0,
        policy: 0,
        run_state: 0,
        flags: 0,
        suspend_count: 0,
        sleep_time: 0,
    };

    let mut info_count = THREAD_BASIC_INFO_COUNT;

    let mut kern_err = thread_info(
        mach_thread_self() as thread_inspect_t,
        THREAD_BASIC_INFO as thread_flavor_t,
        &raw mut info as thread_info_t,
        &raw mut info_count,
    );

    if kern_err == KERN_SUCCESS {
        return info.user_time.seconds as Time * TIME_RESOLUTION as Time
            + info.user_time.microseconds as Time * 1000 as Time;
    } else {
        sysErrorBelch(b"getThreadCPUTime\0" as *const u8 as *const c_char);
        stg_exit(EXIT_FAILURE);
    };
}

unsafe fn getProcessCPUTime() -> Time {
    static mut checked_sysconf: c_int = 0 as c_int;

    static mut sysconf_result: c_int = 0 as c_int;

    if checked_sysconf == 0 {
        sysconf_result = sysconf(_SC_CPUTIME) as c_int;
        checked_sysconf = 1 as c_int;
    }

    if sysconf_result != -(1 as c_int) {
        return getClockTime(_CLOCK_PROCESS_CPUTIME_ID);
    }

    let mut t = rusage {
        ru_utime: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        ru_stime: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        ru_maxrss: 0,
        ru_ixrss: 0,
        ru_idrss: 0,
        ru_isrss: 0,
        ru_minflt: 0,
        ru_majflt: 0,
        ru_nswap: 0,
        ru_inblock: 0,
        ru_oublock: 0,
        ru_msgsnd: 0,
        ru_msgrcv: 0,
        ru_nsignals: 0,
        ru_nvcsw: 0,
        ru_nivcsw: 0,
    };

    getrusage(RUSAGE_SELF, &raw mut t);

    return (t.ru_utime.tv_sec + t.ru_stime.tv_sec) as Time * TIME_RESOLUTION as Time
        + (t.ru_utime.tv_usec + t.ru_stime.tv_usec) as Time * 1000 as Time;
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getMonotonicNSec() -> StgWord64 {
    return getClockTime(_CLOCK_MONOTONIC) as StgWord64;
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getProcessElapsedTime() -> Time {
    return getMonotonicNSec() as Time;
}

unsafe fn getProcessTimes(mut user: *mut Time, mut elapsed: *mut Time) {
    *user = getProcessCPUTime();
    *elapsed = getProcessElapsedTime();
}

unsafe fn getUnixEpochTime(mut sec: *mut StgWord64, mut nsec: *mut StgWord32) {
    let mut tv = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };

    gettimeofday(&raw mut tv, NULL as *mut timezone as *mut c_void);
    *sec = tv.tv_sec as StgWord64;
    *nsec = (tv.tv_usec * 1000 as __darwin_suseconds_t) as StgWord32;
}

unsafe fn getPageFaults() -> W_ {
    let mut t = rusage {
        ru_utime: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        ru_stime: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        ru_maxrss: 0,
        ru_ixrss: 0,
        ru_idrss: 0,
        ru_isrss: 0,
        ru_minflt: 0,
        ru_majflt: 0,
        ru_nswap: 0,
        ru_inblock: 0,
        ru_oublock: 0,
        ru_msgsnd: 0,
        ru_msgrcv: 0,
        ru_nsignals: 0,
        ru_nvcsw: 0,
        ru_nivcsw: 0,
    };

    getrusage(RUSAGE_SELF, &raw mut t);

    return t.ru_majflt as W_;
}
