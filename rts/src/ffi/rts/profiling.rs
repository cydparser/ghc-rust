use crate::ffi::rts::prof::ccs::{CostCentre, CostCentreStack};
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn registerCcList(cc_list: *mut *mut CostCentre) {
    sys! {
        registerCcList(cc_list as * mut * mut sys::CostCentre)
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn registerCcsList(cc_list: *mut *mut CostCentreStack) {
    sys! {
        registerCcsList(cc_list as * mut * mut sys::CostCentreStack)
    }
}
