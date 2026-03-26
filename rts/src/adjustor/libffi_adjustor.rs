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
    *pclosure = ffi_closure_alloc(size_of::<ffi_closure>() as size_t, code) as *mut ffi_closure;

    return ffi_prep_closure_loc(*pclosure, cif, fun, user_data, *code);
}

static mut allocatedExecs: *mut HashTable = null::<HashTable>() as *mut HashTable;

unsafe fn initAdjustors() {
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

    if r as c_uint != FFI_OK as c_int as c_uint {
        barf(
            b"ffi_alloc_prep_closure failed: %d\0" as *const u8 as *const c_char,
            r as c_uint,
        );
    }

    if !(*exec_ret).is_null() {
        insertHashTable(allocatedExecs, *exec_ret as StgWord, writ as *const c_void);
    }

    return writ;
}

unsafe fn exec_to_writable(mut exec: AdjustorExecutable) -> AdjustorWritable {
    let mut writ = null_mut::<c_void>();
    writ = lookupHashTable(allocatedExecs, exec as StgWord) as AdjustorWritable;

    if writ.is_null() {
        barf(b"exec_to_writable: not found\0" as *const u8 as *const c_char);
    }

    return writ;
}

unsafe fn free_adjustor(mut exec: AdjustorExecutable) {
    let mut writ = null_mut::<c_void>();
    let mut cl = null_mut::<ffi_closure>();
    writ = exec_to_writable(exec);
    cl = writ as *mut ffi_closure;
    removeHashTable(allocatedExecs, exec as StgWord, writ as *const c_void);
    ffi_closure_free(cl as *mut c_void);
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
    match c as c_int {
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
            barf(
                b"char_to_ffi_type: unknown type '%c'\0" as *const u8 as *const c_char,
                c as c_int,
            );
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
    let mut n_args: uint32_t = 0;
    let mut i: uint32_t = 0;
    let mut result_type = null_mut::<ffi_type>();
    let mut cl = null_mut::<ffi_closure>();
    let mut r: c_int = 0;
    let mut code = null_mut::<c_void>();
    n_args = strlen(typeString).wrapping_sub(1 as size_t) as uint32_t;

    cif = stgMallocBytes(
        size_of::<ffi_cif>() as size_t,
        b"createAdjustor\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut ffi_cif;

    arg_types = stgMallocBytes(
        (n_args as size_t).wrapping_mul(size_of::<*mut ffi_type>() as size_t),
        b"createAdjustor\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut *mut ffi_type;

    result_type = char_to_ffi_type(*typeString.offset(0 as c_int as isize));
    i = 0 as uint32_t;

    while i < n_args {
        let ref mut fresh5 = *arg_types.offset(i as isize);
        *fresh5 = char_to_ffi_type(*typeString.offset(i.wrapping_add(1 as uint32_t) as isize));
        i = i.wrapping_add(1);
    }

    r = ffi_prep_cif(
        cif,
        FFI_DEFAULT_ABI,
        n_args as c_uint,
        result_type,
        arg_types,
    ) as c_int;

    if r != FFI_OK as c_int {
        barf(
            b"ffi_prep_cif failed: %d\0" as *const u8 as *const c_char,
            r,
        );
    }

    cl = allocate_adjustor(
        &raw mut code,
        cif,
        transmute::<StgFunPtr, *mut c_void>(wptr),
        hptr as *mut c_void,
    ) as *mut ffi_closure;

    if cl.is_null() {
        barf(b"createAdjustor: failed to allocate memory\0" as *const u8 as *const c_char);
    }

    return code;
}
