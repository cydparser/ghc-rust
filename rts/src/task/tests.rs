use super::*;

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_setInCallCapability() {
    let g = &mut Gen::new(100);

    let actual = {
        let preferred_capability: c_int = Arbitrary::arbitrary(g);
        let affinity: c_int = Arbitrary::arbitrary(g);
        unsafe { rts_setInCallCapability(preferred_capability, affinity) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_setInCallCapability(preferred_capability: c_int, affinity: c_int) -> bool {
    let expected = {
        unsafe { sys::rts_setInCallCapability(preferred_capability, affinity) };
        todo!()
    };

    let actual = {
        unsafe { rts_setInCallCapability(preferred_capability, affinity) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_pinThreadToNumaNode() {
    let g = &mut Gen::new(100);

    let actual = {
        let node: c_int = Arbitrary::arbitrary(g);
        unsafe { rts_pinThreadToNumaNode(node) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_pinThreadToNumaNode(node: c_int) -> bool {
    let expected = {
        unsafe { sys::rts_pinThreadToNumaNode(node) };
        todo!()
    };

    let actual = {
        unsafe { rts_pinThreadToNumaNode(node) };
        todo!()
    };

    actual == expected
}
