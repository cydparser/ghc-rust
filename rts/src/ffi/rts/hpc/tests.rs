use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_HpcModuleInfo_layout() {
    assert_eq!(size_of::<HpcModuleInfo>(), size_of::<sys::HpcModuleInfo>());
    assert_eq!(
        align_of::<HpcModuleInfo>(),
        align_of::<sys::HpcModuleInfo>()
    );
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_hs_hpc_module(
    modName: c_char,
    modCount: StgWord32,
    modHashNo: StgWord32,
    tixArr: StgWord64,
) -> bool {
    let expected = {
        let mut modName = modName;
        let mut tixArr = tixArr;
        unsafe { sys::hs_hpc_module(&raw mut modName, modCount, modHashNo, &raw mut tixArr) };
        todo!()
    };

    let actual = {
        let mut modName = modName;
        let mut tixArr = tixArr;
        unsafe { hs_hpc_module(&raw mut modName, modCount, modHashNo, &raw mut tixArr) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_hpc_module() {
    let g = &mut Gen::new(100);
    let actual = {
        let mut modName: c_char = Arbitrary::arbitrary(g);
        let modCount: StgWord32 = Arbitrary::arbitrary(g);
        let modHashNo: StgWord32 = Arbitrary::arbitrary(g);
        let mut tixArr: StgWord64 = Arbitrary::arbitrary(g);
        unsafe { hs_hpc_module(&raw mut modName, modCount, modHashNo, &raw mut tixArr) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

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

    assert_eq!(actual, expected);
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
