use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_HsChar_layout() {
    assert_eq!(size_of::<HsChar>(), size_of::<sys::HsChar>());
    assert_eq!(align_of::<HsChar>(), align_of::<sys::HsChar>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsInt_layout() {
    assert_eq!(size_of::<HsInt>(), size_of::<sys::HsInt>());
    assert_eq!(align_of::<HsInt>(), align_of::<sys::HsInt>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsInt8_layout() {
    assert_eq!(size_of::<HsInt8>(), size_of::<sys::HsInt8>());
    assert_eq!(align_of::<HsInt8>(), align_of::<sys::HsInt8>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsInt16_layout() {
    assert_eq!(size_of::<HsInt16>(), size_of::<sys::HsInt16>());
    assert_eq!(align_of::<HsInt16>(), align_of::<sys::HsInt16>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsInt32_layout() {
    assert_eq!(size_of::<HsInt32>(), size_of::<sys::HsInt32>());
    assert_eq!(align_of::<HsInt32>(), align_of::<sys::HsInt32>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsInt64_layout() {
    assert_eq!(size_of::<HsInt64>(), size_of::<sys::HsInt64>());
    assert_eq!(align_of::<HsInt64>(), align_of::<sys::HsInt64>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsWord_layout() {
    assert_eq!(size_of::<HsWord>(), size_of::<sys::HsWord>());
    assert_eq!(align_of::<HsWord>(), align_of::<sys::HsWord>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsWord8_layout() {
    assert_eq!(size_of::<HsWord8>(), size_of::<sys::HsWord8>());
    assert_eq!(align_of::<HsWord8>(), align_of::<sys::HsWord8>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsWord16_layout() {
    assert_eq!(size_of::<HsWord16>(), size_of::<sys::HsWord16>());
    assert_eq!(align_of::<HsWord16>(), align_of::<sys::HsWord16>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsWord32_layout() {
    assert_eq!(size_of::<HsWord32>(), size_of::<sys::HsWord32>());
    assert_eq!(align_of::<HsWord32>(), align_of::<sys::HsWord32>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsWord64_layout() {
    assert_eq!(size_of::<HsWord64>(), size_of::<sys::HsWord64>());
    assert_eq!(align_of::<HsWord64>(), align_of::<sys::HsWord64>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsFloat_layout() {
    assert_eq!(size_of::<HsFloat>(), size_of::<sys::HsFloat>());
    assert_eq!(align_of::<HsFloat>(), align_of::<sys::HsFloat>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsDouble_layout() {
    assert_eq!(size_of::<HsDouble>(), size_of::<sys::HsDouble>());
    assert_eq!(align_of::<HsDouble>(), align_of::<sys::HsDouble>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsBool_layout() {
    assert_eq!(size_of::<HsBool>(), size_of::<sys::HsBool>());
    assert_eq!(align_of::<HsBool>(), align_of::<sys::HsBool>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsPtr_layout() {
    assert_eq!(size_of::<HsPtr>(), size_of::<sys::HsPtr>());
    assert_eq!(align_of::<HsPtr>(), align_of::<sys::HsPtr>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsFunPtr_layout() {
    assert_eq!(size_of::<HsFunPtr>(), size_of::<sys::HsFunPtr>());
    assert_eq!(align_of::<HsFunPtr>(), align_of::<sys::HsFunPtr>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsStablePtr_layout() {
    assert_eq!(size_of::<HsStablePtr>(), size_of::<sys::HsStablePtr>());
    assert_eq!(align_of::<HsStablePtr>(), align_of::<sys::HsStablePtr>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HS_BOOL_TRUE_eq() {
    assert_eq!(HS_BOOL_TRUE, sys::HS_BOOL_TRUE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_HS_BOOL_TRUE_layout() {
    assert_eq!(size_of_val(&HS_BOOL_TRUE), size_of_val(&sys::HS_BOOL_TRUE));
    assert_eq!(
        align_of_val(&HS_BOOL_TRUE),
        align_of_val(&sys::HS_BOOL_TRUE)
    );
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_perform_gc() {
    let actual = {
        unsafe { hs_perform_gc() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_hs_perform_gc() {
    let expected = {
        unsafe { sys::hs_perform_gc() };
        todo!()
    };

    let actual = {
        unsafe { hs_perform_gc() };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_lock_stable_ptr_table() {
    let actual = {
        unsafe { hs_lock_stable_ptr_table() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_hs_lock_stable_ptr_table() {
    let expected = {
        unsafe { sys::hs_lock_stable_ptr_table() };
        todo!()
    };

    let actual = {
        unsafe { hs_lock_stable_ptr_table() };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_lock_stable_tables() {
    let actual = {
        unsafe { hs_lock_stable_tables() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_hs_lock_stable_tables() {
    let expected = {
        unsafe { sys::hs_lock_stable_tables() };
        todo!()
    };

    let actual = {
        unsafe { hs_lock_stable_tables() };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_unlock_stable_ptr_table() {
    let actual = {
        unsafe { hs_unlock_stable_ptr_table() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_hs_unlock_stable_ptr_table() {
    let expected = {
        unsafe { sys::hs_unlock_stable_ptr_table() };
        todo!()
    };

    let actual = {
        unsafe { hs_unlock_stable_ptr_table() };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_unlock_stable_tables() {
    let actual = {
        unsafe { hs_unlock_stable_tables() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_hs_unlock_stable_tables() {
    let expected = {
        unsafe { sys::hs_unlock_stable_tables() };
        todo!()
    };

    let actual = {
        unsafe { hs_unlock_stable_tables() };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_free_stable_ptr() {
    let actual = {
        let sp: HsStablePtr = todo!();
        unsafe { hs_free_stable_ptr(sp) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_hs_free_stable_ptr() {
    let expected = {
        let sp: HsStablePtr = todo!();
        unsafe { sys::hs_free_stable_ptr(sp) };
        todo!()
    };

    let actual = {
        let sp: HsStablePtr = todo!();
        unsafe { hs_free_stable_ptr(sp) };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_free_stable_ptr_unsafe() {
    let actual = {
        let sp: HsStablePtr = todo!();
        unsafe { hs_free_stable_ptr_unsafe(sp) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_hs_free_stable_ptr_unsafe() {
    let expected = {
        let sp: HsStablePtr = todo!();
        unsafe { sys::hs_free_stable_ptr_unsafe(sp) };
        todo!()
    };

    let actual = {
        let sp: HsStablePtr = todo!();
        unsafe { hs_free_stable_ptr_unsafe(sp) };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_free_fun_ptr() {
    let actual = {
        let fp: HsFunPtr = todo!();
        unsafe { hs_free_fun_ptr(fp) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_hs_free_fun_ptr() {
    let expected = {
        let fp: HsFunPtr = todo!();
        unsafe { sys::hs_free_fun_ptr(fp) };
        todo!()
    };

    let actual = {
        let fp: HsFunPtr = todo!();
        unsafe { hs_free_fun_ptr(fp) };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_thread_done() {
    let actual = {
        unsafe { hs_thread_done() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_hs_thread_done() {
    let expected = {
        unsafe { sys::hs_thread_done() };
        todo!()
    };

    let actual = {
        unsafe { hs_thread_done() };
        todo!()
    };
    assert_eq!(actual, expected);
}
