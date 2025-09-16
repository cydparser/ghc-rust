use super::*;
use crate::stg::types::StgWord;

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_requestHeapCensus() {
    todo!()
}

#[test]
#[ignore]
fn test_requestHeapCensus() {
    unsafe { requestHeapCensus() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_startHeapProfTimer() {
    todo!()
}

#[test]
#[ignore]
fn test_startHeapProfTimer() {
    unsafe { startHeapProfTimer() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_stopHeapProfTimer() {
    todo!()
}

#[test]
#[ignore]
fn test_stopHeapProfTimer() {
    unsafe { stopHeapProfTimer() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_setUserEra() {
    todo!()
}

#[test]
#[ignore]
fn test_setUserEra() {
    let mut g = Gen::new(100);
    let w = Arbitrary::arbitrary(&mut g);
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
    let mut g = Gen::new(100);
    let w = Arbitrary::arbitrary(&mut g);
    unsafe { incrementUserEra(w) };
    todo!("assert")
}
