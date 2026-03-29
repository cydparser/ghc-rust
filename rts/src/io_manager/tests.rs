use super::*;

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_setIOManagerControlFd() {
    let g = &mut Gen::new(100);

    let actual = {
        let cap_no: u32 = Arbitrary::arbitrary(g);
        let fd: i32 = Arbitrary::arbitrary(g);
        unsafe { setIOManagerControlFd(cap_no, fd) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_setIOManagerControlFd(cap_no: u32, fd: i32) -> bool {
    let expected = {
        unsafe { sys::setIOManagerControlFd(cap_no, fd) };
        todo!()
    };

    let actual = {
        unsafe { setIOManagerControlFd(cap_no, fd) };
        todo!()
    };

    actual == expected
}
