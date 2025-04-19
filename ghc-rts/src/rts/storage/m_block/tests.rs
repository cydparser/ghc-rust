use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::mem::{size_of, transmute};
#[test]
#[ignore]
fn test_initMBlocks() {
    unsafe { super::initMBlocks() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getMBlock() -> bool {
    let expected = unsafe { transmute(sys::getMBlock()) };
    let actual = unsafe { super::getMBlock() };
    actual == expected
}

#[test]
#[ignore]
fn test_getMBlock() {
    unsafe { super::getMBlock() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getMBlocks(n: u32) -> bool {
    let expected = unsafe { transmute(sys::getMBlocks(n.into())) };
    let actual = unsafe { super::getMBlocks(n) };
    actual == expected
}

#[test]
#[ignore]
fn test_getMBlocks() {
    let n = Default::default();
    unsafe { super::getMBlocks(n) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getMBlockOnNode(node: u32) -> bool {
    let expected = unsafe { transmute(sys::getMBlockOnNode(node.into())) };
    let actual = unsafe { super::getMBlockOnNode(node) };
    actual == expected
}

#[test]
#[ignore]
fn test_getMBlockOnNode() {
    let node = Default::default();
    unsafe { super::getMBlockOnNode(node) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getMBlocksOnNode(node: u32, n: u32) -> bool {
    let expected = unsafe { transmute(sys::getMBlocksOnNode(node.into(), n.into())) };
    let actual = unsafe { super::getMBlocksOnNode(node, n) };
    actual == expected
}

#[test]
#[ignore]
fn test_getMBlocksOnNode() {
    let node = Default::default();
    let n = Default::default();
    unsafe { super::getMBlocksOnNode(node, n) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_freeMBlocks() {
    let mut addr = Default::default();
    let n = Default::default();
    unsafe { super::freeMBlocks(&mut addr, n) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_releaseFreeMemory() {
    unsafe { super::releaseFreeMemory() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_freeAllMBlocks() {
    unsafe { super::freeAllMBlocks() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getFirstMBlock(state: ::core::ffi::c_void) -> bool {
    let expected = unsafe { transmute(sys::getFirstMBlock(&mut &mut state.into())) };
    let actual = unsafe { super::getFirstMBlock(&mut &mut state) };
    actual == expected
}

#[test]
#[ignore]
fn test_getFirstMBlock() {
    let mut state = Default::default();
    unsafe { super::getFirstMBlock(&mut &mut state) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getNextMBlock(state: ::core::ffi::c_void, mblock: ::core::ffi::c_void) -> bool {
    let expected = unsafe {
        transmute(sys::getNextMBlock(
            &mut &mut state.into(),
            &mut mblock.into(),
        ))
    };
    let actual = unsafe { super::getNextMBlock(&mut &mut state, &mut mblock) };
    actual == expected
}

#[test]
#[ignore]
fn test_getNextMBlock() {
    let mut state = Default::default();
    let mut mblock = Default::default();
    unsafe { super::getNextMBlock(&mut &mut state, &mut mblock) };
    todo!("assert")
}
