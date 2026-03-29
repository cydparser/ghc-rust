use crate::ffi::stg::types::{StgDouble, StgFloat, StgInt, StgInt64};
use crate::ffi::stg::{ASSIGN_Int64, I_, W_};
use crate::prelude::*;

#[cfg(test)]
mod tests;

union C2RustUnnamed_6 {
    d: f64,
    i: [u32; 2],
}

union C2RustUnnamed_7 {
    f: f32,
    i: i32,
}

const MY_DMINEXP: i32 = -1021 - 53 - 1;

const DHIGHBIT: i32 = 0x100000;

const DMSBIT: u32 = 0x80000000;

const MY_FMINEXP: i32 = -125 - 24 - 1;

const FHIGHBIT: i32 = 0x800000;

const FMSBIT: u32 = 0x80000000;

const L: i32 = 0;

const H: i32 = 1;

#[inline]
unsafe fn truncExponent(mut e: I_) -> i32 {
    if (e > 2147483647) as i32 as i64 != 0 {
        e = INT_MAX as I_;
    }

    if (e < (-2147483647 - 1) as I_) as i32 as i64 != 0 {
        e = INT_MIN as I_;
    }

    return e as i32;
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn __word_encodeDouble(mut j: W_, mut e: I_) -> StgDouble {
    let mut r: StgDouble = 0.;
    r = j as StgDouble;

    if r != 0.0f64 {
        r = ldexp(r as f64, truncExponent(e)) as StgDouble;
    }

    return r;
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn __int_encodeDouble(mut j: I_, mut e: I_) -> StgDouble {
    let mut r: StgDouble = 0.;
    r = (if j >= 0 { j } else { -j }) as StgDouble;

    if r != 0.0f64 {
        r = ldexp(r as f64, truncExponent(e)) as StgDouble;
    }

    if j < 0 {
        r = -r;
    }

    return r;
}

unsafe fn __int_encodeFloat(mut j: I_, mut e: I_) -> StgFloat {
    let mut r: StgFloat = 0.;
    r = (if j >= 0 { j } else { -j }) as StgFloat;

    if r as f64 != 0.0f64 {
        r = ldexp(r as f64, truncExponent(e)) as StgFloat;
    }

    if j < 0 {
        r = -r;
    }

    return r;
}

unsafe fn __word_encodeFloat(mut j: W_, mut e: I_) -> StgFloat {
    let mut r: StgFloat = 0.;
    r = j as StgFloat;

    if r as f64 != 0.0f64 {
        r = ldexp(r as f64, truncExponent(e)) as StgFloat;
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
    let mut low: u32 = 0;
    let mut high: u32 = 0;
    let mut sign: i32 = 0;
    let mut iexp: i32 = 0;
    let mut u = C2RustUnnamed_6 { d: 0. };
    u.d = dbl as f64;
    low = u.i[L as usize];
    high = u.i[H as usize];

    if low == 0 && high & !DMSBIT == 0 {
        *man_low = 0;
        *man_high = 0;
        *exp = 0;
    } else {
        iexp = (high >> 20 as i32 & 0x7ff as u32).wrapping_add(MY_DMINEXP as u32) as i32;
        sign = high as i32;
        high &= (DHIGHBIT - 1) as u32;

        if iexp != MY_DMINEXP {
            high |= DHIGHBIT as u32;
        } else {
            iexp += 1;

            while high & DHIGHBIT as u32 == 0 {
                high <<= 1;

                if low & DMSBIT != 0 {
                    high = high.wrapping_add(1);
                }

                low <<= 1;
                iexp -= 1;
            }
        }

        *exp = iexp as I_;
        *man_low = low as W_;
        *man_high = high as W_;
        *man_sign = (if sign < 0 { -1 } else { 1 }) as I_;
    };
}

unsafe fn __decodeDouble_Int64(mantissa: *mut StgInt64, dbl: StgDouble) -> StgInt {
    let mut man_sign: I_ = 0;
    let mut man_high: W_ = 0;
    let mut man_low: W_ = 0;
    let mut exp: I_ = 0;

    __decodeDouble_2Int(
        &raw mut man_sign,
        &raw mut man_high,
        &raw mut man_low,
        &raw mut exp,
        dbl,
    );

    ASSIGN_Int64(
        mantissa as *mut W_,
        ((man_high as StgInt64) << 32 | man_low as StgInt64) * man_sign,
    );

    return exp as StgInt;
}

unsafe fn __decodeFloat_Int(mut man: *mut I_, mut exp: *mut I_, mut flt: StgFloat) {
    let mut high: i32 = 0;
    let mut sign: i32 = 0;
    let mut u = C2RustUnnamed_7 { f: 0. };
    u.f = flt as f32;
    high = u.i;

    if high as u32 & !FMSBIT == 0 {
        *man = 0;
        *exp = 0;
    } else {
        *exp = ((high >> 23 & 0xff) + MY_FMINEXP) as I_;
        sign = high;
        high &= FHIGHBIT - 1;

        if *exp != MY_FMINEXP as I_ {
            high |= FHIGHBIT;
        } else {
            *exp += 1;

            while high & FHIGHBIT == 0 {
                high <<= 1;
                *exp -= 1;
            }
        }

        *man = high as I_;

        if sign < 0 {
            *man = -*man;
        }
    };
}
