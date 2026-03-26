use super::*;

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_stg_copyArray_barrier() {
    let expected = {
        let result: StgFunPtr = unsafe { sys::stg_copyArray_barrier() };
        todo!()
    };

    let actual = {
        let result: StgFunPtr = unsafe { stg_copyArray_barrier() };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_stg_copyArray_barrier() {
    let actual = {
        let result: StgFunPtr = unsafe { stg_copyArray_barrier() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}
