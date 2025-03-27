use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use libc::{clockid_t, pid_t, pthread_cond_t, pthread_key_t, pthread_mutex_t, pthread_t};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use std::mem::transmute;
#[cfg(feature = "tracing")]
use tracing::instrument;
#[cfg(test)]
mod tests;

#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::core::marker::PhantomData<T>, [T; 0]);

impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::core::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::core::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::core::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}

impl<T> ::core::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}

pub(crate) const _HNF: u32 = 1;

pub(crate) const _BTM: u32 = 2;

pub(crate) const _NS: u32 = 4;

pub(crate) const _THU: u32 = 8;

pub(crate) const _MUT: u32 = 16;

pub(crate) const _UPT: u32 = 32;

pub(crate) const _SRT: u32 = 64;

pub(crate) const _IND: u32 = 128;

pub(crate) const _FRM: u32 = 256;

#[repr(C)]
pub(crate) struct StgProfInfo {
    pub closure_type_off: StgHalfInt,
    pub __pad_closure_type_off: StgHalfWord,
    pub closure_desc_off: StgHalfInt,
    pub __pad_closure_desc_off: StgHalfWord,
}

#[cfg(feature = "sys")]
impl From<StgProfInfo> for sys::StgProfInfo {
    fn from(x: StgProfInfo) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for StgProfInfo {
    fn arbitrary(g: &mut Gen) -> Self {
        StgProfInfo {
            closure_type_off: Arbitrary::arbitrary(g),
            __pad_closure_type_off: Arbitrary::arbitrary(g),
            closure_desc_off: Arbitrary::arbitrary(g),
            __pad_closure_desc_off: Arbitrary::arbitrary(g),
        }
    }
}

static closure_flags: [StgWord16; 0usize] = sys::closure_flags;

#[repr(C)]
pub(crate) struct StgLargeBitmap_ {
    pub size: StgWord,
    pub bitmap: __IncompleteArrayField<StgWord>,
}

#[cfg(feature = "sys")]
impl From<StgLargeBitmap_> for sys::StgLargeBitmap_ {
    fn from(x: StgLargeBitmap_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for StgLargeBitmap_ {
    fn arbitrary(g: &mut Gen) -> Self {
        StgLargeBitmap_ {
            size: Arbitrary::arbitrary(g),
            bitmap: Arbitrary::arbitrary(g),
        }
    }
}

pub type StgLargeBitmap = StgLargeBitmap_;

#[repr(C)]
pub(crate) union StgClosureInfo {
    pub payload: ::core::mem::ManuallyDrop<StgClosureInfo__bindgen_ty_1>,
    pub bitmap: ::core::mem::ManuallyDrop<StgWord>,
    pub large_bitmap_offset: ::core::mem::ManuallyDrop<StgHalfInt>,
    pub __pad_large_bitmap_offset: ::core::mem::ManuallyDrop<StgHalfWord>,
    pub selector_offset: ::core::mem::ManuallyDrop<StgWord>,
}

#[cfg(feature = "sys")]
impl From<StgClosureInfo> for sys::StgClosureInfo {
    fn from(x: StgClosureInfo) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for StgClosureInfo {
    fn arbitrary(g: &mut Gen) -> Self {
        match Arbitrary::arbitrary::<usize>(g) % 5usize {
            0 => StgClosureInfo {
                payload: Arbitrary::arbitrary(g),
            },
            1 => StgClosureInfo {
                bitmap: Arbitrary::arbitrary(g),
            },
            2 => StgClosureInfo {
                large_bitmap_offset: Arbitrary::arbitrary(g),
            },
            3 => StgClosureInfo {
                __pad_large_bitmap_offset: Arbitrary::arbitrary(g),
            },
            4 => StgClosureInfo {
                selector_offset: Arbitrary::arbitrary(g),
            },
        }
    }
}

#[repr(C)]
pub(crate) struct StgClosureInfo__bindgen_ty_1 {
    pub ptrs: StgHalfWord,
    pub nptrs: StgHalfWord,
}

#[cfg(feature = "sys")]
impl From<StgClosureInfo__bindgen_ty_1> for sys::StgClosureInfo__bindgen_ty_1 {
    fn from(x: StgClosureInfo__bindgen_ty_1) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for StgClosureInfo__bindgen_ty_1 {
    fn arbitrary(g: &mut Gen) -> Self {
        StgClosureInfo__bindgen_ty_1 {
            ptrs: Arbitrary::arbitrary(g),
            nptrs: Arbitrary::arbitrary(g),
        }
    }
}

pub(crate) type StgSRTField = StgHalfInt;

#[repr(C)]
pub struct StgInfoTable_ {
    pub layout: StgClosureInfo,
    pub type_: StgHalfWord,
    pub srt: StgSRTField,
    pub code: __IncompleteArrayField<StgCode>,
}

#[cfg(feature = "sys")]
impl From<StgInfoTable_> for sys::StgInfoTable_ {
    fn from(x: StgInfoTable_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for StgInfoTable_ {
    fn arbitrary(g: &mut Gen) -> Self {
        StgInfoTable_ {
            layout: Arbitrary::arbitrary(g),
            type_: Arbitrary::arbitrary(g),
            srt: Arbitrary::arbitrary(g),
            code: Arbitrary::arbitrary(g),
        }
    }
}

pub(crate) type StgInfoTablePtr = *mut StgInfoTable_;

#[repr(C)]
pub(crate) struct StgFunInfoExtraRev_ {
    pub slow_apply_offset: StgHalfInt,
    pub __pad_slow_apply_offset: StgHalfWord,
    pub b: StgFunInfoExtraRev___bindgen_ty_1,
    pub fun_type: StgHalfWord,
    pub arity: StgHalfWord,
}

#[cfg(feature = "sys")]
impl From<StgFunInfoExtraRev_> for sys::StgFunInfoExtraRev_ {
    fn from(x: StgFunInfoExtraRev_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for StgFunInfoExtraRev_ {
    fn arbitrary(g: &mut Gen) -> Self {
        StgFunInfoExtraRev_ {
            slow_apply_offset: Arbitrary::arbitrary(g),
            __pad_slow_apply_offset: Arbitrary::arbitrary(g),
            b: Arbitrary::arbitrary(g),
            fun_type: Arbitrary::arbitrary(g),
            arity: Arbitrary::arbitrary(g),
        }
    }
}

#[repr(C)]
pub(crate) union StgFunInfoExtraRev___bindgen_ty_1 {
    pub bitmap: ::core::mem::ManuallyDrop<StgWord>,
    pub bitmap_offset: ::core::mem::ManuallyDrop<StgHalfInt>,
    pub __pad_bitmap_offset: ::core::mem::ManuallyDrop<StgHalfWord>,
}

#[cfg(feature = "sys")]
impl From<StgFunInfoExtraRev___bindgen_ty_1> for sys::StgFunInfoExtraRev___bindgen_ty_1 {
    fn from(x: StgFunInfoExtraRev___bindgen_ty_1) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for StgFunInfoExtraRev___bindgen_ty_1 {
    fn arbitrary(g: &mut Gen) -> Self {
        match Arbitrary::arbitrary::<usize>(g) % 3usize {
            0 => StgFunInfoExtraRev___bindgen_ty_1 {
                bitmap: Arbitrary::arbitrary(g),
            },
            1 => StgFunInfoExtraRev___bindgen_ty_1 {
                bitmap_offset: Arbitrary::arbitrary(g),
            },
            2 => StgFunInfoExtraRev___bindgen_ty_1 {
                __pad_bitmap_offset: Arbitrary::arbitrary(g),
            },
        }
    }
}

pub type StgFunInfoExtraRev = StgFunInfoExtraRev_;

#[repr(C)]
pub(crate) struct StgFunInfoExtraFwd_ {
    pub fun_type: StgHalfWord,
    pub arity: StgHalfWord,
    pub srt: *mut StgClosure,
    pub b: StgFunInfoExtraFwd___bindgen_ty_1,
    pub slow_apply: StgFun,
}

#[cfg(feature = "sys")]
impl From<StgFunInfoExtraFwd_> for sys::StgFunInfoExtraFwd_ {
    fn from(x: StgFunInfoExtraFwd_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for StgFunInfoExtraFwd_ {
    fn arbitrary(g: &mut Gen) -> Self {
        StgFunInfoExtraFwd_ {
            fun_type: Arbitrary::arbitrary(g),
            arity: Arbitrary::arbitrary(g),
            srt: Arbitrary::arbitrary(g),
            b: Arbitrary::arbitrary(g),
            slow_apply: Arbitrary::arbitrary(g),
        }
    }
}

#[repr(C)]
pub(crate) union StgFunInfoExtraFwd___bindgen_ty_1 {
    pub bitmap: ::core::mem::ManuallyDrop<StgWord>,
}

#[cfg(feature = "sys")]
impl From<StgFunInfoExtraFwd___bindgen_ty_1> for sys::StgFunInfoExtraFwd___bindgen_ty_1 {
    fn from(x: StgFunInfoExtraFwd___bindgen_ty_1) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for StgFunInfoExtraFwd___bindgen_ty_1 {
    fn arbitrary(g: &mut Gen) -> Self {
        match Arbitrary::arbitrary::<usize>(g) % 1usize {
            0 => StgFunInfoExtraFwd___bindgen_ty_1 {
                bitmap: Arbitrary::arbitrary(g),
            },
        }
    }
}

pub type StgFunInfoExtraFwd = StgFunInfoExtraFwd_;

#[repr(C)]
pub struct StgFunInfoTable {
    pub f: StgFunInfoExtraRev,
    pub i: StgInfoTable,
}

#[cfg(feature = "sys")]
impl From<StgFunInfoTable> for sys::StgFunInfoTable {
    fn from(x: StgFunInfoTable) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for StgFunInfoTable {
    fn arbitrary(g: &mut Gen) -> Self {
        StgFunInfoTable {
            f: Arbitrary::arbitrary(g),
            i: Arbitrary::arbitrary(g),
        }
    }
}

#[unsafe(no_mangle)]
pub static stg_arg_bitmaps: [StgWord; 0usize] = sys::stg_arg_bitmaps;

#[repr(C)]
pub struct StgRetInfoTable {
    pub i: StgInfoTable,
}

#[cfg(feature = "sys")]
impl From<StgRetInfoTable> for sys::StgRetInfoTable {
    fn from(x: StgRetInfoTable) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for StgRetInfoTable {
    fn arbitrary(g: &mut Gen) -> Self {
        StgRetInfoTable {
            i: Arbitrary::arbitrary(g),
        }
    }
}

#[repr(C)]
pub(crate) struct StgThunkInfoTable_ {
    pub i: StgInfoTable,
}

#[cfg(feature = "sys")]
impl From<StgThunkInfoTable_> for sys::StgThunkInfoTable_ {
    fn from(x: StgThunkInfoTable_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for StgThunkInfoTable_ {
    fn arbitrary(g: &mut Gen) -> Self {
        StgThunkInfoTable_ {
            i: Arbitrary::arbitrary(g),
        }
    }
}

pub(crate) type StgThunkInfoTable = StgThunkInfoTable_;

#[repr(C)]
pub(crate) struct StgConInfoTable_ {
    pub con_desc: StgHalfInt,
    pub __pad_con_desc: StgHalfWord,
    pub i: StgInfoTable,
}

#[cfg(feature = "sys")]
impl From<StgConInfoTable_> for sys::StgConInfoTable_ {
    fn from(x: StgConInfoTable_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for StgConInfoTable_ {
    fn arbitrary(g: &mut Gen) -> Self {
        StgConInfoTable_ {
            con_desc: Arbitrary::arbitrary(g),
            __pad_con_desc: Arbitrary::arbitrary(g),
            i: Arbitrary::arbitrary(g),
        }
    }
}

pub type StgConInfoTable = StgConInfoTable_;
