use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(test)]
use crate::utils::test::{Arbitrary, Gen, HasReferences};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use libc::{clockid_t, pid_t, pthread_cond_t, pthread_key_t, pthread_mutex_t, pthread_t};
use std::ffi::{c_char, c_int, c_uint, c_void};
use std::mem::transmute;
use std::ptr::{null, null_mut};
use std::slice;
#[cfg(feature = "tracing")]
use tracing::instrument;
#[cfg(test)]
mod tests;

#[repr(C)]
///cbindgen:no-export
pub(crate) struct _HpcModuleInfo {
    pub modName: *mut c_char,
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
#[derive(Clone)]
struct _HpcModuleInfoOwned {
    pub tickCount: StgWord32,
    pub hashNo: StgWord32,
    pub from_file: bool,
}

#[cfg(test)]
impl Arbitrary for _HpcModuleInfoOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        _HpcModuleInfoOwned {
            tickCount: Arbitrary::arbitrary(g),
            hashNo: Arbitrary::arbitrary(g),
            from_file: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct _HpcModuleInfoPointees {
    pub modName: c_char,
    pub tixArr: StgWord64,
    pub next: _HpcModuleInfo,
}

#[cfg(test)]
impl Arbitrary for _HpcModuleInfoPointees {
    fn arbitrary(g: &mut Gen) -> Self {
        _HpcModuleInfoPointees {
            modName: Arbitrary::arbitrary(g),
            tixArr: Arbitrary::arbitrary(g),
            next: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for _HpcModuleInfo {
    type Owned = _HpcModuleInfoOwned;
    type Pointees = _HpcModuleInfoPointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            tickCount: owned.tickCount,
            hashNo: owned.hashNo,
            from_file: owned.from_file.clone(),
            modName: unsafe { &raw mut (*pointees).modName },
            tixArr: unsafe { &raw mut (*pointees).tixArr },
            next: unsafe { &raw mut (*pointees).next },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            tickCount: self.tickCount,
            hashNo: self.hashNo,
            from_file: self.from_file.clone(),
        }
    }
}

pub type HpcModuleInfo = _HpcModuleInfo;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_hpc_module"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_hpc_module(
    modName: *mut c_char,
    modCount: StgWord32,
    modHashNo: StgWord32,
    tixArr: *mut StgWord64,
) {
    unsafe { sys::hs_hpc_module(modName, modCount, modHashNo, tixArr) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_hpc_rootModule"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_hpc_rootModule() -> *mut HpcModuleInfo {
    unsafe { transmute(sys::hs_hpc_rootModule()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn startupHpc() {
    unsafe { sys::startupHpc() }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn exitHpc() {
    unsafe { sys::exitHpc() }
}
