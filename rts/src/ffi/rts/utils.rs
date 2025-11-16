use crate::prelude::*;

#[cfg(test)]
mod tests;

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn genericRaise(sig: c_int) -> c_int {
    #[cfg(feature = "sys")]
    unsafe {
        sys::genericRaise(sig)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("genericRaise")
}
