use crate::prelude::*;

#[cfg(test)]
mod tests;

extern "C" {
    pub(crate) static mut simple_ccall_adjustor: u8;

    pub(crate) static mut simple_ccall_adjustor_context: u8;

    pub(crate) static mut simple_ccall_adjustor_end: u8;

    pub(crate) static mut complex_float_ccall_adjustor_context: u8;

    pub(crate) static mut complex_float_ccall_adjustor_end: u8;

    pub(crate) static mut complex_float_ccall_adjustor: u8;

    pub(crate) static mut complex_nofloat_ccall_adjustor_context: u8;

    pub(crate) static mut complex_nofloat_ccall_adjustor_end: u8;

    pub(crate) static mut complex_nofloat_ccall_adjustor: u8;
}

static mut simple_ccall_adjustor_template: AdjustorTemplate = unsafe {
    AdjustorTemplate {
        code_start: &raw const simple_ccall_adjustor as *mut u8,
        code_end: &raw const simple_ccall_adjustor_end as *mut u8,
        context_ptr: &raw const simple_ccall_adjustor_context as *mut u8
            as *mut *const AdjustorContext,
    }
};

static mut simple_ccall_pool: *mut AdjustorPool = null_mut::<AdjustorPool>();

static mut complex_float_ccall_adjustor_template: AdjustorTemplate = unsafe {
    AdjustorTemplate {
        code_start: &raw const complex_float_ccall_adjustor as *mut u8,
        code_end: &raw const complex_float_ccall_adjustor_end as *mut u8,
        context_ptr: &raw const complex_float_ccall_adjustor_context as *mut u8
            as *mut *const AdjustorContext,
    }
};

static mut complex_float_ccall_pool: *mut AdjustorPool = null_mut::<AdjustorPool>();

static mut complex_nofloat_ccall_adjustor_template: AdjustorTemplate = unsafe {
    AdjustorTemplate {
        code_start: &raw const complex_nofloat_ccall_adjustor as *mut u8,
        code_end: &raw const complex_nofloat_ccall_adjustor_end as *mut u8,
        context_ptr: &raw const complex_nofloat_ccall_adjustor_context as *mut u8
            as *mut *const AdjustorContext,
    }
};

static mut complex_nofloat_ccall_pool: *mut AdjustorPool = null_mut::<AdjustorPool>();

pub(crate) unsafe fn initAdjustors() {
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

    if *typeString.offset(0) as i32 == '\0' as i32
        || *typeString.offset(1) as i32 == '\0' as i32
        || *typeString.offset(2) as i32 == '\0' as i32
        || *typeString.offset(3) as i32 == '\0' as i32
    {
        return alloc_adjustor(simple_ccall_pool, &raw mut context as *mut c_void);
    } else {
        let mut fourthFloating = *typeString.offset(3) as i32 == 'f' as i32
            || *typeString.offset(3) as i32 == 'd' as i32;

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
