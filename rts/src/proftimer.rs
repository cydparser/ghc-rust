use crate::ffi::rts::flags::RtsFlags;
use crate::prelude::*;

#[cfg(test)]
mod tests;

static mut do_heap_prof_ticks: bool = r#false != 0;

static mut heap_prof_timer_active: bool = r#false != 0;

static mut ticks_to_heap_profile: c_int = 0;

static mut performHeapProfile: bool = false;

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn stopProfTimer() {}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn startProfTimer() {}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn stopHeapProfTimer() {
    if RtsFlags.ProfFlags.doHeapProfile != 0 {
        ::core::intrinsics::atomic_store_relaxed(&raw mut heap_prof_timer_active, 0 as c_int != 0);
        pauseHeapProfTimer();
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn startHeapProfTimer() {
    if RtsFlags.ProfFlags.doHeapProfile != 0 {
        ::core::intrinsics::atomic_store_relaxed(&raw mut heap_prof_timer_active, 1 as c_int != 0);
        resumeHeapProfTimer();
    }
}

unsafe fn pauseHeapProfTimer() {
    ::core::intrinsics::atomic_store_relaxed(&raw mut do_heap_prof_ticks, 0 as c_int != 0);
}

unsafe fn resumeHeapProfTimer() {
    if RtsFlags.ProfFlags.doHeapProfile != 0
        && RtsFlags.ProfFlags.heapProfileIntervalTicks > 0 as uint32_t
    {
        ::core::intrinsics::atomic_store_relaxed(&raw mut do_heap_prof_ticks, 1 as c_int != 0);
    }
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn requestHeapCensus() {
    if RtsFlags.ProfFlags.doHeapProfile != 0 {
        ::core::intrinsics::atomic_store_relaxed(&raw mut performHeapProfile, 1 as c_int != 0);
    }
}

unsafe fn initProfTimer() {
    ::core::intrinsics::atomic_store_relaxed(&raw mut performHeapProfile, 0 as c_int != 0);
    ticks_to_heap_profile = RtsFlags.ProfFlags.heapProfileIntervalTicks as c_int;

    if RtsFlags.ProfFlags.startHeapProfileAtStartup {
        startHeapProfTimer();
    }
}

static mut total_ticks: uint32_t = 0 as uint32_t;

unsafe fn handleProfTick() {
    if ::core::intrinsics::atomic_load_relaxed(&raw mut do_heap_prof_ticks) as c_int != 0
        && ::core::intrinsics::atomic_load_relaxed(&raw mut heap_prof_timer_active) as c_int != 0
    {
        ticks_to_heap_profile -= 1;

        if ticks_to_heap_profile <= 0 as c_int {
            ticks_to_heap_profile = RtsFlags.ProfFlags.heapProfileIntervalTicks as c_int;
            ::core::intrinsics::atomic_store_relaxed(&raw mut performHeapProfile, 1 as c_int != 0);
        }
    }
}
