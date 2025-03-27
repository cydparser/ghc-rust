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

#[repr(C)]
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

#[cfg(feature = "sys")]
impl From<bdescr_> for sys::bdescr_ {
    fn from(x: bdescr_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for bdescr_ {
    fn arbitrary(g: &mut Gen) -> Self {
        bdescr_ {
            start: Arbitrary::arbitrary(g),
            __bindgen_anon_1: Arbitrary::arbitrary(g),
            link: Arbitrary::arbitrary(g),
            u: Arbitrary::arbitrary(g),
            gen_: Arbitrary::arbitrary(g),
            gen_no: Arbitrary::arbitrary(g),
            dest_no: Arbitrary::arbitrary(g),
            node: Arbitrary::arbitrary(g),
            flags: Arbitrary::arbitrary(g),
            blocks: Arbitrary::arbitrary(g),
            _padding: Arbitrary::arbitrary(g),
        }
    }
}

#[repr(C)]
pub(crate) union bdescr___bindgen_ty_1 {
    pub free: ::core::mem::ManuallyDrop<StgPtr>,
    pub nonmoving_segment: ::core::mem::ManuallyDrop<NonmovingSegmentInfo>,
}

#[cfg(feature = "sys")]
impl From<bdescr___bindgen_ty_1> for sys::bdescr___bindgen_ty_1 {
    fn from(x: bdescr___bindgen_ty_1) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for bdescr___bindgen_ty_1 {
    fn arbitrary(g: &mut Gen) -> Self {
        match Arbitrary::arbitrary::<usize>(g) % 2usize {
            0 => bdescr___bindgen_ty_1 {
                free: Arbitrary::arbitrary(g),
            },
            1 => bdescr___bindgen_ty_1 {
                nonmoving_segment: Arbitrary::arbitrary(g),
            },
        }
    }
}

#[repr(C)]
pub(crate) union bdescr___bindgen_ty_2 {
    pub back: ::core::mem::ManuallyDrop<*mut bdescr_>,
    pub bitmap: ::core::mem::ManuallyDrop<*mut StgWord>,
    pub scan: ::core::mem::ManuallyDrop<StgPtr>,
}

#[cfg(feature = "sys")]
impl From<bdescr___bindgen_ty_2> for sys::bdescr___bindgen_ty_2 {
    fn from(x: bdescr___bindgen_ty_2) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for bdescr___bindgen_ty_2 {
    fn arbitrary(g: &mut Gen) -> Self {
        match Arbitrary::arbitrary::<usize>(g) % 3usize {
            0 => bdescr___bindgen_ty_2 {
                back: Arbitrary::arbitrary(g),
            },
            1 => bdescr___bindgen_ty_2 {
                bitmap: Arbitrary::arbitrary(g),
            },
            2 => bdescr___bindgen_ty_2 {
                scan: Arbitrary::arbitrary(g),
            },
        }
    }
}

pub type bdescr = bdescr_;

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn initBlockAllocator() {
    unsafe { transmute(sys::initBlockAllocator()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn allocGroup(n: W_) -> *mut bdescr {
    unsafe { transmute(sys::allocGroup(n.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn allocGroupOnNode(node: u32, n: W_) -> *mut bdescr {
    unsafe { transmute(sys::allocGroupOnNode(node.into(), n.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn allocAlignedGroupOnNode(node: u32, n: W_) -> *mut bdescr {
    unsafe { transmute(sys::allocAlignedGroupOnNode(node.into(), n.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn allocMBlockAlignedGroupOnNode(node: u32, n: W_) -> *mut bdescr {
    unsafe { transmute(sys::allocMBlockAlignedGroupOnNode(node.into(), n.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn allocGroup_lock(n: W_) -> *mut bdescr {
    unsafe { transmute(sys::allocGroup_lock(n.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn allocBlock_lock() -> *mut bdescr {
    unsafe { transmute(sys::allocBlock_lock()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn allocGroupOnNode_lock(node: u32, n: W_) -> *mut bdescr {
    unsafe { transmute(sys::allocGroupOnNode_lock(node.into(), n.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn allocBlockOnNode_lock(node: u32) -> *mut bdescr {
    unsafe { transmute(sys::allocBlockOnNode_lock(node.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn freeGroup(p: *mut bdescr) {
    unsafe { transmute(sys::freeGroup(&mut p.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn freeChain(p: *mut bdescr) {
    unsafe { transmute(sys::freeChain(&mut p.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn freeGroup_lock(p: *mut bdescr) {
    unsafe { transmute(sys::freeGroup_lock(&mut p.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn freeChain_lock(p: *mut bdescr) {
    unsafe { transmute(sys::freeChain_lock(&mut p.into())) }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct generation_ {
    pub _address: u8,
}

#[cfg(feature = "sys")]
impl From<generation_> for sys::generation_ {
    fn from(x: generation_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for generation_ {
    fn arbitrary(g: &mut Gen) -> Self {
        generation_ {
            _address: Arbitrary::arbitrary(g),
        }
    }
}
