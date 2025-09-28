use super::*;

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

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_hs_hpc_rootModule() {
    let expected = {
        let result: &HpcModuleInfo = unsafe { transmute(&*sys::hs_hpc_rootModule()) };
        todo!()
    };
    let actual = {
        let result: &HpcModuleInfo = unsafe { &*hs_hpc_rootModule() };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_hpc_rootModule() {
    let actual = {
        let result: &HpcModuleInfo = unsafe { &*hs_hpc_rootModule() };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}
