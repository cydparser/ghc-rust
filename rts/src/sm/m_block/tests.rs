use super::*;

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_mblocks_allocated_layout() {
    assert_eq!(
        size_of_val(unsafe { &mblocks_allocated }),
        size_of_val(unsafe { &sys::mblocks_allocated })
    );
    assert_eq!(
        align_of_val(unsafe { &mblocks_allocated }),
        align_of_val(unsafe { &sys::mblocks_allocated })
    );
}
