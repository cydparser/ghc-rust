pub use crate::posix::os_threads::{
    Condition, KernelThreadId, Mutex, OSThreadId, OSThreadProc, broadcastCondition, createOSThread,
    forkOS_createThread, getNumberOfProcessors, initCondition, initMutex, shutdownThread,
    waitCondition,
};
pub use crate::win32::os_threads::{
    Condition, KernelThreadId, Mutex, OSThreadId, OSThreadProc, broadcastCondition, createOSThread,
    forkOS_createThread, getNumberOfProcessors, initCondition, initMutex, shutdownThread,
    waitCondition,
};
