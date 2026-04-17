use crate::check_unload::{insertOCSectionIndices, loaded_objects};
use crate::ffi::rts::_assertFail;
use crate::ffi::rts::linker::{OBJECT_READY, OBJECT_UNLOADED, pathchar};
use crate::ffi::rts::messages::{barf, debugBelch};
use crate::foreign_exports::{foreignExportsFinishedLoadingObject, foreignExportsLoadingObject};
use crate::linker_internals::{
    _ObjectCode, DYNAMIC_OBJECT, NativeCodeRange, ObjectCode, linker_mutex, lookupObjectByPath,
    mkOc,
};
use crate::prelude::*;
use crate::profiling::{ccs_mutex, refreshProfilingCCSs};
use crate::rts_flags::RtsFlags;
use crate::rts_utils::{stgFree, stgMallocBytes};

unsafe fn copyErrmsg(mut errmsg_dest: *mut *mut c_char, mut errmsg: *mut c_char) {
    if errmsg.is_null() {
        errmsg = c"loadNativeObj_POSIX: unknown error".as_ptr();
    }

    *errmsg_dest = stgMallocBytes(
        strlen(errmsg).wrapping_add(1 as usize),
        c"loadNativeObj_POSIX".as_ptr(),
    ) as *mut c_char;

    strcpy(*errmsg_dest, errmsg);
}

unsafe fn freeNativeCode_POSIX(mut nc: *mut ObjectCode) {
    if (pthread_mutex_lock(&raw mut linker_mutex) == 11) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/linker/LoadNativeObjPosix.c".as_ptr(), 79);
    }

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
    let mut dlopen_mode: i32 = 0;
    let mut nc = null_mut::<ObjectCode>();
    let mut hdl = null_mut::<c_void>();
    let mut retval = null_mut::<c_void>();

    if (pthread_mutex_lock(&raw mut linker_mutex) == 11) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/linker/LoadNativeObjPosix.c".as_ptr(), 117);
    }

    if RtsFlags.DebugFlags.linker {
        debugBelch(c"loadNativeObj_POSIX %s\n".as_ptr(), path);
    }

    retval = NULL;

    let mut existing_oc = lookupObjectByPath(path);

    if !existing_oc.is_null() && (*existing_oc).status as u32 != OBJECT_UNLOADED as i32 as u32 {
        if (*existing_oc).r#type as u32 == DYNAMIC_OBJECT as i32 as u32 {
            retval = (*existing_oc).dlopen_handle;
        } else {
            copyErrmsg(
                errmsg,
                c"loadNativeObj_POSIX: already loaded as non-dynamic object".as_ptr(),
            );
        }
    } else {
        nc = mkOc(
            DYNAMIC_OBJECT,
            path,
            null_mut::<c_char>(),
            0,
            false,
            null_mut::<pathchar>(),
            0,
        );

        load_now = false;
        load_now = false;

        loop {
            foreignExportsLoadingObject(nc);

            let mut __r = pthread_mutex_lock(&raw mut ccs_mutex);

            if __r != 0 {
                barf(
                    c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
                    c"rts/linker/LoadNativeObjPosix.c".as_ptr(),
                    166,
                    __r,
                );
            }

            dlopen_mode = if load_now as i32 != 0 {
                RTLD_NOW
            } else {
                RTLD_LAZY
            };
            hdl = dlopen(path, dlopen_mode | RTLD_LOCAL);
            (*nc).dlopen_handle = hdl;
            (*nc).status = OBJECT_READY;

            if pthread_mutex_unlock(&raw mut ccs_mutex) != 0 {
                barf(
                    c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                    c"rts/linker/LoadNativeObjPosix.c".as_ptr(),
                    175,
                );
            }

            foreignExportsFinishedLoadingObject();

            if hdl.is_null() {
                if load_now {
                    load_now = false;
                } else {
                    copyErrmsg(errmsg, dlerror());
                    break;
                }
            } else {
                (*nc).nc_ranges = null_mut::<NativeCodeRange>();
                (*nc).unloadable = false;
                insertOCSectionIndices(nc);
                (*nc).next_loaded_object = loaded_objects as *mut _ObjectCode;
                loaded_objects = nc;
                retval = (*nc).dlopen_handle;
                refreshProfilingCCSs();
                break;
            }
        }
    }

    if RtsFlags.DebugFlags.linker {
        debugBelch(c"loadNativeObj_POSIX result=%p\n".as_ptr(), retval);
    }

    return retval;
}
