use crate::ffi::rts::constants::SPIN_COUNT;
use crate::ffi::rts::os_threads::yieldThread;
use crate::ffi::rts::spin_lock::{SpinLock, SpinLock_};
use crate::ffi::stg::smp::cas;
use crate::ffi::stg::smp::{busy_wait_nop, cas};
use crate::ffi::stg::types::{StgVolatilePtr, StgWord, StgWord32, StgWord64};
use crate::ffi::stg::types::{StgVolatilePtr, StgWord, StgWord32, StgWord64};
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(compiler)]
pub type SpinLock = SpinLock_;

#[ffi(testsuite)]
#[repr(C)]
#[cfg_attr(test, derive(Clone))]
pub struct SpinLock_ {
    pub lock: StgWord,
    pub spin: StgWord64,
    pub r#yield: StgWord64,
}

#[cfg(test)]
impl Arbitrary for SpinLock_ {
    fn arbitrary(g: &mut Gen) -> Self {
        SpinLock_ {
            lock: Arbitrary::arbitrary(g),
            spin: Arbitrary::arbitrary(g),
            yield_: Arbitrary::arbitrary(g),
        }
    }
}

#[inline]
pub(crate) unsafe fn ACQUIRE_SPIN_LOCK(mut p: *mut SpinLock) {
    let mut r: StgWord32 = cas(&raw mut (*p).lock as StgVolatilePtr, 1, 0) as StgWord32;

    if (r == 0) as i32 as i64 != 0 {
        acquire_spin_lock_slow_path(p);
    }
}

#[inline]
pub(crate) unsafe fn RELEASE_SPIN_LOCK(mut p: *mut SpinLock) {
    (&raw mut (*p).lock).store(1, Ordering::Release);
}

#[inline]
pub(crate) unsafe fn initSpinLock(mut p: *mut SpinLock) {
    (*p).spin = 0;
    (*p).r#yield = 0;
    (&raw mut (*p).lock).store(1, Ordering::Release);
}

#[inline(always)]
unsafe fn try_acquire_spin_slow_path(mut p: *mut SpinLock) -> bool {
    let mut r: StgWord = 0;
    r = cas(&raw mut (*p).lock as StgVolatilePtr, 1, 0);

    if r == 0 {
        let fresh10 = &raw mut (*p).spin;
        let fresh11 = 1;
        (fresh10).xadd(fresh11, Ordering::Relaxed) + fresh11;
    }

    return r != 0;
}

unsafe fn acquire_spin_lock_slow_path(mut p: *mut SpinLock) {
    loop {
        let mut i: u32 = 0;

        while i < SPIN_COUNT as u32 {
            if try_acquire_spin_slow_path(p) {
                return;
            }

            busy_wait_nop();
            i = i.wrapping_add(1);
        }

        let fresh8 = &raw mut (*p).r#yield;
        let fresh9 = 1;
        (fresh8).xadd(fresh9, Ordering::Relaxed) + fresh9;
        yieldThread();
    }
}
