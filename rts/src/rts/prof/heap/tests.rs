use super::*;
use crate::stg::types::StgWord;

#[test]
#[ignore]
fn test_requestHeapCensus() {
    unsafe { requestHeapCensus() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_startHeapProfTimer() {
    unsafe { startHeapProfTimer() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_stopHeapProfTimer() {
    unsafe { stopHeapProfTimer() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_setUserEra() {
    let w = Default::default();
    unsafe { setUserEra(w) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_getUserEra() -> bool {
    let expected = unsafe { sys::getUserEra() };
    let actual = unsafe { getUserEra() };
    actual == expected
}

#[test]
#[ignore]
fn test_getUserEra() {
    unsafe { getUserEra() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_incrementUserEra(w: StgWord) -> bool {
    let expected = unsafe { sys::incrementUserEra(w) };
    let actual = unsafe { incrementUserEra(w) };
    actual == expected
}

#[test]
#[ignore]
fn test_incrementUserEra() {
    let w = Default::default();
    unsafe { incrementUserEra(w) };
    todo!("assert")
}
