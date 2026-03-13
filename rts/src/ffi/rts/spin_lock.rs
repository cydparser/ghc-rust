use crate::ffi::stg::types::{StgWord, StgWord64};
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(testsuite)]
#[repr(C)]
#[cfg_attr(test, derive(Clone))]
pub struct SpinLock_ {
    pub(crate) lock: StgWord,
    pub(crate) spin: StgWord64,
    pub(crate) yield_: StgWord64,
}

#[cfg(test)]
impl Arbitrary for SpinLock_ {
    fn arbitrary(g: &mut Gen) -> Self {
        SpinLock_ {
            lock: Arbitrary::arbitrary(g),
            spin: Arbitrary::arbitrary(g),
            yield_: Arbitrary::arbitrary(g),
        }
    }
}

#[ffi(compiler)]
pub type SpinLock = SpinLock_;
