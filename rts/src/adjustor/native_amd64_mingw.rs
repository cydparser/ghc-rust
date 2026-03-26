use crate::adjustor::adjustor_pool::{
    AdjustorContext, AdjustorPool, AdjustorTemplate, alloc_adjustor, free_adjustor,
    new_adjustor_pool_from_template,
};
use crate::ffi::stg::types::{StgFunPtr, StgStablePtr};
use crate::prelude::*;
use crate::stable_ptr::freeStablePtr;

#[cfg(test)]
mod tests;

extern "C" {
    pub(crate) static mut simple_ccall_adjustor: uint8_t;

    pub(crate) static mut simple_ccall_adjustor_context: uint8_t;

    pub(crate) static mut simple_ccall_adjustor_end: uint8_t;

    pub(crate) static mut complex_float_ccall_adjustor_context: uint8_t;

    pub(crate) static mut complex_float_ccall_adjustor_end: uint8_t;

    pub(crate) static mut complex_float_ccall_adjustor: uint8_t;

    pub(crate) static mut complex_nofloat_ccall_adjustor_context: uint8_t;

    pub(crate) static mut complex_nofloat_ccall_adjustor_end: uint8_t;

    pub(crate) static mut complex_nofloat_ccall_adjustor: uint8_t;
}

static mut simple_ccall_adjustor_template: AdjustorTemplate = unsafe {
    AdjustorTemplate {
        code_start: &raw const simple_ccall_adjustor as *mut uint8_t,
        code_end: &raw const simple_ccall_adjustor_end as *mut uint8_t,
        context_ptr: &raw const simple_ccall_adjustor_context as *mut uint8_t
            as *mut *const AdjustorContext,
    }
};

static mut simple_ccall_pool: *mut AdjustorPool = null::<AdjustorPool>() as *mut AdjustorPool;

static mut complex_float_ccall_adjustor_template: AdjustorTemplate = unsafe {
    AdjustorTemplate {
        code_start: &raw const complex_float_ccall_adjustor as *mut uint8_t,
        code_end: &raw const complex_float_ccall_adjustor_end as *mut uint8_t,
        context_ptr: &raw const complex_float_ccall_adjustor_context as *mut uint8_t
            as *mut *const AdjustorContext,
    }
};

static mut complex_float_ccall_pool: *mut AdjustorPool =
    null::<AdjustorPool>() as *mut AdjustorPool;

static mut complex_nofloat_ccall_adjustor_template: AdjustorTemplate = unsafe {
    AdjustorTemplate {
        code_start: &raw const complex_nofloat_ccall_adjustor as *mut uint8_t,
        code_end: &raw const complex_nofloat_ccall_adjustor_end as *mut uint8_t,
        context_ptr: &raw const complex_nofloat_ccall_adjustor_context as *mut uint8_t
            as *mut *const AdjustorContext,
    }
};

static mut complex_nofloat_ccall_pool: *mut AdjustorPool =
    null::<AdjustorPool>() as *mut AdjustorPool;

unsafe fn initAdjustors() {
    simple_ccall_pool = new_adjustor_pool_from_template(&raw const simple_ccall_adjustor_template);
    complex_float_ccall_pool =
        new_adjustor_pool_from_template(&raw const complex_float_ccall_adjustor_template);
    complex_nofloat_ccall_pool =
        new_adjustor_pool_from_template(&raw const complex_nofloat_ccall_adjustor_template);
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn createAdjustor(
    mut hptr: StgStablePtr,
    mut wptr: StgFunPtr,
    mut typeString: *mut c_char,
) -> *mut c_void {
    let mut context = AdjustorContext {
        hptr: hptr,
        wptr: wptr,
    };

    if *typeString.offset(0 as c_int as isize) as c_int == '\0' as i32
        || *typeString.offset(1 as c_int as isize) as c_int == '\0' as i32
        || *typeString.offset(2 as c_int as isize) as c_int == '\0' as i32
        || *typeString.offset(3 as c_int as isize) as c_int == '\0' as i32
    {
        return alloc_adjustor(simple_ccall_pool, &raw mut context as *mut c_void);
    } else {
        let mut fourthFloating = *typeString.offset(3 as c_int as isize) as c_int == 'f' as i32
            || *typeString.offset(3 as c_int as isize) as c_int == 'd' as i32;

        if fourthFloating {
            return alloc_adjustor(complex_float_ccall_pool, &raw mut context as *mut c_void);
        } else {
            return alloc_adjustor(complex_nofloat_ccall_pool, &raw mut context as *mut c_void);
        }
    };
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn freeHaskellFunctionPtr(mut ptr: *mut c_void) {
    let mut context = AdjustorContext {
        hptr: null_mut::<c_void>(),
        wptr: None,
    };

    free_adjustor(ptr, &raw mut context as *mut c_void);
    freeStablePtr(context.hptr);
}
