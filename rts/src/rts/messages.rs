use crate::prelude::*;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rtsOutOfBoundsAccess"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rtsOutOfBoundsAccess() -> ! {
    unsafe { sys::rtsOutOfBoundsAccess() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rtsMemcpyRangeOverlap"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rtsMemcpyRangeOverlap() -> ! {
    unsafe { sys::rtsMemcpyRangeOverlap() }
}
