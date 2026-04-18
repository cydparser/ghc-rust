#![expect(unused_imports)]

#[cfg(feature = "loom")]
pub use loom::cell::{Cell, UnsafeCell};
#[cfg(feature = "loom")]
pub use loom::sync::atomic::{
    AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicPtr, AtomicU8,
    AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering,
};
#[cfg(feature = "loom")]
pub use loom::sync::{Arc, Mutex, RwLock};
#[cfg(feature = "loom")]
pub use loom::{atomic, mpsc, thread, thread_local};

#[cfg(not(feature = "loom"))]
pub use std::cell::Cell;
#[cfg(not(feature = "loom"))]
pub use std::sync::atomic::{
    AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicPtr, AtomicU8,
    AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering,
};
#[cfg(not(feature = "loom"))]
pub use std::sync::{Arc, Mutex, RwLock};
#[cfg(not(feature = "loom"))]
pub use std::sync::{atomic, mpsc};
#[cfg(not(feature = "loom"))]
pub use std::{thread, thread_local};

#[derive(Debug)]
pub(crate) struct UnsafeCell<T>(std::cell::UnsafeCell<T>);

#[cfg(not(feature = "loom"))]
impl<T> UnsafeCell<T> {
    #[inline]
    pub fn new(data: T) -> UnsafeCell<T> {
        UnsafeCell(std::cell::UnsafeCell::new(data))
    }

    #[inline]
    pub fn into_inner(self) -> T {
        self.0.into_inner()
    }

    #[inline]
    pub fn with<R>(&self, f: impl FnOnce(*const T) -> R) -> R {
        f(self.0.get())
    }

    #[inline]
    pub fn with_mut<R>(&self, f: impl FnOnce(*mut T) -> R) -> R {
        f(self.0.get())
    }

    #[inline]
    pub fn get(&self) -> *const T {
        self.0.get()
    }

    #[inline]
    pub fn get_mut(&self) -> *mut T {
        self.0.get()
    }
}
