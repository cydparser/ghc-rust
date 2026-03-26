use crate::ffi::rts::messages::debugBelch;
use crate::ffi::rts::time::Time;
use crate::prelude::*;

unsafe fn LongGCSync(mut me: uint32_t, mut t: Time) {}

unsafe fn LongGCSyncEnd(mut t: Time) {
    debugBelch(
        b"Warning: waited %lluus for GC sync\n\0" as *const u8 as *const c_char,
        t / 1000 as Time,
    );
}
