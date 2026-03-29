use super::*;

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_setInCallCapability() {
    let g = &mut Gen::new(100);

    let actual = {
        let preferred_capability: i32 = Arbitrary::arbitrary(g);
        let affinity: i32 = Arbitrary::arbitrary(g);
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
fn equivalent_rts_setInCallCapability(preferred_capability: i32, affinity: i32) -> bool {
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
        let node: i32 = Arbitrary::arbitrary(g);
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
fn equivalent_rts_pinThreadToNumaNode(node: i32) -> bool {
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
