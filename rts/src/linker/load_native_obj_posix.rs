use crate::check_unload::{insertOCSectionIndices, loaded_objects};
use crate::ffi::rts::linker::{OBJECT_READY, OBJECT_UNLOADED, pathchar};
use crate::foreign_exports::{foreignExportsFinishedLoadingObject, foreignExportsLoadingObject};
use crate::linker_internals::{
    _ObjectCode, DYNAMIC_OBJECT, NativeCodeRange, ObjectCode, lookupObjectByPath, mkOc,
};
use crate::prelude::*;
use crate::rts_utils::{stgFree, stgMallocBytes};

unsafe fn copyErrmsg(mut errmsg_dest: *mut *mut c_char, mut errmsg: *mut c_char) {
    if errmsg.is_null() {
        errmsg =
            b"loadNativeObj_POSIX: unknown error\0" as *const u8 as *const c_char as *mut c_char;
    }

    *errmsg_dest = stgMallocBytes(
        strlen(errmsg).wrapping_add(1 as size_t),
        b"loadNativeObj_POSIX\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut c_char;

    strcpy(*errmsg_dest, errmsg);
}

unsafe fn freeNativeCode_POSIX(mut nc: *mut ObjectCode) {
    dlclose((*nc).dlopen_handle);

    let mut ncr = (*nc).nc_ranges;

    while !ncr.is_null() {
        let mut last_ncr = ncr;
        ncr = (*ncr).next as *mut NativeCodeRange;
        stgFree(last_ncr as *mut c_void);
    }
}

unsafe fn loadNativeObj_POSIX(
    mut path: *mut pathchar,
    mut errmsg: *mut *mut c_char,
) -> *mut c_void {
    let mut load_now: bool = false;
    let mut dlopen_mode: c_int = 0;
    let mut nc = null_mut::<ObjectCode>();
    let mut hdl = null_mut::<c_void>();
    let mut retval = null_mut::<c_void>();
    retval = NULL;

    let mut existing_oc = lookupObjectByPath(path);

    if !existing_oc.is_null()
        && (*existing_oc).status as c_uint != OBJECT_UNLOADED as c_int as c_uint
    {
        if (*existing_oc).r#type as c_uint == DYNAMIC_OBJECT as c_int as c_uint {
            retval = (*existing_oc).dlopen_handle;
        } else {
            copyErrmsg(
                errmsg,
                b"loadNativeObj_POSIX: already loaded as non-dynamic object\0" as *const u8
                    as *const c_char as *mut c_char,
            );
        }
    } else {
        nc = mkOc(
            DYNAMIC_OBJECT,
            path,
            null_mut::<c_char>(),
            0 as c_int,
            r#false != 0,
            null_mut::<pathchar>(),
            0 as c_int,
        );

        load_now = false;
        load_now = r#false != 0;

        loop {
            foreignExportsLoadingObject(nc);

            dlopen_mode = if load_now as c_int != 0 {
                RTLD_NOW
            } else {
                RTLD_LAZY
            };

            hdl = dlopen(path, dlopen_mode | RTLD_LOCAL);
            (*nc).dlopen_handle = hdl;
            (*nc).status = OBJECT_READY;
            foreignExportsFinishedLoadingObject();

            if hdl.is_null() {
                if load_now {
                    load_now = r#false != 0;
                } else {
                    copyErrmsg(errmsg, dlerror());
                    break;
                }
            } else {
                (*nc).nc_ranges = null_mut::<NativeCodeRange>();
                (*nc).unloadable = r#false != 0;
                insertOCSectionIndices(nc);
                (*nc).next_loaded_object = loaded_objects as *mut _ObjectCode;
                loaded_objects = nc;
                retval = (*nc).dlopen_handle;
                break;
            }
        }
    }

    return retval;
}
