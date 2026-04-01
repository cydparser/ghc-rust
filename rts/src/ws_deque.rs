use crate::ffi::rts::_assertFail;
use crate::ffi::rts::messages::barf;
use crate::ffi::stg::smp::cas_seq_cst_relaxed;
use crate::ffi::stg::types::{StgInt, StgPtr, StgVolatilePtr, StgWord};
use crate::ffi::stg::types::{StgInt, StgWord};
use crate::prelude::*;
use crate::rts_utils::{stgFree, stgMallocBytes};
use crate::ws_deque::{WSDeque, WSDeque_, dequeElements, looksEmptyWSDeque};

/// cbindgen:no-export
pub(crate) struct WSDeque_ {
    pub(crate) size: StgInt,
    pub(crate) moduloSize: StgWord,
    pub(crate) top: StgInt,
    pub(crate) bottom: StgInt,
    pub(crate) elements: *mut *mut c_void,
}

pub(crate) type WSDeque = WSDeque_;

#[inline]
pub(crate) unsafe fn dequeElements(mut q: *mut WSDeque) -> StgInt {
    let mut t: StgWord = (&raw mut (*q).top).load(Ordering::Acquire) as StgWord;
    let mut b: StgWord = (&raw mut (*q).bottom).load(Ordering::Acquire) as StgWord;
    let mut n: StgInt = b as StgInt - t as StgInt;

    return if n > 0 { n } else { 0 };
}

#[inline]
pub(crate) unsafe fn looksEmptyWSDeque(mut q: *mut WSDeque) -> bool {
    return dequeElements(q) <= 0;
}

#[inline]
pub(crate) unsafe fn discardElements(mut q: *mut WSDeque) {
    (&raw mut (*q).top).store(
        (&raw mut (*q).bottom).load(Ordering::Relaxed),
        Ordering::Relaxed,
    );
}

#[inline]
pub(crate) unsafe fn cas_top(mut q: *mut WSDeque, mut old: StgInt, mut new: StgInt) -> bool {
    return old as StgWord
        == cas_seq_cst_relaxed(
            &raw mut (*q).top as StgPtr as StgVolatilePtr,
            old as StgWord,
            new as StgWord,
        );
}

unsafe fn roundUp2(mut val: StgWord) -> StgWord {
    let mut rounded: StgWord = 1;

    if val == 0 {
        barf(c"DeQue,roundUp2: invalid size 0 requested".as_ptr());
    }

    loop {
        rounded = rounded << 1;
        val = val >> 1;

        if !(0 != val) {
            break;
        }
    }

    return rounded;
}

unsafe fn newWSDeque(mut size: u32) -> *mut WSDeque {
    let mut realsize: StgWord = 0;
    let mut q = null_mut::<WSDeque>();
    realsize = roundUp2(size as StgWord);
    q = stgMallocBytes(size_of::<WSDeque>() as usize, c"newWSDeque".as_ptr()) as *mut WSDeque;

    (*q).elements = stgMallocBytes(
        realsize.wrapping_mul(size_of::<StgClosurePtr>() as StgWord) as usize,
        c"newWSDeque:data space".as_ptr(),
    ) as *mut *mut c_void;

    (*q).size = realsize as StgInt;
    (*q).moduloSize = realsize.wrapping_sub(1 as StgWord);
    (*q).top = 0;
    (&raw mut (*q).bottom).store(0, Ordering::Release);

    if ((*q).size > 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/WSDeque.c".as_ptr(), 104);
    }

    if !(&raw mut (*q).elements).load(Ordering::Relaxed).is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/WSDeque.c".as_ptr(), 104);
    }

    if (!((*q).elements.offset(0) as *mut *mut c_void)
        .load(Ordering::Relaxed)
        .is_null()
        || 1 != 0) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/WSDeque.c".as_ptr(), 104);
    }

    if (!((*q).elements.offset(((*q).size - 1) as isize) as *mut *mut c_void)
        .load(Ordering::Relaxed)
        .is_null()
        || 1 != 0) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/WSDeque.c".as_ptr(), 104);
    }

    return q;
}

unsafe fn freeWSDeque(mut q: *mut WSDeque) {
    stgFree((*q).elements as *mut c_void);
    stgFree(q as *mut c_void);
}

unsafe fn popWSDeque(mut q: *mut WSDeque) -> *mut c_void {
    let mut b: StgInt = (&raw mut (*q).bottom).load(Ordering::Relaxed) - 1;
    (&raw mut (*q).bottom).store(b, Ordering::Relaxed);
    ::std::sync::atomic::fence(::std::sync::atomic::Ordering::SeqCst);

    let mut t: StgInt = (&raw mut (*q).top).load(Ordering::Relaxed);
    let mut result = null_mut::<c_void>();

    if t <= b {
        result = ((*q)
            .elements
            .offset((b as StgWord & (*q).moduloSize) as isize)
            as *mut *mut c_void)
            .load(Ordering::Relaxed);

        if t == b {
            if !cas_top(q, t, t + 1) {
                result = NULL;
            }

            (&raw mut (*q).bottom).store(b + 1, Ordering::Relaxed);
        }
    } else {
        result = NULL;
        (&raw mut (*q).bottom).store(b + 1, Ordering::Relaxed);
    }

    return result;
}

unsafe fn stealWSDeque_(mut q: *mut WSDeque) -> *mut c_void {
    let mut t: StgInt = (&raw mut (*q).top).load(Ordering::Acquire);
    ::std::sync::atomic::fence(::std::sync::atomic::Ordering::SeqCst);

    let mut b: StgInt = (&raw mut (*q).bottom).load(Ordering::Acquire);
    let mut result = NULL;

    if t < b {
        result = ((*q).elements.offset((t % (*q).size) as isize) as *mut *mut c_void)
            .load(Ordering::Relaxed);

        if !cas_top(q, t, t + 1) {
            return NULL;
        }
    }

    return result;
}

unsafe fn stealWSDeque(mut q: *mut WSDeque) -> *mut c_void {
    let mut stolen = null_mut::<c_void>();

    loop {
        stolen = stealWSDeque_(q);

        if !(stolen.is_null() && !looksEmptyWSDeque(q)) {
            break;
        }
    }

    return stolen;
}

unsafe fn pushWSDeque(mut q: *mut WSDeque, mut elem: *mut c_void) -> bool {
    let mut b: StgInt = (&raw mut (*q).bottom).load(Ordering::Acquire);
    let mut t: StgInt = (&raw mut (*q).top).load(Ordering::Acquire);

    if b - t > (*q).size - 1 {
        return false;
    }

    ((*q)
        .elements
        .offset((b as StgWord & (*q).moduloSize) as isize) as *mut *mut c_void)
        .store(elem, Ordering::Relaxed);
    ::std::sync::atomic::fence(::std::sync::atomic::Ordering::Release);
    (&raw mut (*q).bottom).store(b + 1, Ordering::Relaxed);

    return true;
}
