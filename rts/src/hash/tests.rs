use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_hashtable_layout() {
    assert_eq!(
        offset_of!(hashtable, _address),
        offset_of!(sys::hashtable, _address)
    );
    assert_eq!(size_of::<hashtable>(), size_of::<sys::hashtable>());
    assert_eq!(align_of::<hashtable>(), align_of::<sys::hashtable>());
}
