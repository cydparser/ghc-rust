use crate::ffi::rts::foreign_exports::ForeignExportsList;
use crate::ffi::rts::stable_ptr::getStablePtr;
use crate::ffi::stg::types::{StgPtr, StgStablePtr};
use crate::linker_internals::{_ObjectCode, ObjectCode};
use crate::prelude::*;
use crate::rts_utils::stgMallocBytes;

#[cfg(test)]
mod tests;

static mut pending: *mut ForeignExportsList =
    null::<ForeignExportsList>() as *mut ForeignExportsList;

static mut loading_obj: *mut ObjectCode = null::<ObjectCode>() as *mut ObjectCode;

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn registerForeignExports(mut exports: *mut ForeignExportsList) {
    (*exports).next = pending;
    (*exports).oc = loading_obj as *mut _ObjectCode;
    pending = exports;
}

unsafe fn foreignExportsLoadingObject(mut oc: *mut ObjectCode) {
    loading_obj = oc;
}

unsafe fn foreignExportsFinishedLoadingObject() {
    loading_obj = null_mut::<ObjectCode>();
    processForeignExports();
}

unsafe fn processForeignExports() {
    while !pending.is_null() {
        let mut cur = pending;
        pending = (*cur).next;

        if !(*cur).oc.is_null() {
            (*cur).stable_ptrs = stgMallocBytes(
                (size_of::<*mut StgStablePtr>() as size_t).wrapping_mul((*cur).n_entries as size_t),
                b"foreignExportStablePtr\0" as *const u8 as *const c_char as *mut c_char,
            ) as *mut *mut StgStablePtr;

            let mut i = 0 as c_int;

            while i < (*cur).n_entries {
                let mut sptr =
                    getStablePtr(*(&raw mut (*cur).exports as *mut StgPtr).offset(i as isize))
                        as *mut StgStablePtr;

                if !(*cur).oc.is_null() {
                    let ref mut fresh5 = *(*cur).stable_ptrs.offset(i as isize);
                    *fresh5 = sptr;
                }

                i += 1;
            }

            (*cur).next = (*(*cur).oc).foreign_exports;
            (*(*cur).oc).foreign_exports = cur;
        } else {
            let mut i_0 = 0 as c_int;

            while i_0 < (*cur).n_entries {
                getStablePtr(*(&raw mut (*cur).exports as *mut StgPtr).offset(i_0 as isize));
                i_0 += 1;
            }
        }
    }
}
