use crate::ffi::rts::_assertFail;
use crate::ffi::rts::foreign_exports::ForeignExportsList;
use crate::ffi::rts::stable_ptr::getStablePtr;
use crate::ffi::stg::types::{StgPtr, StgStablePtr};
use crate::linker_internals::{_ObjectCode, ObjectCode};
use crate::prelude::*;
use crate::rts_utils::stgMallocBytes;

#[cfg(test)]
mod tests;

static mut pending: *mut ForeignExportsList = null_mut::<ForeignExportsList>();

static mut loading_obj: *mut ObjectCode = null_mut::<ObjectCode>();

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn registerForeignExports(mut exports: *mut ForeignExportsList) {
    if (*exports).next.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/ForeignExports.c".as_ptr(), 63);
    }

    if (*exports).oc.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/ForeignExports.c".as_ptr(), 64);
    }

    (*exports).next = pending;
    (*exports).oc = loading_obj as *mut _ObjectCode;
    pending = exports;
}

unsafe fn foreignExportsLoadingObject(mut oc: *mut ObjectCode) {
    if loading_obj.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/ForeignExports.c".as_ptr(), 82);
    }

    loading_obj = oc;
}

unsafe fn foreignExportsFinishedLoadingObject() {
    if !loading_obj.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/ForeignExports.c".as_ptr(), 88);
    }

    loading_obj = null_mut::<ObjectCode>();
    processForeignExports();
}

unsafe fn processForeignExports() {
    while !pending.is_null() {
        let mut cur = pending;
        pending = (*cur).next;

        if (*cur).stable_ptrs.is_null() as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/ForeignExports.c".as_ptr(), 102);
        }

        if !(*cur).oc.is_null() {
            (*cur).stable_ptrs = stgMallocBytes(
                (size_of::<*mut StgStablePtr>() as usize).wrapping_mul((*cur).n_entries as usize),
                c"foreignExportStablePtr".as_ptr(),
            ) as *mut *mut StgStablePtr;

            let mut i = 0;

            while i < (*cur).n_entries {
                let mut sptr =
                    getStablePtr(*(&raw mut (*cur).exports as *mut StgPtr).offset(i as isize))
                        as *mut StgStablePtr;

                if !(*cur).oc.is_null() {
                    let ref mut fresh12 = *(*cur).stable_ptrs.offset(i as isize);
                    *fresh12 = sptr;
                }

                i += 1;
            }

            (*cur).next = (*(*cur).oc).foreign_exports;
            (*(*cur).oc).foreign_exports = cur;
        } else {
            let mut i_0 = 0;

            while i_0 < (*cur).n_entries {
                getStablePtr(*(&raw mut (*cur).exports as *mut StgPtr).offset(i_0 as isize));

                i_0 += 1;
            }
        }
    }
}
