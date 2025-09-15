use crate::prelude::*;
use crate::rts::storage::closures::StgClosure;
use crate::stg::types::{StgCode, StgFun, StgHalfInt, StgHalfWord, StgInt, StgWord};

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

/// cbindgen:no-export
#[repr(C)]
#[cfg_attr(test, derive(Clone))]
pub struct StgProfInfo {
    closure_type_off: StgInt,
    closure_desc_off: StgInt,
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
            closure_desc_off: Arbitrary::arbitrary(g),
        }
    }
}

/// cbindgen:no-export
#[repr(C)]
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

/// - GHC_PLACES: {libraries, testsuite}
pub type StgLargeBitmap = StgLargeBitmap_;

#[repr(C)]
pub(crate) union StgClosureInfo {
    pub(crate) payload: StgClosureInfo__bindgen_ty_1,
    pub(crate) bitmap: StgWord,
    pub(crate) large_bitmap_offset: StgInt,
    pub(crate) selector_offset: StgWord,
}

#[cfg(feature = "sys")]
impl From<StgClosureInfo> for sys::StgClosureInfo {
    fn from(x: StgClosureInfo) -> Self {
        unsafe { transmute(x) }
    }
}

/// cbindgen:no-export
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StgClosureInfo__bindgen_ty_1 {
    ptrs: StgHalfWord,
    nptrs: StgHalfWord,
}

#[cfg(feature = "sys")]
impl From<StgClosureInfo__bindgen_ty_1> for sys::StgClosureInfo__bindgen_ty_1 {
    fn from(x: StgClosureInfo__bindgen_ty_1) -> Self {
        unsafe { transmute(x) }
    }
}

pub(crate) type StgSRTField = StgHalfInt;

/// cbindgen:no-export
#[repr(C)]
pub struct StgInfoTable_ {
    pub(crate) layout: StgClosureInfo,
    pub(crate) type_: StgHalfWord,
    pub(crate) srt: StgSRTField,
    pub(crate) code: __IncompleteArrayField<StgCode>,
}

#[cfg(feature = "sys")]
impl From<StgInfoTable_> for sys::StgInfoTable_ {
    fn from(x: StgInfoTable_) -> Self {
        unsafe { transmute(x) }
    }
}

pub(crate) type StgInfoTablePtr = *mut StgInfoTable_;

/// cbindgen:no-export
#[repr(C)]
pub struct StgFunInfoExtraRev_ {
    pub(crate) slow_apply_offset: StgInt,
    pub(crate) b: StgFunInfoExtraRev___bindgen_ty_1,
    pub(crate) fun_type: StgHalfWord,
    pub(crate) arity: StgHalfWord,
}

#[cfg(feature = "sys")]
impl From<StgFunInfoExtraRev_> for sys::StgFunInfoExtraRev_ {
    fn from(x: StgFunInfoExtraRev_) -> Self {
        unsafe { transmute(x) }
    }
}

#[repr(C)]
pub(crate) union StgFunInfoExtraRev___bindgen_ty_1 {
    bitmap: StgWord,
    bitmap_offset: StgInt,
}

#[cfg(feature = "sys")]
impl From<StgFunInfoExtraRev___bindgen_ty_1> for sys::StgFunInfoExtraRev___bindgen_ty_1 {
    fn from(x: StgFunInfoExtraRev___bindgen_ty_1) -> Self {
        unsafe { transmute(x) }
    }
}

pub(crate) type StgFunInfoExtraRev = StgFunInfoExtraRev_;

/// cbindgen:no-export
#[repr(C)]
pub struct StgFunInfoExtraFwd_ {
    fun_type: StgHalfWord,
    arity: StgHalfWord,
    srt: *mut StgClosure,
    b: StgFunInfoExtraFwd___bindgen_ty_1,
    slow_apply: StgFun,
}

#[cfg(feature = "sys")]
impl From<StgFunInfoExtraFwd_> for sys::StgFunInfoExtraFwd_ {
    fn from(x: StgFunInfoExtraFwd_) -> Self {
        unsafe { transmute(x) }
    }
}

#[repr(C)]
pub(crate) union StgFunInfoExtraFwd___bindgen_ty_1 {
    bitmap: StgWord,
}

#[cfg(feature = "sys")]
impl From<StgFunInfoExtraFwd___bindgen_ty_1> for sys::StgFunInfoExtraFwd___bindgen_ty_1 {
    fn from(x: StgFunInfoExtraFwd___bindgen_ty_1) -> Self {
        unsafe { transmute(x) }
    }
}

pub(crate) type StgFunInfoExtraFwd = StgFunInfoExtraFwd_;

/// - GHC_PLACES: {libraries}
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

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_arg_bitmaps"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_arg_bitmaps: [StgWord; 0usize] = [];

/// - GHC_PLACES: {libraries}
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

/// cbindgen:no-export
#[repr(C)]
pub struct StgThunkInfoTable_ {
    i: StgInfoTable,
}

#[cfg(feature = "sys")]
impl From<StgThunkInfoTable_> for sys::StgThunkInfoTable_ {
    fn from(x: StgThunkInfoTable_) -> Self {
        unsafe { transmute(x) }
    }
}

pub(crate) type StgThunkInfoTable = StgThunkInfoTable_;

/// cbindgen:no-export
#[repr(C)]
pub struct StgConInfoTable_ {
    con_desc: StgInt,
    i: StgInfoTable,
}

#[cfg(feature = "sys")]
impl From<StgConInfoTable_> for sys::StgConInfoTable_ {
    fn from(x: StgConInfoTable_) -> Self {
        unsafe { transmute(x) }
    }
}

/// - GHC_PLACES: {libraries, testsuite}
pub type StgConInfoTable = StgConInfoTable_;
