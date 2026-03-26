use super::*;

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_revertCAFs() {
    let actual = {
        unsafe { revertCAFs() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_revertCAFs() {
    let expected = {
        unsafe { sys::revertCAFs() };
        todo!()
    };

    let actual = {
        unsafe { revertCAFs() };
        todo!()
    };
    assert_eq!(actual, expected);
}
