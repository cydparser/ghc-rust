use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_lockFile(
    id: StgWord64,
    dev: StgWord64,
    ino: StgWord64,
    for_writing: ::core::ffi::c_int,
) -> bool {
    let expected = unsafe {
        transmute(sys::lockFile(
            id.into(),
            dev.into(),
            ino.into(),
            for_writing.into(),
        ))
    };
    let actual = unsafe { super::lockFile(id, dev, ino, for_writing) };
    actual == expected
}

#[test]
#[ignore]
fn test_lockFile() {
    let id = Default::default();
    let dev = Default::default();
    let ino = Default::default();
    let for_writing = Default::default();
    unsafe { super::lockFile(id, dev, ino, for_writing) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_unlockFile(id: StgWord64) -> bool {
    let expected = unsafe { transmute(sys::unlockFile(id.into())) };
    let actual = unsafe { super::unlockFile(id) };
    actual == expected
}

#[test]
#[ignore]
fn test_unlockFile() {
    let id = Default::default();
    unsafe { super::unlockFile(id) };
    todo!("assert")
}
