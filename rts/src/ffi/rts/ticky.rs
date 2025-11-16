use crate::prelude::*;
use crate::rts::storage::info_tables::StgInfoTable;
use crate::stg::types::{StgInt, StgWord};

#[cfg(test)]
mod tests;

/// cbindgen:no-export
#[repr(C)]
pub struct _StgEntCounter {
    registeredp: StgWord,
    arity: StgInt,
    allocd: StgInt,
    str_: *mut c_char,
    arg_kinds: *mut c_char,
    ticky_json: *mut c_char,
    info: *mut StgInfoTable,
    entry_count: StgInt,
    allocs: StgInt,
    link: *mut _StgEntCounter,
}

#[cfg(feature = "sys")]
impl From<_StgEntCounter> for sys::_StgEntCounter {
    fn from(x: _StgEntCounter) -> Self {
        unsafe { transmute(x) }
    }
}

pub(crate) type StgEntCounter = _StgEntCounter;

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn requestTickyCounterSamples() {
    #[cfg(feature = "sys")]
    unsafe {
        sys::requestTickyCounterSamples()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("requestTickyCounterSamples")
}
