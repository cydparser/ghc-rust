use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_size_spEntry() {
    assert_eq!(size_of::<sys::spEntry>(), size_of::<spEntry>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of spEntry"][size_of::<spEntry>() - 8usize];
    ["Alignment of spEntry"][align_of::<spEntry>() - 8usize];
    ["Offset of field: spEntry::addr"][offset_of!(spEntry, addr) - 0usize];
};
