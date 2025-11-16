use crate::ffi::rts::storage::gc::generation_;
#[cfg(feature = "sys")]
use crate::ffi::stg::W_;
use crate::ffi::stg::types::{StgPtr, StgWord, StgWord16, StgWord32};
#[allow(unused_imports)]
use crate::prelude::*;

#[cfg(test)]
mod tests;

pub(crate) const UNIT: u32 = 1;

/// - GHC_PLACES: {compiler, libraries, testsuite}
pub const BLOCK_SIZE: u32 = 4096;

pub(crate) const BLOCK_MASK: u32 = 4095;

/// - GHC_PLACES: {libraries}
pub const MBLOCK_SIZE: u32 = 1048576;

pub(crate) const MBLOCK_MASK: u32 = 1048575;

pub(crate) const BDESCR_SIZE: u32 = 64;

pub(crate) const BDESCR_MASK: u32 = 63;

pub(crate) const BDESCR_SHIFT: u32 = 6;

pub(crate) const BF_EVACUATED: u32 = 1;

pub(crate) const BF_LARGE: u32 = 2;

pub(crate) const BF_PINNED: u32 = 4;

pub(crate) const BF_MARKED: u32 = 8;

pub(crate) const BF_EXEC: u32 = 32;

pub(crate) const BF_FRAGMENTED: u32 = 64;

pub(crate) const BF_KNOWN: u32 = 128;

pub(crate) const BF_SWEPT: u32 = 256;

pub(crate) const BF_COMPACT: u32 = 512;

pub(crate) const BF_NONMOVING: u32 = 1024;

pub(crate) const BF_NONMOVING_SWEEPING: u32 = 2048;

pub(crate) const BF_FLAG_MAX: u32 = 32768;

/// cbindgen:no-export
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct NonmovingSegmentInfo {
    allocator_idx: StgWord16,
    next_free_snap: StgWord16,
}

#[cfg(feature = "sys")]
impl From<NonmovingSegmentInfo> for sys::NonmovingSegmentInfo {
    fn from(x: NonmovingSegmentInfo) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for NonmovingSegmentInfo {
    fn arbitrary(g: &mut Gen) -> Self {
        NonmovingSegmentInfo {
            allocator_idx: Arbitrary::arbitrary(g),
            next_free_snap: Arbitrary::arbitrary(g),
        }
    }
}

/// cbindgen:no-export
#[repr(C)]
pub struct bdescr_ {
    start: StgPtr,
    __bindgen_anon_1: bdescr___bindgen_ty_1,
    link: *mut bdescr_,
    u: bdescr___bindgen_ty_2,
    gen_: *mut generation_,
    gen_no: StgWord16,
    dest_no: StgWord16,
    node: StgWord16,
    flags: StgWord16,
    blocks: StgWord32,
    _padding: [StgWord32; 3usize],
}

#[cfg(feature = "sys")]
impl From<bdescr_> for sys::bdescr_ {
    fn from(x: bdescr_) -> Self {
        unsafe { transmute(x) }
    }
}

#[repr(C)]
pub(crate) union bdescr___bindgen_ty_1 {
    free: StgPtr,
    nonmoving_segment: NonmovingSegmentInfo,
}

#[cfg(feature = "sys")]
impl From<bdescr___bindgen_ty_1> for sys::bdescr___bindgen_ty_1 {
    fn from(x: bdescr___bindgen_ty_1) -> Self {
        unsafe { transmute(x) }
    }
}

#[repr(C)]
pub(crate) union bdescr___bindgen_ty_2 {
    back: *mut bdescr_,
    bitmap: *mut StgWord,
    scan: StgPtr,
}

#[cfg(feature = "sys")]
impl From<bdescr___bindgen_ty_2> for sys::bdescr___bindgen_ty_2 {
    fn from(x: bdescr___bindgen_ty_2) -> Self {
        unsafe { transmute(x) }
    }
}

/// - GHC_PLACES: {testsuite}
pub type bdescr = bdescr_;

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn allocAlignedGroupOnNode(node: u32, n: W_) -> *mut bdescr {
    #[cfg(feature = "sys")]
    unsafe {
        sys::allocAlignedGroupOnNode(node, n) as *mut bdescr
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("allocAlignedGroupOnNode")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn allocGroup_lock(n: W_) -> *mut bdescr {
    #[cfg(feature = "sys")]
    unsafe {
        sys::allocGroup_lock(n) as *mut bdescr
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("allocGroup_lock")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn freeGroup_lock(p: *mut bdescr) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::freeGroup_lock(p as *mut sys::bdescr)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("freeGroup_lock")
}
