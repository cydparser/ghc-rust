pub use crate::posix::os_threads::{
    Condition, KernelThreadId, Mutex, OSThreadId, OSThreadProc, broadcastCondition, createOSThread,
    forkOS_createThread, getNumberOfProcessors, initCondition, initMutex, shutdownThread,
    waitCondition,
};
use crate::prelude::*;
pub use crate::win32::os_threads::{
    Condition, KernelThreadId, Mutex, OSThreadId, OSThreadProc, broadcastCondition, createOSThread,
    forkOS_createThread, getNumberOfProcessors, initCondition, initMutex, shutdownThread,
    waitCondition,
};

/// cbindgen:no-export
#[repr(C)]
pub struct TODO_ {
    _unused: [u8; 0],
}
