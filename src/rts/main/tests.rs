use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_main(
    argc: ::core::ffi::c_int,
    argv: ::core::ffi::c_char,
    main_closure: StgClosure,
    rts_config: RtsConfig,
) -> bool {
    let expected = unsafe {
        transmute(sys::hs_main(
            argc.into(),
            &mut &mut argv.into(),
            &mut main_closure.into(),
            rts_config.into(),
        ))
    };
    let actual = unsafe { super::hs_main(argc, &mut &mut argv, &mut main_closure, rts_config) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_main() {
    let argc = Default::default();
    let mut argv = Default::default();
    let mut main_closure = Default::default();
    let rts_config = Default::default();
    unsafe { super::hs_main(argc, &mut &mut argv, &mut main_closure, rts_config) };
    todo!("assert")
}
