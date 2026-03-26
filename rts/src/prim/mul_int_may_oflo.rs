use crate::ffi::stg::{HALF_NEG_INT, HALF_POS_INT, I_, W_};
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_mulIntMayOflo(mut a: W_, mut b: W_) -> W_ {
    return ({
        let mut c: I_ = 0;

        if a as I_ <= HALF_NEG_INT
            || a >= HALF_POS_INT as W_
            || b as I_ <= HALF_NEG_INT
            || b >= HALF_POS_INT as W_
        {
            c = 1 as I_;
        } else {
            c = 0 as I_;
        }

        c
    }) as W_;
}
