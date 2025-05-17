use super::*;
use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
use crate::utils::test::*;
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::ffi::{c_char, c_int, c_uint, c_void};
use std::mem::transmute;
use std::ptr::{null, null_mut};
#[test]
#[ignore]
fn test_initMBlocks() {
    unsafe { initMBlocks() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getMBlock() -> bool {
    let expected = unsafe { sys::getMBlock() };
    let actual = unsafe { getMBlock() };
    actual == expected
}

#[test]
#[ignore]
fn test_getMBlock() {
    unsafe { getMBlock() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getMBlocks(n: u32) -> bool {
    let expected = unsafe { sys::getMBlocks(n) };
    let actual = unsafe { getMBlocks(n) };
    actual == expected
}

#[test]
#[ignore]
fn test_getMBlocks() {
    let n = Default::default();
    unsafe { getMBlocks(n) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getMBlockOnNode(node: u32) -> bool {
    let expected = unsafe { sys::getMBlockOnNode(node) };
    let actual = unsafe { getMBlockOnNode(node) };
    actual == expected
}

#[test]
#[ignore]
fn test_getMBlockOnNode() {
    let node = Default::default();
    unsafe { getMBlockOnNode(node) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getMBlocksOnNode(node: u32, n: u32) -> bool {
    let expected = unsafe { sys::getMBlocksOnNode(node, n) };
    let actual = unsafe { getMBlocksOnNode(node, n) };
    actual == expected
}

#[test]
#[ignore]
fn test_getMBlocksOnNode() {
    let node = Default::default();
    let n = Default::default();
    unsafe { getMBlocksOnNode(node, n) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_freeMBlocks() {
    let mut addr = null_mut();
    let n = Default::default();
    unsafe { freeMBlocks(&mut addr, n) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_releaseFreeMemory() {
    unsafe { releaseFreeMemory() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_freeAllMBlocks() {
    unsafe { freeAllMBlocks() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getFirstMBlock(state: c_void) -> bool {
    let expected = unsafe { sys::getFirstMBlock(&mut &mut state) };
    let actual = unsafe { getFirstMBlock(&mut &mut state) };
    actual == expected
}

#[test]
#[ignore]
fn test_getFirstMBlock() {
    let mut state = null_mut();
    unsafe { getFirstMBlock(&mut &mut state) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getNextMBlock(state: c_void, mblock: c_void) -> bool {
    let expected = unsafe { sys::getNextMBlock(&mut &mut state, &mut mblock) };
    let actual = unsafe { getNextMBlock(&mut &mut state, &mut mblock) };
    actual == expected
}

#[test]
#[ignore]
fn test_getNextMBlock() {
    let mut state = null_mut();
    let mut mblock = null_mut();
    unsafe { getNextMBlock(&mut &mut state, &mut mblock) };
    todo!("assert")
}
