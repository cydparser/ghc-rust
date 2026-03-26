use crate::linker_internals::ObjectCode;
use crate::prelude::*;

unsafe fn ocFlushInstructionCache(mut oc: *mut ObjectCode) {}
