use crate::ffi::rts::messages::debugBelch;
use crate::ffi::rts::time::Time;
use crate::prelude::*;

unsafe fn LongGCSync(mut me: u32, mut t: Time) {}

unsafe fn LongGCSyncEnd(mut t: Time) {
    debugBelch(c"Warning: waited %lluus for GC sync\n".as_ptr(), t / 1000);
}
