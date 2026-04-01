use crate::ffi::rts::messages::debugBelch;
use crate::ffi::rts::threads::getNumCapabilities;
use crate::ffi::rts::time::Time;
use crate::ffi::stg::types::StgWord;
use crate::prelude::*;
use crate::sm::gc_thread::{GC_THREAD_STANDING_BY, gc_threads};

unsafe fn LongGCSync(mut me: u32, mut t: Time) {
    let mut i: u32 = 0;
    i = 0;

    while i < getNumCapabilities() as u32 {
        if i != me
            && (&raw mut (**gc_threads.offset(i as isize)).wakeup).load(Ordering::SeqCst)
                == GC_THREAD_STANDING_BY as StgWord
        {
            debugBelch(
                c"Warning: slow GC sync: still waiting for cap %d\n".as_ptr(),
                i,
            );
        }

        i = i.wrapping_add(1);
    }
}

unsafe fn LongGCSyncEnd(mut t: Time) {
    debugBelch(c"Warning: waited %lluus for GC sync\n".as_ptr(), t / 1000);
}
