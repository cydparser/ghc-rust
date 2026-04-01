use crate::ffi::rts::messages::barf;
use crate::ffi::rts::storage::gc::{AdjustorExecutable, AdjustorWritable};
use crate::ffi::stg::types::{StgFunPtr, StgStablePtr, StgWord};
use crate::ffi::{
    FFI_OK, ffi_cif, ffi_closure, ffi_closure_alloc, ffi_closure_free, ffi_prep_cif,
    ffi_prep_closure_loc, ffi_status, ffi_type, ffi_type_double, ffi_type_float, ffi_type_pointer,
    ffi_type_sint8, ffi_type_sint16, ffi_type_sint32, ffi_type_sint64, ffi_type_uint8,
    ffi_type_uint16, ffi_type_uint32, ffi_type_uint64, ffi_type_void,
};
use crate::ffitarget::FFI_DEFAULT_ABI;
use crate::hash::{HashTable, allocHashTable, insertHashTable, lookupHashTable, removeHashTable};
use crate::prelude::*;
use crate::rts_utils::{stgFree, stgMallocBytes};
use crate::sm::storage::sm_mutex;
use crate::stable_ptr::freeStablePtr;

#[cfg(test)]
mod tests;

unsafe fn ffi_alloc_prep_closure(
    mut pclosure: *mut *mut ffi_closure,
    mut cif: *mut ffi_cif,
    mut fun: Option<
        unsafe extern "C" fn(*mut ffi_cif, *mut c_void, *mut *mut c_void, *mut c_void) -> (),
    >,
    mut user_data: *mut c_void,
    mut code: *mut *mut c_void,
) -> ffi_status {
    *pclosure = ffi_closure_alloc(size_of::<ffi_closure>() as usize, code) as *mut ffi_closure;

    return ffi_prep_closure_loc(*pclosure, cif, fun, user_data, *code);
}

static mut allocatedExecs: *mut HashTable = null_mut::<HashTable>();

pub(crate) unsafe fn initAdjustors() {
    allocatedExecs = allocHashTable();
}

unsafe fn allocate_adjustor(
    mut exec_ret: *mut AdjustorExecutable,
    mut cif: *mut ffi_cif,
    mut wptr: *mut c_void,
    mut hptr: *mut c_void,
) -> AdjustorWritable {
    let mut writ = null_mut::<c_void>();

    let mut r = ffi_alloc_prep_closure(
        &raw mut writ as *mut *mut ffi_closure,
        cif,
        transmute::<
            *mut c_void,
            Option<
                unsafe extern "C" fn(
                    *mut ffi_cif,
                    *mut c_void,
                    *mut *mut c_void,
                    *mut c_void,
                ) -> (),
            >,
        >(wptr),
        hptr,
        exec_ret as *mut *mut c_void,
    );

    if r as u32 != FFI_OK as i32 as u32 {
        barf(c"ffi_alloc_prep_closure failed: %d".as_ptr(), r as u32);
    }

    if !(*exec_ret).is_null() {
        let mut __r = pthread_mutex_lock(&raw mut sm_mutex);

        if __r != 0 {
            barf(
                c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
                c"rts/adjustor/LibffiAdjustor.c".as_ptr(),
                48,
                __r,
            );
        }

        insertHashTable(allocatedExecs, *exec_ret as StgWord, writ as *const c_void);

        if pthread_mutex_unlock(&raw mut sm_mutex) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/adjustor/LibffiAdjustor.c".as_ptr(),
                50,
            );
        }
    }

    return writ;
}

unsafe fn exec_to_writable(mut exec: AdjustorExecutable) -> AdjustorWritable {
    let mut writ = null_mut::<c_void>();
    let mut __r = pthread_mutex_lock(&raw mut sm_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/adjustor/LibffiAdjustor.c".as_ptr(),
            59,
            __r,
        );
    }

    writ = lookupHashTable(allocatedExecs, exec as StgWord) as AdjustorWritable;

    if writ.is_null() {
        if pthread_mutex_unlock(&raw mut sm_mutex) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/adjustor/LibffiAdjustor.c".as_ptr(),
                61,
            );
        }

        barf(c"exec_to_writable: not found".as_ptr());
    }

    if pthread_mutex_unlock(&raw mut sm_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/adjustor/LibffiAdjustor.c".as_ptr(),
            64,
        );
    }

    return writ;
}

