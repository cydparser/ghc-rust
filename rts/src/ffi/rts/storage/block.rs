use crate::ffi::rts::storage::gc::generation_;
use crate::ffi::stg::W_;
use crate::ffi::stg::types::{StgPtr, StgWord, StgWord16, StgWord32};
use crate::prelude::*;

#[cfg(test)]
mod tests;

pub(crate) const UNIT: u32 = 1;

#[ffi(compiler, ghc_lib, testsuite)]
pub const BLOCK_SIZE: u32 = 4096;

pub(crate) const BLOCK_MASK: u32 = 4095;

#[ffi(ghc_lib)]
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

#[ffi(compiler, docs, ghc_lib)]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NonmovingSegmentInfo {
    pub allocator_idx: StgWord16,
    pub next_free_snap: StgWord16,
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

#[ffi(compiler, ghc_lib)]
#[repr(C)]
pub struct bdescr_ {
    pub start: StgPtr,
    pub __bindgen_anon_1: bdescr___bindgen_ty_1,
    pub link: *mut bdescr_,
    pub u: bdescr___bindgen_ty_2,
    pub gen_: *mut generation_,
    pub gen_no: StgWord16,
    pub dest_no: StgWord16,
    pub node: StgWord16,
    pub flags: StgWord16,
    pub blocks: StgWord32,
    pub _padding: [StgWord32; 3usize],
}

#[ffi(compiler, docs, ghc_lib)]
#[repr(C)]
pub union bdescr___bindgen_ty_1 {
    pub free: StgPtr,
    pub nonmoving_segment: NonmovingSegmentInfo,
}

#[ffi(compiler, ghc_lib)]
#[repr(C)]
pub union bdescr___bindgen_ty_2 {
    pub back: *mut bdescr_,
    pub bitmap: *mut StgWord,
    pub scan: StgPtr,
}

#[ffi(testsuite)]
pub type bdescr = bdescr_;

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn allocAlignedGroupOnNode(node: u32, n: W_) -> *mut bdescr {
    sys! {
        allocAlignedGroupOnNode(node, n).cast()
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn allocGroup_lock(n: W_) -> *mut bdescr {
    sys! {
        allocGroup_lock(n).cast()
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn freeGroup_lock(p: *mut bdescr) {
    sys! {
        freeGroup_lock(p as * mut sys::bdescr)
    }
}
