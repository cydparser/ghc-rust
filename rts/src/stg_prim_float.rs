use crate::ffi::stg::types::{StgDouble, StgFloat, StgInt, StgInt64};
use crate::ffi::stg::{ASSIGN_Int64, I_, W_};
use crate::prelude::*;

#[cfg(test)]
mod tests;

union C2RustUnnamed_6 {
    d: c_double,
    i: [c_uint; 2],
}

union C2RustUnnamed_7 {
    f: c_float,
    i: c_int,
}

const MY_DMINEXP: c_int = -(1021 as c_int) - 53 as c_int - 1 as c_int;

const DHIGHBIT: c_int = 0x100000 as c_int;

const DMSBIT: c_uint = 0x80000000 as c_uint;

const MY_FMINEXP: c_int = -(125 as c_int) - 24 as c_int - 1 as c_int;

const FHIGHBIT: c_int = 0x800000 as c_int;

const FMSBIT: c_uint = 0x80000000 as c_uint;

const L: c_int = 0 as c_int;

const H: c_int = 1 as c_int;

#[inline]
unsafe fn truncExponent(mut e: I_) -> c_int {
    if (e > 2147483647 as I_) as c_int as c_long != 0 {
        e = INT_MAX as I_;
    }

    if (e < (-(2147483647 as c_int) - 1 as c_int) as I_) as c_int as c_long != 0 {
        e = INT_MIN as I_;
    }

    return e as c_int;
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn __word_encodeDouble(mut j: W_, mut e: I_) -> StgDouble {
    let mut r: StgDouble = 0.;
    r = j as StgDouble;

    if r != 0.0f64 {
        r = ldexp(r as c_double, truncExponent(e)) as StgDouble;
    }

    return r;
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn __int_encodeDouble(mut j: I_, mut e: I_) -> StgDouble {
    let mut r: StgDouble = 0.;
    r = (if j >= 0 as I_ { j } else { -j }) as StgDouble;

    if r != 0.0f64 {
        r = ldexp(r as c_double, truncExponent(e)) as StgDouble;
    }

    if j < 0 as I_ {
        r = -r;
    }

    return r;
}

unsafe fn __int_encodeFloat(mut j: I_, mut e: I_) -> StgFloat {
    let mut r: StgFloat = 0.;
    r = (if j >= 0 as I_ { j } else { -j }) as StgFloat;

    if r as c_double != 0.0f64 {
        r = ldexp(r as c_double, truncExponent(e)) as StgFloat;
    }

    if j < 0 as I_ {
        r = -r;
    }

    return r;
}

unsafe fn __word_encodeFloat(mut j: W_, mut e: I_) -> StgFloat {
    let mut r: StgFloat = 0.;
    r = j as StgFloat;

    if r as c_double != 0.0f64 {
        r = ldexp(r as c_double, truncExponent(e)) as StgFloat;
    }

    return r;
}

unsafe fn __decodeDouble_2Int(
    mut man_sign: *mut I_,
    mut man_high: *mut W_,
    mut man_low: *mut W_,
    mut exp: *mut I_,
    mut dbl: StgDouble,
) {
    let mut low: c_uint = 0;
    let mut high: c_uint = 0;
    let mut sign: c_int = 0;
    let mut iexp: c_int = 0;
    let mut u = C2RustUnnamed_6 { d: 0. };
    u.d = dbl as c_double;
    low = u.i[L as usize];
    high = u.i[H as usize];

    if low == 0 as c_uint && high & !DMSBIT == 0 as c_uint {
        *man_low = 0 as W_;
        *man_high = 0 as W_;
        *exp = 0 as I_;
    } else {
        iexp = (high >> 20 as c_int & 0x7ff as c_uint).wrapping_add(MY_DMINEXP as c_uint) as c_int;
        sign = high as c_int;
        high &= (DHIGHBIT - 1 as c_int) as c_uint;

        if iexp != MY_DMINEXP {
            high |= DHIGHBIT as c_uint;
        } else {
            iexp += 1;

            while high & DHIGHBIT as c_uint == 0 {
                high <<= 1 as c_int;

                if low & DMSBIT != 0 {
                    high = high.wrapping_add(1);
                }

                low <<= 1 as c_int;
                iexp -= 1;
            }
        }

        *exp = iexp as I_;
        *man_low = low as W_;
        *man_high = high as W_;

        *man_sign = (if sign < 0 as c_int {
            -(1 as c_int)
        } else {
            1 as c_int
        }) as I_;
    };
}

unsafe fn __decodeDouble_Int64(mantissa: *mut StgInt64, dbl: StgDouble) -> StgInt {
    let mut man_sign: I_ = 0 as I_;
    let mut man_high: W_ = 0 as W_;
    let mut man_low: W_ = 0 as W_;
    let mut exp: I_ = 0 as I_;

    __decodeDouble_2Int(
        &raw mut man_sign,
        &raw mut man_high,
        &raw mut man_low,
        &raw mut exp,
        dbl,
    );

    ASSIGN_Int64(
        mantissa as *mut W_,
        ((man_high as StgInt64) << 32 as c_int | man_low as StgInt64) * man_sign,
    );

    return exp as StgInt;
}

unsafe fn __decodeFloat_Int(mut man: *mut I_, mut exp: *mut I_, mut flt: StgFloat) {
    let mut high: c_int = 0;
    let mut sign: c_int = 0;
    let mut u = C2RustUnnamed_7 { f: 0. };
    u.f = flt as c_float;
    high = u.i;

    if high as c_uint & !FMSBIT == 0 as c_uint {
        *man = 0 as I_;
        *exp = 0 as I_;
    } else {
        *exp = ((high >> 23 as c_int & 0xff as c_int) + MY_FMINEXP) as I_;
        sign = high;
        high &= FHIGHBIT - 1 as c_int;

        if *exp != MY_FMINEXP as I_ {
            high |= FHIGHBIT;
        } else {
            *exp += 1;

            while high & FHIGHBIT == 0 {
                high <<= 1 as c_int;
                *exp -= 1;
            }
        }

        *man = high as I_;

        if sign < 0 as c_int {
            *man = -*man;
        }
    };
}
