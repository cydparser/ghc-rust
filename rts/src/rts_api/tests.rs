use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_SchedulerStatus_discriminants() {
    assert_eq!(
        SchedulerStatus::NoStatus as isize,
        sys::SchedulerStatus::NoStatus as isize
    );
    assert_eq!(
        SchedulerStatus::Success as isize,
        sys::SchedulerStatus::Success as isize
    );
    assert_eq!(
        SchedulerStatus::Killed as isize,
        sys::SchedulerStatus::Killed as isize
    );
    assert_eq!(
        SchedulerStatus::Interrupted as isize,
        sys::SchedulerStatus::Interrupted as isize
    );
    assert_eq!(
        SchedulerStatus::HeapExhausted as isize,
        sys::SchedulerStatus::HeapExhausted as isize
    );
    assert_eq!(
        SchedulerStatus::SchedulerStatus_End as isize,
        sys::SchedulerStatus::SchedulerStatus_End as isize
    )
}

#[cfg(feature = "sys")]
#[test]
fn sys_SchedulerStatus_layout() {
    assert_eq!(
        size_of::<SchedulerStatus>(),
        size_of::<sys::SchedulerStatus>()
    );
    assert_eq!(
        align_of::<SchedulerStatus>(),
        align_of::<sys::SchedulerStatus>()
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_Capability_layout() {
    assert_eq!(size_of::<Capability>(), size_of::<sys::Capability>());
    assert_eq!(align_of::<Capability>(), align_of::<sys::Capability>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_Capability_layout() {
    assert_eq!(size_of::<Capability>(), size_of::<sys::Capability>());
    assert_eq!(align_of::<Capability>(), align_of::<sys::Capability>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HaskellObj_layout() {
    assert_eq!(size_of::<HaskellObj>(), size_of::<sys::HaskellObj>());
    assert_eq!(align_of::<HaskellObj>(), align_of::<sys::HaskellObj>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_PauseToken_layout() {
    assert_eq!(size_of::<PauseToken>(), size_of::<sys::PauseToken>());
    assert_eq!(align_of::<PauseToken>(), align_of::<sys::PauseToken>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_ListThreadsCb_layout() {
    assert_eq!(size_of::<ListThreadsCb>(), size_of::<sys::ListThreadsCb>());
    assert_eq!(
        align_of::<ListThreadsCb>(),
        align_of::<sys::ListThreadsCb>()
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_ListRootsCb_layout() {
    assert_eq!(size_of::<ListRootsCb>(), size_of::<sys::ListRootsCb>());
    assert_eq!(align_of::<ListRootsCb>(), align_of::<sys::ListRootsCb>());
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_mkChar() {
    let g = &mut Gen::new(100);

    let actual = {
        let arg1: Capability = todo!();
        let c: HsChar = Arbitrary::arbitrary(g);
        let result: HaskellObj = unsafe { rts_mkChar(&raw mut arg1, c) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_mkChar(c: HsChar) -> bool {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let result: HaskellObj = unsafe { transmute(sys::rts_mkChar(&raw mut arg1, c)) };
        todo!()
    };

    let actual = {
        let mut arg1: Capability = todo!();
        let result: HaskellObj = unsafe { rts_mkChar(&raw mut arg1, c) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_mkInt() {
    let g = &mut Gen::new(100);

    let actual = {
        let arg1: Capability = todo!();
        let i: HsInt = Arbitrary::arbitrary(g);
        let result: HaskellObj = unsafe { rts_mkInt(&raw mut arg1, i) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_mkInt(i: HsInt) -> bool {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let result: HaskellObj = unsafe { transmute(sys::rts_mkInt(&raw mut arg1, i)) };
        todo!()
    };

    let actual = {
        let mut arg1: Capability = todo!();
        let result: HaskellObj = unsafe { rts_mkInt(&raw mut arg1, i) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_mkInt8() {
    let g = &mut Gen::new(100);

    let actual = {
        let arg1: Capability = todo!();
        let i: HsInt8 = Arbitrary::arbitrary(g);
        let result: HaskellObj = unsafe { rts_mkInt8(&raw mut arg1, i) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_mkInt8(i: HsInt8) -> bool {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let result: HaskellObj = unsafe { transmute(sys::rts_mkInt8(&raw mut arg1, i)) };
        todo!()
    };

    let actual = {
        let mut arg1: Capability = todo!();
        let result: HaskellObj = unsafe { rts_mkInt8(&raw mut arg1, i) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_mkInt16() {
    let g = &mut Gen::new(100);

    let actual = {
        let arg1: Capability = todo!();
        let i: HsInt16 = Arbitrary::arbitrary(g);
        let result: HaskellObj = unsafe { rts_mkInt16(&raw mut arg1, i) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_mkInt16(i: HsInt16) -> bool {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let result: HaskellObj = unsafe { transmute(sys::rts_mkInt16(&raw mut arg1, i)) };
        todo!()
    };

    let actual = {
        let mut arg1: Capability = todo!();
        let result: HaskellObj = unsafe { rts_mkInt16(&raw mut arg1, i) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_mkInt32() {
    let g = &mut Gen::new(100);

    let actual = {
        let arg1: Capability = todo!();
        let i: HsInt32 = Arbitrary::arbitrary(g);
        let result: HaskellObj = unsafe { rts_mkInt32(&raw mut arg1, i) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_mkInt32(i: HsInt32) -> bool {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let result: HaskellObj = unsafe { transmute(sys::rts_mkInt32(&raw mut arg1, i)) };
        todo!()
    };

    let actual = {
        let mut arg1: Capability = todo!();
        let result: HaskellObj = unsafe { rts_mkInt32(&raw mut arg1, i) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_mkInt64() {
    let g = &mut Gen::new(100);

    let actual = {
        let arg1: Capability = todo!();
        let i: HsInt64 = Arbitrary::arbitrary(g);
        let result: HaskellObj = unsafe { rts_mkInt64(&raw mut arg1, i) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_mkInt64(i: HsInt64) -> bool {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let result: HaskellObj = unsafe { transmute(sys::rts_mkInt64(&raw mut arg1, i)) };
        todo!()
    };

    let actual = {
        let mut arg1: Capability = todo!();
        let result: HaskellObj = unsafe { rts_mkInt64(&raw mut arg1, i) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_mkWord() {
    let g = &mut Gen::new(100);

    let actual = {
        let arg1: Capability = todo!();
        let w: HsWord = Arbitrary::arbitrary(g);
        let result: HaskellObj = unsafe { rts_mkWord(&raw mut arg1, w) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_mkWord(w: HsWord) -> bool {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let result: HaskellObj = unsafe { transmute(sys::rts_mkWord(&raw mut arg1, w)) };
        todo!()
    };

    let actual = {
        let mut arg1: Capability = todo!();
        let result: HaskellObj = unsafe { rts_mkWord(&raw mut arg1, w) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_mkWord8() {
    let g = &mut Gen::new(100);

    let actual = {
        let arg1: Capability = todo!();
        let w: HsWord8 = Arbitrary::arbitrary(g);
        let result: HaskellObj = unsafe { rts_mkWord8(&raw mut arg1, w) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_mkWord8(w: HsWord8) -> bool {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let result: HaskellObj = unsafe { transmute(sys::rts_mkWord8(&raw mut arg1, w)) };
        todo!()
    };

    let actual = {
        let mut arg1: Capability = todo!();
        let result: HaskellObj = unsafe { rts_mkWord8(&raw mut arg1, w) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_mkWord16() {
    let g = &mut Gen::new(100);

    let actual = {
        let arg1: Capability = todo!();
        let w: HsWord16 = Arbitrary::arbitrary(g);
        let result: HaskellObj = unsafe { rts_mkWord16(&raw mut arg1, w) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_mkWord16(w: HsWord16) -> bool {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let result: HaskellObj = unsafe { transmute(sys::rts_mkWord16(&raw mut arg1, w)) };
        todo!()
    };

    let actual = {
        let mut arg1: Capability = todo!();
        let result: HaskellObj = unsafe { rts_mkWord16(&raw mut arg1, w) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_mkWord32() {
    let g = &mut Gen::new(100);

    let actual = {
        let arg1: Capability = todo!();
        let w: HsWord32 = Arbitrary::arbitrary(g);
        let result: HaskellObj = unsafe { rts_mkWord32(&raw mut arg1, w) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_mkWord32(w: HsWord32) -> bool {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let result: HaskellObj = unsafe { transmute(sys::rts_mkWord32(&raw mut arg1, w)) };
        todo!()
    };

    let actual = {
        let mut arg1: Capability = todo!();
        let result: HaskellObj = unsafe { rts_mkWord32(&raw mut arg1, w) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_mkWord64() {
    let g = &mut Gen::new(100);

    let actual = {
        let arg1: Capability = todo!();
        let w: HsWord64 = Arbitrary::arbitrary(g);
        let result: HaskellObj = unsafe { rts_mkWord64(&raw mut arg1, w) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_mkWord64(w: HsWord64) -> bool {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let result: HaskellObj = unsafe { transmute(sys::rts_mkWord64(&raw mut arg1, w)) };
        todo!()
    };

    let actual = {
        let mut arg1: Capability = todo!();
        let result: HaskellObj = unsafe { rts_mkWord64(&raw mut arg1, w) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_mkFloat() {
    let g = &mut Gen::new(100);

    let actual = {
        let arg1: Capability = todo!();
        let f: HsFloat = Arbitrary::arbitrary(g);
        let result: HaskellObj = unsafe { rts_mkFloat(&raw mut arg1, f) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_mkFloat(f: HsFloat) -> bool {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let result: HaskellObj = unsafe { transmute(sys::rts_mkFloat(&raw mut arg1, f)) };
        todo!()
    };

    let actual = {
        let mut arg1: Capability = todo!();
        let result: HaskellObj = unsafe { rts_mkFloat(&raw mut arg1, f) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_mkDouble() {
    let g = &mut Gen::new(100);

    let actual = {
        let arg1: Capability = todo!();
        let f: HsDouble = Arbitrary::arbitrary(g);
        let result: HaskellObj = unsafe { rts_mkDouble(&raw mut arg1, f) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_mkDouble(f: HsDouble) -> bool {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let result: HaskellObj = unsafe { transmute(sys::rts_mkDouble(&raw mut arg1, f)) };
        todo!()
    };

    let actual = {
        let mut arg1: Capability = todo!();
        let result: HaskellObj = unsafe { rts_mkDouble(&raw mut arg1, f) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_mkStablePtr() {
    let actual = {
        let arg1: Capability = todo!();
        let s: HsStablePtr = todo!();
        let result: HaskellObj = unsafe { rts_mkStablePtr(&raw mut arg1, s) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_mkStablePtr() {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let s: HsStablePtr = todo!();
        let result: HaskellObj = unsafe { transmute(sys::rts_mkStablePtr(&raw mut arg1, s)) };
        todo!()
    };

    let actual = {
        let mut arg1: Capability = todo!();
        let s: HsStablePtr = todo!();
        let result: HaskellObj = unsafe { rts_mkStablePtr(&raw mut arg1, s) };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_mkPtr() {
    let actual = {
        let arg1: Capability = todo!();
        let a: HsPtr = todo!();
        let result: HaskellObj = unsafe { rts_mkPtr(&raw mut arg1, a) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_mkPtr() {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let a: HsPtr = todo!();
        let result: HaskellObj = unsafe { transmute(sys::rts_mkPtr(&raw mut arg1, a)) };
        todo!()
    };

    let actual = {
        let mut arg1: Capability = todo!();
        let a: HsPtr = todo!();
        let result: HaskellObj = unsafe { rts_mkPtr(&raw mut arg1, a) };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_mkFunPtr() {
    let actual = {
        let arg1: Capability = todo!();
        let a: HsFunPtr = todo!();
        let result: HaskellObj = unsafe { rts_mkFunPtr(&raw mut arg1, a) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_mkFunPtr() {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let a: HsFunPtr = todo!();
        let result: HaskellObj = unsafe { transmute(sys::rts_mkFunPtr(&raw mut arg1, a)) };
        todo!()
    };

    let actual = {
        let mut arg1: Capability = todo!();
        let a: HsFunPtr = todo!();
        let result: HaskellObj = unsafe { rts_mkFunPtr(&raw mut arg1, a) };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_mkBool() {
    let g = &mut Gen::new(100);

    let actual = {
        let arg1: Capability = todo!();
        let b: HsBool = Arbitrary::arbitrary(g);
        let result: HaskellObj = unsafe { rts_mkBool(&raw mut arg1, b) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_mkBool(b: HsBool) -> bool {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let result: HaskellObj = unsafe { transmute(sys::rts_mkBool(&raw mut arg1, b)) };
        todo!()
    };

    let actual = {
        let mut arg1: Capability = todo!();
        let result: HaskellObj = unsafe { rts_mkBool(&raw mut arg1, b) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_mkString() {
    let g = &mut Gen::new(100);

    let actual = {
        let arg1: Capability = todo!();
        let mut s: c_char = Arbitrary::arbitrary(g);
        let result: HaskellObj = unsafe { rts_mkString(&raw mut arg1, &raw mut s) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_mkString(s: c_char) -> bool {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let mut s = s;
        let result: HaskellObj = unsafe { transmute(sys::rts_mkString(&raw mut arg1, &raw mut s)) };
        todo!()
    };

    let actual = {
        let mut arg1: Capability = todo!();
        let mut s = s;
        let result: HaskellObj = unsafe { rts_mkString(&raw mut arg1, &raw mut s) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_apply() {
    let actual = {
        let arg1: Capability = todo!();
        let arg2: HaskellObj = todo!();
        let arg3: HaskellObj = todo!();
        let result: HaskellObj = unsafe { rts_apply(&raw mut arg1, arg2, arg3) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_apply() {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let arg2: sys::HaskellObj = todo!();
        let arg3: sys::HaskellObj = todo!();
        let result: HaskellObj = unsafe { transmute(sys::rts_apply(&raw mut arg1, arg2, arg3)) };
        todo!()
    };

    let actual = {
        let mut arg1: Capability = todo!();
        let arg2: HaskellObj = todo!();
        let arg3: HaskellObj = todo!();
        let result: HaskellObj = unsafe { rts_apply(&raw mut arg1, arg2, arg3) };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_getChar() {
    let actual: HsChar = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getChar(arg1) }
    };

    let expected: HsChar = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_getChar() {
    let expected: HsChar = {
        let arg1: sys::HaskellObj = todo!();
        unsafe { sys::rts_getChar(arg1) }
    };

    let actual: HsChar = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getChar(arg1) }
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_getInt() {
    let actual: HsInt = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getInt(arg1) }
    };

    let expected: HsInt = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_getInt() {
    let expected: HsInt = {
        let arg1: sys::HaskellObj = todo!();
        unsafe { sys::rts_getInt(arg1) }
    };

    let actual: HsInt = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getInt(arg1) }
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_getInt8() {
    let actual: HsInt8 = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getInt8(arg1) }
    };

    let expected: HsInt8 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_getInt8() {
    let expected: HsInt8 = {
        let arg1: sys::HaskellObj = todo!();
        unsafe { sys::rts_getInt8(arg1) }
    };

    let actual: HsInt8 = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getInt8(arg1) }
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_getInt16() {
    let actual: HsInt16 = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getInt16(arg1) }
    };

    let expected: HsInt16 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_getInt16() {
    let expected: HsInt16 = {
        let arg1: sys::HaskellObj = todo!();
        unsafe { sys::rts_getInt16(arg1) }
    };

    let actual: HsInt16 = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getInt16(arg1) }
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_getInt32() {
    let actual: HsInt32 = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getInt32(arg1) }
    };

    let expected: HsInt32 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_getInt32() {
    let expected: HsInt32 = {
        let arg1: sys::HaskellObj = todo!();
        unsafe { sys::rts_getInt32(arg1) }
    };

    let actual: HsInt32 = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getInt32(arg1) }
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_getInt64() {
    let actual: HsInt64 = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getInt64(arg1) }
    };

    let expected: HsInt64 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_getInt64() {
    let expected: HsInt64 = {
        let arg1: sys::HaskellObj = todo!();
        unsafe { sys::rts_getInt64(arg1) }
    };

    let actual: HsInt64 = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getInt64(arg1) }
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_getWord() {
    let actual: HsWord = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getWord(arg1) }
    };

    let expected: HsWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_getWord() {
    let expected: HsWord = {
        let arg1: sys::HaskellObj = todo!();
        unsafe { sys::rts_getWord(arg1) }
    };

    let actual: HsWord = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getWord(arg1) }
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_getWord8() {
    let actual: HsWord8 = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getWord8(arg1) }
    };

    let expected: HsWord8 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_getWord8() {
    let expected: HsWord8 = {
        let arg1: sys::HaskellObj = todo!();
        unsafe { sys::rts_getWord8(arg1) }
    };

    let actual: HsWord8 = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getWord8(arg1) }
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_getWord16() {
    let actual: HsWord16 = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getWord16(arg1) }
    };

    let expected: HsWord16 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_getWord16() {
    let expected: HsWord16 = {
        let arg1: sys::HaskellObj = todo!();
        unsafe { sys::rts_getWord16(arg1) }
    };

    let actual: HsWord16 = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getWord16(arg1) }
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_getWord32() {
    let actual: HsWord32 = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getWord32(arg1) }
    };

    let expected: HsWord32 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_getWord32() {
    let expected: HsWord32 = {
        let arg1: sys::HaskellObj = todo!();
        unsafe { sys::rts_getWord32(arg1) }
    };

    let actual: HsWord32 = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getWord32(arg1) }
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_getWord64() {
    let actual: HsWord64 = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getWord64(arg1) }
    };

    let expected: HsWord64 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_getWord64() {
    let expected: HsWord64 = {
        let arg1: sys::HaskellObj = todo!();
        unsafe { sys::rts_getWord64(arg1) }
    };

    let actual: HsWord64 = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getWord64(arg1) }
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_getFloat() {
    let actual: HsFloat = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getFloat(arg1) }
    };

    let expected: HsFloat = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_getFloat() {
    let expected: HsFloat = {
        let arg1: sys::HaskellObj = todo!();
        unsafe { sys::rts_getFloat(arg1) }
    };

    let actual: HsFloat = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getFloat(arg1) }
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_getDouble() {
    let actual: HsDouble = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getDouble(arg1) }
    };

    let expected: HsDouble = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_getDouble() {
    let expected: HsDouble = {
        let arg1: sys::HaskellObj = todo!();
        unsafe { sys::rts_getDouble(arg1) }
    };

    let actual: HsDouble = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getDouble(arg1) }
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_getStablePtr() {
    let actual = {
        let arg1: HaskellObj = todo!();
        let result: HsStablePtr = unsafe { rts_getStablePtr(arg1) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_getStablePtr() {
    let expected = {
        let arg1: sys::HaskellObj = todo!();
        let result: HsStablePtr = unsafe { sys::rts_getStablePtr(arg1) };
        todo!()
    };

    let actual = {
        let arg1: HaskellObj = todo!();
        let result: HsStablePtr = unsafe { rts_getStablePtr(arg1) };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_getPtr() {
    let actual = {
        let arg1: HaskellObj = todo!();
        let result: HsPtr = unsafe { rts_getPtr(arg1) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_getPtr() {
    let expected = {
        let arg1: sys::HaskellObj = todo!();
        let result: HsPtr = unsafe { sys::rts_getPtr(arg1) };
        todo!()
    };

    let actual = {
        let arg1: HaskellObj = todo!();
        let result: HsPtr = unsafe { rts_getPtr(arg1) };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_getFunPtr() {
    let actual = {
        let arg1: HaskellObj = todo!();
        let result: HsFunPtr = unsafe { rts_getFunPtr(arg1) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_getFunPtr() {
    let expected = {
        let arg1: sys::HaskellObj = todo!();
        let result: HsFunPtr = unsafe { sys::rts_getFunPtr(arg1) };
        todo!()
    };

    let actual = {
        let arg1: HaskellObj = todo!();
        let result: HsFunPtr = unsafe { rts_getFunPtr(arg1) };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_getBool() {
    let actual: HsBool = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getBool(arg1) }
    };

    let expected: HsBool = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_getBool() {
    let expected: HsBool = {
        let arg1: sys::HaskellObj = todo!();
        unsafe { sys::rts_getBool(arg1) }
    };

    let actual: HsBool = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getBool(arg1) }
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_createGenThread() {
    let g = &mut Gen::new(100);

    let actual = {
        let cap: Capability = todo!();
        let stack_size: W_ = Arbitrary::arbitrary(g);
        let closure: StgClosure = todo!();
        let result: &StgTSO =
            unsafe { &*createGenThread(&raw mut cap, stack_size, &raw mut closure) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_createGenThread(stack_size: W_) -> bool {
    let expected = {
        let mut cap: sys::Capability = todo!();
        let mut closure: sys::StgClosure = todo!();

        let result: &StgTSO = unsafe {
            transmute(&*sys::createGenThread(
                &raw mut cap,
                stack_size,
                &raw mut closure,
            ))
        };

        todo!()
    };

    let actual = {
        let mut cap: Capability = todo!();
        let mut closure: StgClosure = todo!();
        let result: &StgTSO =
            unsafe { &*createGenThread(&raw mut cap, stack_size, &raw mut closure) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_eval() {
    let actual = {
        let mut arg1: Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let p: HaskellObj = todo!();
        let ret: HaskellObj = todo!();
        unsafe { rts_eval(&raw mut arg1, p, &raw mut ret) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_eval() {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let p: sys::HaskellObj = todo!();
        let mut ret: sys::HaskellObj = todo!();
        unsafe { sys::rts_eval(&raw mut arg1, p, &raw mut ret) };
        todo!()
    };

    let actual = {
        let mut arg1: Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let p: HaskellObj = todo!();
        let mut ret: HaskellObj = todo!();
        unsafe { rts_eval(&raw mut arg1, p, &raw mut ret) };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_eval_() {
    let g = &mut Gen::new(100);

    let actual = {
        let mut arg1: Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let p: HaskellObj = todo!();
        let stack_size: c_uint = Arbitrary::arbitrary(g);
        let ret: HaskellObj = todo!();
        unsafe { rts_eval_(&raw mut arg1, p, stack_size, &raw mut ret) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_eval_(stack_size: c_uint) -> bool {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let p: sys::HaskellObj = todo!();
        let mut ret: sys::HaskellObj = todo!();
        unsafe { sys::rts_eval_(&raw mut arg1, p, stack_size, &raw mut ret) };
        todo!()
    };

    let actual = {
        let mut arg1: Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let p: HaskellObj = todo!();
        let mut ret: HaskellObj = todo!();
        unsafe { rts_eval_(&raw mut arg1, p, stack_size, &raw mut ret) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_evalIO() {
    let actual = {
        let mut arg1: Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let p: HaskellObj = todo!();
        let ret: HaskellObj = todo!();
        unsafe { rts_evalIO(&raw mut arg1, p, &raw mut ret) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_evalIO() {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let p: sys::HaskellObj = todo!();
        let mut ret: sys::HaskellObj = todo!();
        unsafe { sys::rts_evalIO(&raw mut arg1, p, &raw mut ret) };
        todo!()
    };

    let actual = {
        let mut arg1: Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let p: HaskellObj = todo!();
        let mut ret: HaskellObj = todo!();
        unsafe { rts_evalIO(&raw mut arg1, p, &raw mut ret) };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_inCall() {
    let actual = {
        let mut arg1: Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let p: HaskellObj = todo!();
        let ret: HaskellObj = todo!();
        unsafe { rts_inCall(&raw mut arg1, p, &raw mut ret) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_inCall() {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let p: sys::HaskellObj = todo!();
        let mut ret: sys::HaskellObj = todo!();
        unsafe { sys::rts_inCall(&raw mut arg1, p, &raw mut ret) };
        todo!()
    };

    let actual = {
        let mut arg1: Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let p: HaskellObj = todo!();
        let mut ret: HaskellObj = todo!();
        unsafe { rts_inCall(&raw mut arg1, p, &raw mut ret) };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_evalStableIOMain() {
    let actual = {
        let mut arg1: Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let s: HsStablePtr = todo!();
        let ret: HsStablePtr = todo!();
        unsafe { rts_evalStableIOMain(&raw mut arg1, s, &raw mut ret) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_evalStableIOMain() {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let s: HsStablePtr = todo!();
        let mut ret: HsStablePtr = todo!();
        unsafe { sys::rts_evalStableIOMain(&raw mut arg1, s, &raw mut ret) };
        todo!()
    };

    let actual = {
        let mut arg1: Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let s: HsStablePtr = todo!();
        let mut ret: HsStablePtr = todo!();
        unsafe { rts_evalStableIOMain(&raw mut arg1, s, &raw mut ret) };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_evalStableIO() {
    let actual = {
        let mut arg1: Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let s: HsStablePtr = todo!();
        let ret: HsStablePtr = todo!();
        unsafe { rts_evalStableIO(&raw mut arg1, s, &raw mut ret) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_evalStableIO() {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let s: HsStablePtr = todo!();
        let mut ret: HsStablePtr = todo!();
        unsafe { sys::rts_evalStableIO(&raw mut arg1, s, &raw mut ret) };
        todo!()
    };

    let actual = {
        let mut arg1: Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let s: HsStablePtr = todo!();
        let mut ret: HsStablePtr = todo!();
        unsafe { rts_evalStableIO(&raw mut arg1, s, &raw mut ret) };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_evalLazyIO() {
    let actual = {
        let mut arg1: Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let p: HaskellObj = todo!();
        let ret: HaskellObj = todo!();
        unsafe { rts_evalLazyIO(&raw mut arg1, p, &raw mut ret) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_evalLazyIO() {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let p: sys::HaskellObj = todo!();
        let mut ret: sys::HaskellObj = todo!();
        unsafe { sys::rts_evalLazyIO(&raw mut arg1, p, &raw mut ret) };
        todo!()
    };

    let actual = {
        let mut arg1: Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let p: HaskellObj = todo!();
        let mut ret: HaskellObj = todo!();
        unsafe { rts_evalLazyIO(&raw mut arg1, p, &raw mut ret) };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_evalLazyIO_() {
    let g = &mut Gen::new(100);

    let actual = {
        let mut arg1: Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let p: HaskellObj = todo!();
        let stack_size: c_uint = Arbitrary::arbitrary(g);
        let ret: HaskellObj = todo!();
        unsafe { rts_evalLazyIO_(&raw mut arg1, p, stack_size, &raw mut ret) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_evalLazyIO_(stack_size: c_uint) -> bool {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let p: sys::HaskellObj = todo!();
        let mut ret: sys::HaskellObj = todo!();
        unsafe { sys::rts_evalLazyIO_(&raw mut arg1, p, stack_size, &raw mut ret) };
        todo!()
    };

    let actual = {
        let mut arg1: Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let p: HaskellObj = todo!();
        let mut ret: HaskellObj = todo!();
        unsafe { rts_evalLazyIO_(&raw mut arg1, p, stack_size, &raw mut ret) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_checkSchedStatus() {
    let g = &mut Gen::new(100);

    let actual = {
        let site: c_char = Arbitrary::arbitrary(g);
        let arg1: Capability = todo!();
        unsafe { rts_checkSchedStatus(&raw mut site, &raw mut arg1) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_checkSchedStatus(site: c_char) -> bool {
    let expected = {
        let site = site;
        let mut arg1: sys::Capability = todo!();
        unsafe { sys::rts_checkSchedStatus(&raw mut site, &raw mut arg1) };
        todo!()
    };

    let actual = {
        let mut site = site;
        let mut arg1: Capability = todo!();
        unsafe { rts_checkSchedStatus(&raw mut site, &raw mut arg1) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_getSchedStatus() {
    let actual: SchedulerStatus = {
        let cap: Capability = todo!();
        unsafe { rts_getSchedStatus(&raw mut cap) }
    };

    let expected: SchedulerStatus = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_getSchedStatus() {
    let expected: SchedulerStatus = {
        let mut cap: sys::Capability = todo!();
        unsafe { transmute(sys::rts_getSchedStatus(&raw mut cap)) }
    };

    let actual: SchedulerStatus = {
        let mut cap: Capability = todo!();
        unsafe { rts_getSchedStatus(&raw mut cap) }
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_lock() {
    let actual = {
        let result: &Capability = unsafe { &*rts_lock() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_lock() {
    let expected = {
        let result: &Capability = unsafe { transmute(&*sys::rts_lock()) };
        todo!()
    };

    let actual = {
        let result: &Capability = unsafe { &*rts_lock() };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_unlock() {
    let actual = {
        let token: Capability = todo!();
        unsafe { rts_unlock(&raw mut token) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_unlock() {
    let expected = {
        let mut token: sys::Capability = todo!();
        unsafe { sys::rts_unlock(&raw mut token) };
        todo!()
    };

    let actual = {
        let mut token: Capability = todo!();
        unsafe { rts_unlock(&raw mut token) };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_pauseTokenCapability() {
    let actual = {
        let pauseToken: PauseToken = todo!();
        let result: &Capability = unsafe { &*pauseTokenCapability(&raw mut pauseToken) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_pauseTokenCapability() {
    let expected = {
        let mut pauseToken: sys::PauseToken = todo!();
        let result: &Capability =
            unsafe { transmute(&*sys::pauseTokenCapability(&raw mut pauseToken)) };
        todo!()
    };

    let actual = {
        let mut pauseToken: PauseToken = todo!();
        let result: &Capability = unsafe { &*pauseTokenCapability(&raw mut pauseToken) };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_pause() {
    let actual = {
        let result: &PauseToken = unsafe { &*rts_pause() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_pause() {
    let expected = {
        let result: &PauseToken = unsafe { transmute(&*sys::rts_pause()) };
        todo!()
    };

    let actual = {
        let result: &PauseToken = unsafe { &*rts_pause() };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_resume() {
    let actual = {
        let pauseToken: PauseToken = todo!();
        unsafe { rts_resume(&raw mut pauseToken) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_resume() {
    let expected = {
        let mut pauseToken: sys::PauseToken = todo!();
        unsafe { sys::rts_resume(&raw mut pauseToken) };
        todo!()
    };

    let actual = {
        let mut pauseToken: PauseToken = todo!();
        unsafe { rts_resume(&raw mut pauseToken) };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_isPaused() {
    let actual: bool = { unsafe { rts_isPaused() } };
    let expected: bool = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_rts_isPaused() {
    let expected: bool = { unsafe { sys::rts_isPaused() } };
    let actual: bool = { unsafe { rts_isPaused() } };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_listThreads() {
    let actual = {
        let cb: ListThreadsCb = todo!();
        let user: c_void = todo!();
        unsafe { rts_listThreads(cb, &raw mut user) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_listThreads() {
    let expected = {
        let cb: sys::ListThreadsCb = todo!();
        let mut user: c_void = todo!();
        unsafe { sys::rts_listThreads(cb, &raw mut user) };
        todo!()
    };

    let actual = {
        let cb: ListThreadsCb = todo!();
        let mut user: c_void = todo!();
        unsafe { rts_listThreads(cb, &raw mut user) };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_listMiscRoots() {
    let actual = {
        let cb: ListRootsCb = todo!();
        let user: c_void = todo!();
        unsafe { rts_listMiscRoots(cb, &raw mut user) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_listMiscRoots() {
    let expected = {
        let cb: sys::ListRootsCb = todo!();
        let mut user: c_void = todo!();
        unsafe { sys::rts_listMiscRoots(cb, &raw mut user) };
        todo!()
    };

    let actual = {
        let cb: ListRootsCb = todo!();
        let mut user: c_void = todo!();
        unsafe { rts_listMiscRoots(cb, &raw mut user) };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_try_putmvar() {
    let g = &mut Gen::new(100);

    let actual = {
        let capability: c_int = Arbitrary::arbitrary(g);
        let sp: HsStablePtr = todo!();
        unsafe { hs_try_putmvar(capability, sp) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_hs_try_putmvar(capability: c_int) -> bool {
    let expected = {
        let sp: HsStablePtr = todo!();
        unsafe { sys::hs_try_putmvar(capability, sp) };
        todo!()
    };

    let actual = {
        let sp: HsStablePtr = todo!();
        unsafe { hs_try_putmvar(capability, sp) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_try_putmvar_with_value() {
    let g = &mut Gen::new(100);

    let actual = {
        let capability: c_int = Arbitrary::arbitrary(g);
        let sp: HsStablePtr = todo!();
        let value: StgClosure = todo!();
        unsafe { hs_try_putmvar_with_value(capability, sp, &raw mut value) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_hs_try_putmvar_with_value(capability: c_int) -> bool {
    let expected = {
        let sp: HsStablePtr = todo!();
        let mut value: sys::StgClosure = todo!();
        unsafe { sys::hs_try_putmvar_with_value(capability, sp, &raw mut value) };
        todo!()
    };

    let actual = {
        let sp: HsStablePtr = todo!();
        let mut value: StgClosure = todo!();
        unsafe { hs_try_putmvar_with_value(capability, sp, &raw mut value) };
        todo!()
    };

    actual == expected
}
