use super::*;

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_stopProfTimer() {
    let actual = {
        unsafe { stopProfTimer() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_stopProfTimer() {
    let expected = {
        unsafe { sys::stopProfTimer() };
        todo!()
    };

    let actual = {
        unsafe { stopProfTimer() };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_startProfTimer() {
    let actual = {
        unsafe { startProfTimer() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_startProfTimer() {
    let expected = {
        unsafe { sys::startProfTimer() };
        todo!()
    };

    let actual = {
        unsafe { startProfTimer() };
        todo!()
    };
    assert_eq!(actual, expected);
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
    assert_eq!(actual, expected);
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
fn equivalent_startHeapProfTimer() {
    let expected = {
        unsafe { sys::startHeapProfTimer() };
        todo!()
    };

    let actual = {
        unsafe { startHeapProfTimer() };
        todo!()
    };
    assert_eq!(actual, expected);
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
fn equivalent_requestHeapCensus() {
    let expected = {
        unsafe { sys::requestHeapCensus() };
        todo!()
    };

    let actual = {
        unsafe { requestHeapCensus() };
        todo!()
    };
    assert_eq!(actual, expected);
}
