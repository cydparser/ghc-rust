//! = Work-stealing Deque data structure

use std::alloc::Layout;

use crate::prelude::*;
use crate::rts_messages::rts_assert;
use crate::rts_utils::{stg_alloc, stg_alloc_layout, stg_free, stg_free_layout};
use crate::stg::types::{StgInt, StgWord};

/// cbindgen:no-export
pub(crate) struct WSDeque<T> {
    /// Size of elements array. Used for modulo calculation: we round up
    /// to powers of 2 and use the dyadic log (modulo == bitwise &)
    pub(crate) size: isize,
    /// Bitmask for modulo
    pub(crate) moduloSize: isize,
    /// top, index where multiple readers steal() (protected by a cas)
    pub(crate) top: AtomicIsize,
    /// bottom, index of next free place where one writer can push
    /// elements. This happens unsynchronised.
    pub(crate) bottom: AtomicIsize,
    /// The elements array
    pub(crate) elements: *mut AtomicPtr<T>,
}

macro_rules! assert_wsdeque_invariants {
    ($p:expr) => {
        rts_assert!($p.size > 0);
        rts_assert!(!$p.elements.is_null());
        rts_assert!(
            !(*$p.elements.offset(0)).load(Relaxed).is_null() || true // TODO(rust): The C impl does the same
        );
        rts_assert!(
            !(*$p.elements.offset($p.size - 1)).load(Relaxed).is_null() || true // TODO(rust): The C impl does the same
        );
    };
}

#[inline]
pub(crate) fn dequeElements<T>(q: &WSDeque<T>) -> isize {
    let t = q.top.load(Acquire);
    let b = q.bottom.load(Acquire);
    let n = b - t;

    0.max(n)
}

#[inline]
pub(crate) fn looksEmptyWSDeque<T>(q: &WSDeque<T>) -> bool {
    return dequeElements(q) <= 0;
}

#[inline]
pub(crate) fn discardElements<T>(q: &WSDeque<T>) {
    q.top.store(q.bottom.load(Relaxed), Relaxed);
}

#[inline]
pub(crate) fn cas_top<T>(q: &WSDeque<T>, old: isize, new: isize) -> bool {
    q.top.compare_exchange(old, new, SeqCst, Relaxed).is_ok()
}

unsafe fn roundUp2(mut val: usize) -> StgWord {
    rts_assert!(val > 0);

    let mut rounded: StgWord = 1;

    while val > 0 {
        rounded <<= 1;
        val >>= 1;
    }

    rounded
}

unsafe fn newWSDeque<T>(size: usize) -> *mut WSDeque<T> {
    let realsize = roundUp2(size);
    let q = stg_alloc::<WSDeque<T>>(c"newWSDeque");

    (*q).elements = stg_alloc_layout(
        Layout::array::<AtomicPtr<T>>(realsize).unwrap(),
        c"newWSDeque:data space",
    )
    .cast();

    (*q).size = realsize as isize;
    (*q).moduloSize = realsize as isize - 1;
    (*q).top = AtomicIsize::new(0);
    (*q).bottom.store(0, Release);

    assert_wsdeque_invariants!(*q);

    q
}

unsafe fn freeWSDeque<T>(q: *mut WSDeque<T>) {
    stg_free_layout(
        (*q).elements.cast(),
        Layout::array::<AtomicPtr<T>>((*q).size as usize).unwrap(),
    );
    stg_free(q);
}

fn popWSDeque<T>(q: &WSDeque<T>) -> *mut T {
    let b: StgInt = (q.bottom).load(Relaxed) - 1;
    ((*q).bottom).store(b, Relaxed);
    fence(SeqCst);

    let t: StgInt = ((*q).top).load(Relaxed);
    let mut result;

    if t <= b {
        result = unsafe { (*q.elements.offset(b & q.moduloSize)).load(Relaxed) };

        if t == b {
            if !cas_top(q, t, t + 1) {
                result = null_mut();
            }

            (q.bottom).store(b + 1, Relaxed);
        }
    } else {
        result = null_mut();
        (q.bottom).store(b + 1, Relaxed);
    }

    result
}

fn stealWSDeque_<T>(q: &WSDeque<T>) -> *mut T {
    let t: StgInt = q.top.load(Acquire);
    fence(SeqCst);

    let b: StgInt = q.bottom.load(Acquire);
    let mut result = null_mut();

    if t < b {
        result = unsafe { (*q.elements.offset(t % q.size)).load(Relaxed) };

        if !cas_top(q, t, t + 1) {
            return null_mut();
        }
    }

    result
}

fn stealWSDeque<T>(q: &WSDeque<T>) -> *mut T {
    let mut stolen;

    loop {
        stolen = stealWSDeque_(q);

        if !(stolen.is_null() && !looksEmptyWSDeque(q)) {
            break;
        }
    }

    stolen
}

fn pushWSDeque<T>(q: &WSDeque<T>, elem: *mut T) -> bool {
    let b = q.bottom.load(Acquire);
    let t = q.top.load(Acquire);

    if b - t > q.size - 1 {
        return false;
    }

    unsafe { (*q.elements.offset(b & q.moduloSize)).store(elem, Relaxed) };
    fence(Release);
    q.bottom.store(b + 1, Relaxed);

    true
}
