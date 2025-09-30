use crate::prelude::*;

#[cfg(test)]
mod tests;

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_genericRaise"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn genericRaise(sig: c_int) -> c_int {
    #[cfg(feature = "sys")]
    unsafe {
        sys::genericRaise(sig)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("genericRaise")
}