unsafe fn free_adjustor(mut exec: AdjustorExecutable) {
    let mut writ = null_mut::<c_void>();
    let mut cl = null_mut::<ffi_closure>();
    writ = exec_to_writable(exec);
    cl = writ as *mut ffi_closure;

    let mut __r = pthread_mutex_lock(&raw mut sm_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/adjustor/LibffiAdjustor.c".as_ptr(),
            73,
            __r,
        );
    }

    removeHashTable(allocatedExecs, exec as StgWord, writ as *const c_void);
    ffi_closure_free(cl as *mut c_void);

    if pthread_mutex_unlock(&raw mut sm_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/adjustor/LibffiAdjustor.c".as_ptr(),
            76,
        );
    }
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn freeHaskellFunctionPtr(mut ptr: *mut c_void) {
    let mut cl = null_mut::<ffi_closure>();
    cl = exec_to_writable(ptr as AdjustorExecutable) as *mut ffi_closure;
    freeStablePtr((*cl).user_data as StgStablePtr);
    stgFree((*(*cl).cif).arg_types as *mut c_void);
    stgFree((*cl).cif as *mut c_void);
    free_adjustor(ptr as AdjustorExecutable);
}

unsafe fn char_to_ffi_type(mut c: c_char) -> *mut ffi_type {
    match c as i32 {
        118 => return &raw mut ffi_type_void,
        102 => return &raw mut ffi_type_float,
        100 => return &raw mut ffi_type_double,
        76 => return &raw mut ffi_type_sint64,
        108 => return &raw mut ffi_type_uint64,
        87 => return &raw mut ffi_type_sint32,
        119 => return &raw mut ffi_type_uint32,
        83 => return &raw mut ffi_type_sint16,
        115 => return &raw mut ffi_type_uint16,
        66 => return &raw mut ffi_type_sint8,
        98 => return &raw mut ffi_type_uint8,
        112 => return &raw mut ffi_type_pointer,
        _ => {
            barf(c"char_to_ffi_type: unknown type '%c'".as_ptr(), c as i32);
        }
    };
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn createAdjustor(
    mut hptr: StgStablePtr,
    mut wptr: StgFunPtr,
    mut typeString: *mut c_char,
) -> *mut c_void {
    let mut cif = null_mut::<ffi_cif>();
    let mut arg_types = null_mut::<*mut ffi_type>();
    let mut n_args: u32 = 0;
    let mut i: u32 = 0;
    let mut result_type = null_mut::<ffi_type>();
    let mut cl = null_mut::<ffi_closure>();
    let mut r: i32 = 0;
    let mut code = null_mut::<c_void>();
    n_args = strlen(typeString).wrapping_sub(1 as usize) as u32;
    cif = stgMallocBytes(size_of::<ffi_cif>() as usize, c"createAdjustor".as_ptr()) as *mut ffi_cif;

    arg_types = stgMallocBytes(
        (n_args as usize).wrapping_mul(size_of::<*mut ffi_type>() as usize),
        c"createAdjustor".as_ptr(),
    ) as *mut *mut ffi_type;

    result_type = char_to_ffi_type(*typeString.offset(0));
    i = 0;

    while i < n_args {
        let ref mut fresh5 = *arg_types.offset(i as isize);

        *fresh5 = char_to_ffi_type(*typeString.offset(i.wrapping_add(1 as u32) as isize));

        i = i.wrapping_add(1);
    }

    r = ffi_prep_cif(cif, FFI_DEFAULT_ABI, n_args as u32, result_type, arg_types) as i32;

    if r != FFI_OK as i32 {
        barf(c"ffi_prep_cif failed: %d".as_ptr(), r);
    }

    cl = allocate_adjustor(
        &raw mut code,
        cif,
        transmute::<StgFunPtr, *mut c_void>(wptr),
        hptr as *mut c_void,
    ) as *mut ffi_closure;

    if cl.is_null() {
        barf(c"createAdjustor: failed to allocate memory".as_ptr());
    }

    return code;
}
