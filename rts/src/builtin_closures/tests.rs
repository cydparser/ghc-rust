use super::*;

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_stg_INTLIKE_closure_layout() {
    assert_eq!(
        size_of_val(unsafe { &stg_INTLIKE_closure }),
        size_of_val(unsafe { &sys::stg_INTLIKE_closure })
    );
    assert_eq!(
        align_of_val(unsafe { &stg_INTLIKE_closure }),
        align_of_val(unsafe { &sys::stg_INTLIKE_closure })
    );
}
