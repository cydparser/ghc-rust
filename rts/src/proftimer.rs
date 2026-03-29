use crate::ffi::rts::flags::RtsFlags;
use crate::prelude::*;

#[cfg(test)]
mod tests;

static mut do_heap_prof_ticks: bool = false;

static mut heap_prof_timer_active: bool = false;

static mut ticks_to_heap_profile: i32 = 0;

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
        (&raw mut heap_prof_timer_active).store(0 != 0, Ordering::Relaxed);
        pauseHeapProfTimer();
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn startHeapProfTimer() {
    if RtsFlags.ProfFlags.doHeapProfile != 0 {
        (&raw mut heap_prof_timer_active).store(1 != 0, Ordering::Relaxed);
        resumeHeapProfTimer();
    }
}

unsafe fn pauseHeapProfTimer() {
    (&raw mut do_heap_prof_ticks).store(0 != 0, Ordering::Relaxed);
}

unsafe fn resumeHeapProfTimer() {
    if RtsFlags.ProfFlags.doHeapProfile != 0 && RtsFlags.ProfFlags.heapProfileIntervalTicks > 0 {
        (&raw mut do_heap_prof_ticks).store(1 != 0, Ordering::Relaxed);
    }
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn requestHeapCensus() {
    if RtsFlags.ProfFlags.doHeapProfile != 0 {
        (&raw mut performHeapProfile).store(1 != 0, Ordering::Relaxed);
    }
}

unsafe fn initProfTimer() {
    (&raw mut performHeapProfile).store(0 != 0, Ordering::Relaxed);
    ticks_to_heap_profile = RtsFlags.ProfFlags.heapProfileIntervalTicks as i32;

    if RtsFlags.ProfFlags.startHeapProfileAtStartup {
        startHeapProfTimer();
    }
}

static mut total_ticks: u32 = 0;

unsafe fn handleProfTick() {
    if (&raw mut do_heap_prof_ticks).load(Ordering::Relaxed) as i32 != 0
        && (&raw mut heap_prof_timer_active).load(Ordering::Relaxed) as i32 != 0
    {
        ticks_to_heap_profile -= 1;

        if ticks_to_heap_profile <= 0 {
            ticks_to_heap_profile = RtsFlags.ProfFlags.heapProfileIntervalTicks as i32;
            (&raw mut performHeapProfile).store(1 != 0, Ordering::Relaxed);
        }
    }
}
