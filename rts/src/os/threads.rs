#[cfg(unix)]
pub(crate) use crate::posix::os_threads::{
    Condition, KernelThreadId, Mutex, OS_ACQUIRE_LOCK, OS_ASSERT_LOCK_HELD, OS_TRY_ACQUIRE_LOCK,
    OSThreadId, OSThreadProc, broadcastCondition, closeMutex, createOSThread, forkOS_createThread,
    getNumberOfProcessors, initCondition, initMutex, osThreadId, osThreadIsAlive, shutdownThread,
    signalCondition, waitCondition, yieldThread,
};

macro_rules! ACQUIRE_LOCK {
    ($l:expr) => {
        OS_ACQUIRE_LOCK(l)
    };
}

pub(crate) use ACQUIRE_LOCK;

macro_rules! TRY_ACQUIRE_LOCK {
    ($l:expr) => {
        OS_TRY_ACQUIRE_LOCK(l)
    };
}

pub(crate) use TRY_ACQUIRE_LOCK;

macro_rules! RELEASE_LOCK {
    ($l:expr) => {
        OS_RELEASE_LOCK(l)
    };
}

pub(crate) use RELEASE_LOCK;

macro_rules! ASSERT_LOCK_HELD {
    ($l:expr) => {
        $crate::os::threads::OS_ASSERT_LOCK_HELD!($l)
    };
}

pub(crate) use ASSERT_LOCK_HELD;
