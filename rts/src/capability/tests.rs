use super::*;

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_MainCapability_layout() {
    assert_eq!(
        size_of_val(unsafe { &MainCapability }),
        size_of_val(unsafe { &sys::MainCapability })
    );
    assert_eq!(
        align_of_val(unsafe { &MainCapability }),
        align_of_val(unsafe { &sys::MainCapability })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_n_capabilities_layout() {
    assert_eq!(
        size_of_val(unsafe { &n_capabilities }),
        size_of_val(unsafe { &sys::n_capabilities })
    );
    assert_eq!(
        align_of_val(unsafe { &n_capabilities }),
        align_of_val(unsafe { &sys::n_capabilities })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_enabled_capabilities_layout() {
    assert_eq!(
        size_of_val(unsafe { &enabled_capabilities }),
        size_of_val(unsafe { &sys::enabled_capabilities })
    );
    assert_eq!(
        align_of_val(unsafe { &enabled_capabilities }),
        align_of_val(unsafe { &sys::enabled_capabilities })
    );
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_unsafeGetMyCapability() {
    let actual = {
        let result: &Capability = unsafe { &*rts_unsafeGetMyCapability() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_unsafeGetMyCapability() {
    let expected = {
        let result: &Capability = unsafe { transmute(&*sys::rts_unsafeGetMyCapability()) };

        todo!()
    };

    let actual = {
        let result: &Capability = unsafe { &*rts_unsafeGetMyCapability() };
        todo!()
    };
    assert_eq!(actual, expected);
}
