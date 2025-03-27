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

#[repr(C)]
pub(crate) struct _HpcModuleInfo {
    pub modName: *mut ::core::ffi::c_char,
    pub tickCount: StgWord32,
    pub hashNo: StgWord32,
    pub tixArr: *mut StgWord64,
    pub from_file: bool,
    pub next: *mut _HpcModuleInfo,
}

#[cfg(feature = "sys")]
impl From<_HpcModuleInfo> for sys::_HpcModuleInfo {
    fn from(x: _HpcModuleInfo) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for _HpcModuleInfo {
    fn arbitrary(g: &mut Gen) -> Self {
        _HpcModuleInfo {
            modName: Arbitrary::arbitrary(g),
            tickCount: Arbitrary::arbitrary(g),
            hashNo: Arbitrary::arbitrary(g),
            tixArr: Arbitrary::arbitrary(g),
            from_file: Arbitrary::arbitrary(g),
            next: Arbitrary::arbitrary(g),
        }
    }
}

pub type HpcModuleInfo = _HpcModuleInfo;

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_hpc_module(
    modName: *mut ::core::ffi::c_char,
    modCount: StgWord32,
    modHashNo: StgWord32,
    tixArr: *mut StgWord64,
) {
    unsafe {
        transmute(sys::hs_hpc_module(
            &mut modName.into(),
            modCount.into(),
            modHashNo.into(),
            &mut tixArr.into(),
        ))
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_hpc_rootModule() -> *mut HpcModuleInfo {
    unsafe { transmute(sys::hs_hpc_rootModule()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn startupHpc() {
    unsafe { transmute(sys::startupHpc()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn exitHpc() {
    unsafe { transmute(sys::exitHpc()) }
}
