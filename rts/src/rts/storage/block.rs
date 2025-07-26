use crate::prelude::*;
use crate::{
    rts::storage::gc::generation_,
    stg::types::{StgPtr, StgWord, StgWord16, StgWord32},
    stg::W_,
};

#[cfg(test)]
mod tests;

pub const UNIT: u32 = 1;

pub const BLOCK_SIZE: u32 = 4096;

pub(crate) const BLOCK_MASK: u32 = 4095;

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

///cbindgen:no-export
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct NonmovingSegmentInfo {
    pub allocator_idx: StgWord16,
    pub next_free_snap: StgWord16,
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

pub type bdescr = bdescr_;

#[repr(C)]
///cbindgen:no-export
pub(crate) struct bdescr_ {
    pub start: StgPtr,
    pub _anon_union_1: bdescr__anon_union_1,
    pub link: *mut bdescr_,
    pub u: bdescr__anon_union_2,
    pub gen_: *mut generation_,
    pub gen_no: StgWord16,
    pub dest_no: StgWord16,
    pub node: StgWord16,
    pub flags: StgWord16,
    pub blocks: StgWord32,
    pub _padding: [StgWord32; 3usize],
}

#[cfg(feature = "sys")]
impl From<bdescr_> for sys::bdescr_ {
    fn from(x: bdescr_) -> Self {
        unsafe { transmute(x) }
    }
}

#[repr(C)]
pub(crate) union bdescr__anon_union_1 {
    pub free: StgPtr,
    pub nonmoving_segment: NonmovingSegmentInfo,
}

#[cfg(feature = "sys")]
impl From<bdescr__anon_union_1> for sys::bdescr___bindgen_ty_1 {
    fn from(x: bdescr__anon_union_1) -> Self {
        unsafe { transmute(x) }
    }
}

#[repr(C)]
pub(crate) union bdescr__anon_union_2 {
    pub back: *mut bdescr_,
    pub bitmap: *mut StgWord,
    pub scan: StgPtr,
}

#[cfg(feature = "sys")]
impl From<bdescr__anon_union_2> for sys::bdescr___bindgen_ty_2 {
    fn from(x: bdescr__anon_union_2) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn initBlockAllocator() {
    unsafe { sys::initBlockAllocator() }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn allocGroup(n: W_) -> *mut bdescr {
    unsafe { transmute(sys::allocGroup(n)) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn allocGroupOnNode(node: u32, n: W_) -> *mut bdescr {
    unsafe { transmute(sys::allocGroupOnNode(node, n)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_allocAlignedGroupOnNode"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn allocAlignedGroupOnNode(node: u32, n: W_) -> *mut bdescr {
    unsafe { transmute(sys::allocAlignedGroupOnNode(node, n)) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn allocMBlockAlignedGroupOnNode(node: u32, n: W_) -> *mut bdescr {
    unsafe { transmute(sys::allocMBlockAlignedGroupOnNode(node, n)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_allocGroup_lock"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn allocGroup_lock(n: W_) -> *mut bdescr {
    unsafe { transmute(sys::allocGroup_lock(n)) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn allocBlock_lock() -> *mut bdescr {
    unsafe { transmute(sys::allocBlock_lock()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn allocGroupOnNode_lock(node: u32, n: W_) -> *mut bdescr {
    unsafe { transmute(sys::allocGroupOnNode_lock(node, n)) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn allocBlockOnNode_lock(node: u32) -> *mut bdescr {
    unsafe { transmute(sys::allocBlockOnNode_lock(node)) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn freeGroup(p: *mut bdescr) {
    unsafe { sys::freeGroup(p as *mut sys::bdescr) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn freeChain(p: *mut bdescr) {
    unsafe { sys::freeChain(p as *mut sys::bdescr) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_freeGroup_lock"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn freeGroup_lock(p: *mut bdescr) {
    unsafe { sys::freeGroup_lock(p as *mut sys::bdescr) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn freeChain_lock(p: *mut bdescr) {
    unsafe { sys::freeChain_lock(p as *mut sys::bdescr) }
}
