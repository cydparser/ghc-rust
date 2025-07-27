use std::mem::transmute;

#[cfg(test)]
use crate::utils::test::{Arbitrary, Gen};
use crate::{
    rts::storage::closures::StgClosure,
    stg::types::{StgCode, StgFun, StgHalfInt, StgHalfWord, StgWord, StgWord16},
    utils::bindgen::__IncompleteArrayField,
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
///cbindgen:no-export
#[cfg_attr(test, derive(Clone))]
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

static closure_flags: [StgWord16; 0usize] = [];

pub type StgLargeBitmap = StgLargeBitmap_;

#[repr(C)]
///cbindgen:no-export
pub struct StgLargeBitmap_ {
    size: StgWord,
    bitmap: __IncompleteArrayField<StgWord>,
}

#[cfg(feature = "sys")]
impl From<StgLargeBitmap_> for sys::StgLargeBitmap_ {
    fn from(x: StgLargeBitmap_) -> Self {
        unsafe { transmute(x) }
    }
}

#[repr(C)]
pub(crate) union StgClosureInfo {
    pub payload: StgClosureInfo_anon_union_1,
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

#[repr(C)]
///cbindgen:no-export
#[derive(Copy, Clone)]
pub(crate) struct StgClosureInfo_anon_union_1 {
    pub ptrs: StgHalfWord,
    pub nptrs: StgHalfWord,
}

#[cfg(feature = "sys")]
impl From<StgClosureInfo_anon_union_1> for sys::StgClosureInfo__bindgen_ty_1 {
    fn from(x: StgClosureInfo_anon_union_1) -> Self {
        unsafe { transmute(x) }
    }
}

pub(crate) type StgSRTField = StgHalfInt;

pub type StgInfoTable = StgInfoTable_;

#[repr(C)]
///cbindgen:no-export
pub struct StgInfoTable_ {
    layout: StgClosureInfo,
    type_: StgHalfWord,
    srt: StgSRTField,
    code: __IncompleteArrayField<StgCode>,
}

#[cfg(feature = "sys")]
impl From<StgInfoTable_> for sys::StgInfoTable_ {
    fn from(x: StgInfoTable_) -> Self {
        unsafe { transmute(x) }
    }
}

pub(crate) type StgInfoTablePtr = *mut StgInfoTable_;

pub type StgFunInfoExtraRev = StgFunInfoExtraRev_;

#[repr(C)]
///cbindgen:no-export
pub struct StgFunInfoExtraRev_ {
    slow_apply_offset: StgHalfInt,
    __pad_slow_apply_offset: StgHalfWord,
    b: StgFunInfoExtraRev__anon_union_1,
    fun_type: StgHalfWord,
    arity: StgHalfWord,
}

#[cfg(feature = "sys")]
impl From<StgFunInfoExtraRev_> for sys::StgFunInfoExtraRev_ {
    fn from(x: StgFunInfoExtraRev_) -> Self {
        unsafe { transmute(x) }
    }
}

#[repr(C)]
pub(crate) union StgFunInfoExtraRev__anon_union_1 {
    pub bitmap: StgWord,
    pub bitmap_offset: StgHalfInt,
    pub __pad_bitmap_offset: StgHalfWord,
}

#[cfg(feature = "sys")]
impl From<StgFunInfoExtraRev__anon_union_1> for sys::StgFunInfoExtraRev___bindgen_ty_1 {
    fn from(x: StgFunInfoExtraRev__anon_union_1) -> Self {
        unsafe { transmute(x) }
    }
}

pub type StgFunInfoExtraFwd = StgFunInfoExtraFwd_;

#[repr(C)]
///cbindgen:no-export
pub struct StgFunInfoExtraFwd_ {
    fun_type: StgHalfWord,
    arity: StgHalfWord,
    srt: *mut StgClosure,
    b: StgFunInfoExtraFwd__anon_union_1,
    slow_apply: StgFun,
}

#[cfg(feature = "sys")]
impl From<StgFunInfoExtraFwd_> for sys::StgFunInfoExtraFwd_ {
    fn from(x: StgFunInfoExtraFwd_) -> Self {
        unsafe { transmute(x) }
    }
}

#[repr(C)]
pub(crate) union StgFunInfoExtraFwd__anon_union_1 {
    pub bitmap: StgWord,
}

#[cfg(feature = "sys")]
impl From<StgFunInfoExtraFwd__anon_union_1> for sys::StgFunInfoExtraFwd___bindgen_ty_1 {
    fn from(x: StgFunInfoExtraFwd__anon_union_1) -> Self {
        unsafe { transmute(x) }
    }
}

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

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_arg_bitmaps"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_arg_bitmaps: [StgWord; 0usize] = [];

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

pub(crate) type StgThunkInfoTable = StgThunkInfoTable_;

#[repr(C)]
///cbindgen:no-export
pub(crate) struct StgThunkInfoTable_ {
    pub i: StgInfoTable,
}

#[cfg(feature = "sys")]
impl From<StgThunkInfoTable_> for sys::StgThunkInfoTable_ {
    fn from(x: StgThunkInfoTable_) -> Self {
        unsafe { transmute(x) }
    }
}

pub type StgConInfoTable = StgConInfoTable_;

#[repr(C)]
///cbindgen:no-export
pub struct StgConInfoTable_ {
    con_desc: StgHalfInt,
    __pad_con_desc: StgHalfWord,
    i: StgInfoTable,
}

#[cfg(feature = "sys")]
impl From<StgConInfoTable_> for sys::StgConInfoTable_ {
    fn from(x: StgConInfoTable_) -> Self {
        unsafe { transmute(x) }
    }
}
