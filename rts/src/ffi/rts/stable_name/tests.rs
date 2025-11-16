use super::*;
use std::mem::offset_of;

#[cfg(feature = "sys")]
#[test]
fn sys_size_snEntry() {
    assert_eq!(size_of::<sys::snEntry>(), size_of::<snEntry>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of snEntry"][size_of::<snEntry>() - 24usize];
    ["Alignment of snEntry"][align_of::<snEntry>() - 8usize];
    ["Offset of field: snEntry::addr"][offset_of!(snEntry, addr) - 0usize];
    ["Offset of field: snEntry::old"][offset_of!(snEntry, old) - 8usize];
    ["Offset of field: snEntry::sn_obj"][offset_of!(snEntry, sn_obj) - 16usize];
};
