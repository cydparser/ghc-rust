use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_HsFunPtr_layout() {
    assert_eq!(size_of::<HsFunPtr>(), size_of::<HsFunPtr>());
    assert_eq!(align_of::<HsFunPtr>(), align_of::<HsFunPtr>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsStablePtr_layout() {
    assert_eq!(size_of::<HsStablePtr>(), size_of::<HsStablePtr>());
    assert_eq!(align_of::<HsStablePtr>(), align_of::<HsStablePtr>());
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
