use super::*;
use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
use crate::utils::test::*;
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::ffi::{c_char, c_int, c_uint, c_void};
use std::mem::transmute;
use std::ptr::{null, null_mut};
#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_main(
    argc: c_int,
    argv: c_char,
    main_closure: StgClosure,
    rts_config: RtsConfig,
) -> bool {
    let expected = unsafe {
        sys::hs_main(
            argc,
            &mut &mut argv,
            &mut main_closure.into(),
            rts_config.into(),
        )
    };
    let actual = unsafe { hs_main(argc, &mut &mut argv, &mut main_closure, rts_config) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_main() {
    let argc = Default::default();
    let mut argv = null_mut();
    let mut main_closure = null_mut();
    let rts_config = todo!();
    unsafe { hs_main(argc, &mut &mut argv, &mut main_closure, rts_config) };
    todo!("assert")
}
