use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn genericRaise(sig: c_int) -> c_int {
    sys! {
        genericRaise(sig)
    }
}
