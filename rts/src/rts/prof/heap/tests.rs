use super::*;

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_requestHeapCensus() {
    let expected = {
        unsafe { sys::requestHeapCensus() };
        todo!()
    };
    let actual = {
        unsafe { requestHeapCensus() };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_requestHeapCensus() {
    let actual = {
        unsafe { requestHeapCensus() };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_startHeapProfTimer() {
    let expected = {
        unsafe { sys::startHeapProfTimer() };
        todo!()
    };
    let actual = {
        unsafe { startHeapProfTimer() };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_startHeapProfTimer() {
    let actual = {
        unsafe { startHeapProfTimer() };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_stopHeapProfTimer() {
    let expected = {
        unsafe { sys::stopHeapProfTimer() };
        todo!()
    };
    let actual = {
        unsafe { stopHeapProfTimer() };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_stopHeapProfTimer() {
    let actual = {
        unsafe { stopHeapProfTimer() };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_setUserEra(w: StgWord) -> bool {
    let expected = {
        unsafe { sys::setUserEra(w) };
        todo!()
    };
    let actual = {
        unsafe { setUserEra(w) };
        todo!()
    };
    expected == actual
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_setUserEra() {
    let g = &mut Gen::new(100);
    let actual = {
        let w: StgWord = Arbitrary::arbitrary(g);
        unsafe { setUserEra(w) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_getUserEra() {
    let expected: StgWord = { unsafe { sys::getUserEra() } };
    let actual: StgWord = { unsafe { getUserEra() } };
    assert_eq!(expected, actual);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_getUserEra() {
    let actual: StgWord = { unsafe { getUserEra() } };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_incrementUserEra(w: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::incrementUserEra(w) } };
    let actual: StgWord = { unsafe { incrementUserEra(w) } };
    expected == actual
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_incrementUserEra() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let w: StgWord = Arbitrary::arbitrary(g);
        unsafe { incrementUserEra(w) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}
