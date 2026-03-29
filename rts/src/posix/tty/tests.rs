use super::*;

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test___hscore_get_saved_termios() {
    let g = &mut Gen::new(100);

    let actual = {
        let fd: i32 = Arbitrary::arbitrary(g);
        let result: &c_void = unsafe { &*__hscore_get_saved_termios(fd) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent___hscore_get_saved_termios(fd: i32) -> bool {
    let expected = {
        let result: &c_void = unsafe { &*sys::__hscore_get_saved_termios(fd) };
        todo!()
    };

    let actual = {
        let result: &c_void = unsafe { &*__hscore_get_saved_termios(fd) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test___hscore_set_saved_termios() {
    let g = &mut Gen::new(100);

    let actual = {
        let fd: i32 = Arbitrary::arbitrary(g);
        let ts: c_void = todo!();
        unsafe { __hscore_set_saved_termios(fd, &raw mut ts) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent___hscore_set_saved_termios(fd: i32) -> bool {
    let expected = {
        let mut ts: c_void = todo!();
        unsafe { sys::__hscore_set_saved_termios(fd, &raw mut ts) };
        todo!()
    };

    let actual = {
        let mut ts: c_void = todo!();
        unsafe { __hscore_set_saved_termios(fd, &raw mut ts) };
        todo!()
    };

    actual == expected
}
