use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[test]
fn test_size_of__HpcModuleInfo() {
    assert_eq!(
        size_of::<sys::_HpcModuleInfo>(),
        size_of::<super::_HpcModuleInfo>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _HpcModuleInfo"][::core::mem::size_of::<_HpcModuleInfo>() - 40usize];
    ["Alignment of _HpcModuleInfo"][::core::mem::align_of::<_HpcModuleInfo>() - 8usize];
    ["Offset of field: _HpcModuleInfo::modName"]
        [::core::mem::offset_of!(_HpcModuleInfo, modName) - 0usize];
    ["Offset of field: _HpcModuleInfo::tickCount"]
        [::core::mem::offset_of!(_HpcModuleInfo, tickCount) - 8usize];
    ["Offset of field: _HpcModuleInfo::hashNo"]
        [::core::mem::offset_of!(_HpcModuleInfo, hashNo) - 12usize];
    ["Offset of field: _HpcModuleInfo::tixArr"]
        [::core::mem::offset_of!(_HpcModuleInfo, tixArr) - 16usize];
    ["Offset of field: _HpcModuleInfo::from_file"]
        [::core::mem::offset_of!(_HpcModuleInfo, from_file) - 24usize];
    ["Offset of field: _HpcModuleInfo::next"]
        [::core::mem::offset_of!(_HpcModuleInfo, next) - 32usize];
};

#[test]
#[ignore]
fn test_hs_hpc_module() {
    let mut modName = Default::default();
    let modCount = Default::default();
    let modHashNo = Default::default();
    let mut tixArr = Default::default();
    unsafe { super::hs_hpc_module(&mut modName, modCount, modHashNo, &mut tixArr) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_hpc_rootModule() -> bool {
    let expected = unsafe { transmute(sys::hs_hpc_rootModule()) };
    let actual = unsafe { super::hs_hpc_rootModule() };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_hpc_rootModule() {
    unsafe { super::hs_hpc_rootModule() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_startupHpc() {
    unsafe { super::startupHpc() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_exitHpc() {
    unsafe { super::exitHpc() };
    todo!("assert")
}
