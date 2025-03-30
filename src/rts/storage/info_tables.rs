use std::mem::transmute;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

use crate::{
    bindgen::__IncompleteArrayField,
    rts::types::{StgClosure, StgInfoTable},
    stg::types::{StgCode, StgFun, StgHalfInt, StgHalfWord, StgWord, StgWord16},
};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;

#[cfg(test)]
mod tests;

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

static closure_flags: [StgWord16; 0usize] = unsafe { sys::closure_flags };

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

pub type StgLargeBitmap = StgLargeBitmap_;

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union StgClosureInfo {
    pub payload: StgClosureInfo__bindgen_ty_1,
    pub bitmap: StgWord,
    pub large_bitmap_offset: StgHalfInt,
    pub __pad_large_bitmap_offset: StgHalfWord,
    pub selector_offset: StgWord,
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
        match <usize as Arbitrary>::arbitrary(g) % 5usize {
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
            4.. => StgClosureInfo {
                selector_offset: Arbitrary::arbitrary(g),
            },
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
pub(crate) union StgFunInfoExtraRev___bindgen_ty_1 {
    pub bitmap: StgWord,
    pub bitmap_offset: StgHalfInt,
    pub __pad_bitmap_offset: StgHalfWord,
}

#[cfg(feature = "sys")]
impl From<StgFunInfoExtraRev___bindgen_ty_1> for sys::StgFunInfoExtraRev___bindgen_ty_1 {
    fn from(x: StgFunInfoExtraRev___bindgen_ty_1) -> Self {
        unsafe { transmute(x) }
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

// TODO: Can `bitmap` be moved into StgFunInfoExtraFwd_?
#[repr(C)]
pub(crate) union StgFunInfoExtraFwd___bindgen_ty_1 {
    pub bitmap: StgWord,
}

#[cfg(feature = "sys")]
impl From<StgFunInfoExtraFwd___bindgen_ty_1> for sys::StgFunInfoExtraFwd___bindgen_ty_1 {
    fn from(x: StgFunInfoExtraFwd___bindgen_ty_1) -> Self {
        unsafe { transmute(x) }
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

#[unsafe(no_mangle)]
pub static stg_arg_bitmaps: [StgWord; 0usize] = unsafe { sys::stg_arg_bitmaps };

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

pub type StgConInfoTable = StgConInfoTable_;
