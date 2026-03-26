use crate::ffi::hs_ffi::HsBool;
use crate::ffi::rts::linker::pathchar;
use crate::path_utils::pathsize;
use crate::prelude::*;
use crate::rts_utils::stgMallocBytes;

pub(crate) const pathsize: usize = size_of::<c_char>();

unsafe fn pathdup(mut path: *const pathchar) -> *mut pathchar {
    let mut ret = null_mut::<pathchar>();

    ret = stgMallocBytes(
        strlen(path as *const c_char).wrapping_add(1 as size_t),
        b"pathdup\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut pathchar;

    strcpy(ret as *mut c_char, path as *const c_char);

    return ret;
}

unsafe fn pathdir(mut path: *const pathchar) -> *mut pathchar {
    let mut ret = null_mut::<pathchar>();
    let mut dirName: *const pathchar = dirname(path as *mut c_char);
    let mut memberLen = strlen(dirName as *const c_char);

    ret = stgMallocBytes(
        pathsize.wrapping_mul(memberLen.wrapping_add(2 as size_t)),
        b"pathdir(path)\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut pathchar;

    strcpy(ret as *mut c_char, dirName as *const c_char);
    *ret.offset(memberLen as isize) = '/' as i32 as pathchar;
    *ret.offset(memberLen.wrapping_add(1 as size_t) as isize) = '\0' as i32 as pathchar;

    return ret;
}

unsafe fn mkPath(mut path: *const c_char) -> *mut pathchar {
    return pathdup(path as *const pathchar);
}

unsafe fn endsWithPath(mut base: *const pathchar, mut str: *const pathchar) -> HsBool {
    let mut blen = strlen(base as *const c_char) as c_int;
    let mut slen = strlen(str as *const c_char) as c_int;

    return (blen >= slen
        && 0 as c_int
            == strcmp(
                base.offset(blen as isize).offset(-(slen as isize)),
                str as *const c_char,
            )) as c_int as HsBool;
}
