use crate::ffi::rts::messages::barf;
use crate::ffi::stg::smp::cas;
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
    let mut t: StgWord = (*q).top as StgWord;
    let mut b: StgWord = (*q).bottom as StgWord;
    let mut n: StgInt = b as StgInt - t as StgInt;

    return if n > 0 as StgInt { n } else { 0 as StgInt };
}

#[inline]
pub(crate) unsafe fn looksEmptyWSDeque(mut q: *mut WSDeque) -> bool {
    return dequeElements(q) <= 0 as StgInt;
}

#[inline]
pub(crate) unsafe fn discardElements(mut q: *mut WSDeque) {
    (*q).top = (*q).bottom;
}

#[inline]
unsafe fn cas_top(mut q: *mut WSDeque, mut old: StgInt, mut new: StgInt) -> bool {
    return old as StgWord
        == cas(
            &raw mut (*q).top as StgPtr as StgVolatilePtr,
            old as StgWord,
            new as StgWord,
        );
}

unsafe fn roundUp2(mut val: StgWord) -> StgWord {
    let mut rounded: StgWord = 1 as StgWord;

    if val == 0 as StgWord {
        barf(b"DeQue,roundUp2: invalid size 0 requested\0" as *const u8 as *const c_char);
    }

    loop {
        rounded = rounded << 1 as c_int;
        val = val >> 1 as c_int;

        if !(0 as StgWord != val) {
            break;
        }
    }

    return rounded;
}

unsafe fn newWSDeque(mut size: uint32_t) -> *mut WSDeque {
    let mut realsize: StgWord = 0;
    let mut q = null_mut::<WSDeque>();
    realsize = roundUp2(size as StgWord);

    q = stgMallocBytes(
        size_of::<WSDeque>() as size_t,
        b"newWSDeque\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut WSDeque;

    (*q).elements = stgMallocBytes(
        realsize.wrapping_mul(size_of::<StgClosurePtr>() as StgWord) as size_t,
        b"newWSDeque:data space\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut *mut c_void;

    (*q).size = realsize as StgInt;
    (*q).moduloSize = realsize.wrapping_sub(1 as StgWord);
    (*q).top = 0 as StgInt;
    (*q).bottom = 0 as StgInt;

    return q;
}

unsafe fn freeWSDeque(mut q: *mut WSDeque) {
    stgFree((*q).elements as *mut c_void);
    stgFree(q as *mut c_void);
}

unsafe fn popWSDeque(mut q: *mut WSDeque) -> *mut c_void {
    let mut b: StgInt = (*q).bottom - 1 as StgInt;
    (*q).bottom = b;

    let mut t: StgInt = (*q).top;
    let mut result = null_mut::<c_void>();

    if t <= b {
        result = *(*q)
            .elements
            .offset((b as StgWord & (*q).moduloSize) as isize);

        if t == b {
            if !cas_top(q, t, t + 1 as StgInt) {
                result = NULL;
            }

            (*q).bottom = b + 1 as StgInt;
        }
    } else {
        result = NULL;
        (*q).bottom = b + 1 as StgInt;
    }

    return result;
}

unsafe fn stealWSDeque_(mut q: *mut WSDeque) -> *mut c_void {
    let mut t: StgInt = (*q).top;
    let mut b: StgInt = (*q).bottom;
    let mut result = NULL;

    if t < b {
        result = *(*q).elements.offset((t % (*q).size) as isize);

        if !cas_top(q, t, t + 1 as StgInt) {
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
    let mut b: StgInt = (*q).bottom;
    let mut t: StgInt = (*q).top;

    if b - t > (*q).size - 1 as StgInt {
        return r#false != 0;
    }

    let ref mut fresh5 = *(*q)
        .elements
        .offset((b as StgWord & (*q).moduloSize) as isize);
    *fresh5 = elem;
    (*q).bottom = b + 1 as StgInt;

    return r#true != 0;
}
