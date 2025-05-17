use super::*;
use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
use crate::utils::test::*;
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::ffi::{c_char, c_int, c_uint, c_void};
use std::mem::transmute;
use std::ptr::{null, null_mut};
#[cfg(feature = "sys")]
#[test]
fn sys_size__HpcModuleInfo() {
    assert_eq!(
        size_of::<sys::_HpcModuleInfo>(),
        size_of::<_HpcModuleInfo>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _HpcModuleInfo"][size_of::<_HpcModuleInfo>() - 40usize];
    ["Alignment of _HpcModuleInfo"][align_of::<_HpcModuleInfo>() - 8usize];
    ["Offset of field: _HpcModuleInfo::modName"][offset_of!(_HpcModuleInfo, modName) - 0usize];
    ["Offset of field: _HpcModuleInfo::tickCount"][offset_of!(_HpcModuleInfo, tickCount) - 8usize];
    ["Offset of field: _HpcModuleInfo::hashNo"][offset_of!(_HpcModuleInfo, hashNo) - 12usize];
    ["Offset of field: _HpcModuleInfo::tixArr"][offset_of!(_HpcModuleInfo, tixArr) - 16usize];
    ["Offset of field: _HpcModuleInfo::from_file"][offset_of!(_HpcModuleInfo, from_file) - 24usize];
    ["Offset of field: _HpcModuleInfo::next"][offset_of!(_HpcModuleInfo, next) - 32usize];
};

#[test]
#[ignore]
fn test_hs_hpc_module() {
    let mut modName = null_mut();
    let modCount = Default::default();
    let modHashNo = Default::default();
    let mut tixArr = null_mut();
    unsafe { hs_hpc_module(&mut modName, modCount, modHashNo, &mut tixArr) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_hpc_rootModule() -> bool {
    let expected = unsafe { transmute(sys::hs_hpc_rootModule()) };
    let actual = unsafe { hs_hpc_rootModule() };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_hpc_rootModule() {
    unsafe { hs_hpc_rootModule() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_startupHpc() {
    unsafe { startupHpc() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_exitHpc() {
    unsafe { exitHpc() };
    todo!("assert")
}
