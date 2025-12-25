#![cfg_attr(not(feature = "sys"), expect(unused_imports))]
use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_GEN_eq() {
    assert_eq!(ARG_GEN, sys::ARG_GEN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_GEN_layout() {
    assert_eq!(size_of_val(&ARG_GEN), size_of_val(&sys::ARG_GEN));
    assert_eq!(align_of_val(&ARG_GEN), align_of_val(&sys::ARG_GEN));
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_GEN_BIG_eq() {
    assert_eq!(ARG_GEN_BIG, sys::ARG_GEN_BIG);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_GEN_BIG_layout() {
    assert_eq!(size_of_val(&ARG_GEN_BIG), size_of_val(&sys::ARG_GEN_BIG));
    assert_eq!(align_of_val(&ARG_GEN_BIG), align_of_val(&sys::ARG_GEN_BIG));
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_BCO_eq() {
    assert_eq!(ARG_BCO, sys::ARG_BCO);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_BCO_layout() {
    assert_eq!(size_of_val(&ARG_BCO), size_of_val(&sys::ARG_BCO));
    assert_eq!(align_of_val(&ARG_BCO), align_of_val(&sys::ARG_BCO));
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_NONE_eq() {
    assert_eq!(ARG_NONE, sys::ARG_NONE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_NONE_layout() {
    assert_eq!(size_of_val(&ARG_NONE), size_of_val(&sys::ARG_NONE));
    assert_eq!(align_of_val(&ARG_NONE), align_of_val(&sys::ARG_NONE));
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_N_eq() {
    assert_eq!(ARG_N, sys::ARG_N);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_N_layout() {
    assert_eq!(size_of_val(&ARG_N), size_of_val(&sys::ARG_N));
    assert_eq!(align_of_val(&ARG_N), align_of_val(&sys::ARG_N));
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_P_eq() {
    assert_eq!(ARG_P, sys::ARG_P);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_P_layout() {
    assert_eq!(size_of_val(&ARG_P), size_of_val(&sys::ARG_P));
    assert_eq!(align_of_val(&ARG_P), align_of_val(&sys::ARG_P));
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_F_eq() {
    assert_eq!(ARG_F, sys::ARG_F);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_F_layout() {
    assert_eq!(size_of_val(&ARG_F), size_of_val(&sys::ARG_F));
    assert_eq!(align_of_val(&ARG_F), align_of_val(&sys::ARG_F));
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_D_eq() {
    assert_eq!(ARG_D, sys::ARG_D);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_D_layout() {
    assert_eq!(size_of_val(&ARG_D), size_of_val(&sys::ARG_D));
    assert_eq!(align_of_val(&ARG_D), align_of_val(&sys::ARG_D));
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_L_eq() {
    assert_eq!(ARG_L, sys::ARG_L);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_L_layout() {
    assert_eq!(size_of_val(&ARG_L), size_of_val(&sys::ARG_L));
    assert_eq!(align_of_val(&ARG_L), align_of_val(&sys::ARG_L));
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_V16_eq() {
    assert_eq!(ARG_V16, sys::ARG_V16);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_V16_layout() {
    assert_eq!(size_of_val(&ARG_V16), size_of_val(&sys::ARG_V16));
    assert_eq!(align_of_val(&ARG_V16), align_of_val(&sys::ARG_V16));
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_V32_eq() {
    assert_eq!(ARG_V32, sys::ARG_V32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_V32_layout() {
    assert_eq!(size_of_val(&ARG_V32), size_of_val(&sys::ARG_V32));
    assert_eq!(align_of_val(&ARG_V32), align_of_val(&sys::ARG_V32));
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_V64_eq() {
    assert_eq!(ARG_V64, sys::ARG_V64);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_V64_layout() {
    assert_eq!(size_of_val(&ARG_V64), size_of_val(&sys::ARG_V64));
    assert_eq!(align_of_val(&ARG_V64), align_of_val(&sys::ARG_V64));
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_NN_eq() {
    assert_eq!(ARG_NN, sys::ARG_NN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_NN_layout() {
    assert_eq!(size_of_val(&ARG_NN), size_of_val(&sys::ARG_NN));
    assert_eq!(align_of_val(&ARG_NN), align_of_val(&sys::ARG_NN));
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_NP_eq() {
    assert_eq!(ARG_NP, sys::ARG_NP);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_NP_layout() {
    assert_eq!(size_of_val(&ARG_NP), size_of_val(&sys::ARG_NP));
    assert_eq!(align_of_val(&ARG_NP), align_of_val(&sys::ARG_NP));
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_PN_eq() {
    assert_eq!(ARG_PN, sys::ARG_PN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_PN_layout() {
    assert_eq!(size_of_val(&ARG_PN), size_of_val(&sys::ARG_PN));
    assert_eq!(align_of_val(&ARG_PN), align_of_val(&sys::ARG_PN));
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_PP_eq() {
    assert_eq!(ARG_PP, sys::ARG_PP);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_PP_layout() {
    assert_eq!(size_of_val(&ARG_PP), size_of_val(&sys::ARG_PP));
    assert_eq!(align_of_val(&ARG_PP), align_of_val(&sys::ARG_PP));
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_NNN_eq() {
    assert_eq!(ARG_NNN, sys::ARG_NNN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_NNN_layout() {
    assert_eq!(size_of_val(&ARG_NNN), size_of_val(&sys::ARG_NNN));
    assert_eq!(align_of_val(&ARG_NNN), align_of_val(&sys::ARG_NNN));
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_NNP_eq() {
    assert_eq!(ARG_NNP, sys::ARG_NNP);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_NNP_layout() {
    assert_eq!(size_of_val(&ARG_NNP), size_of_val(&sys::ARG_NNP));
    assert_eq!(align_of_val(&ARG_NNP), align_of_val(&sys::ARG_NNP));
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_NPN_eq() {
    assert_eq!(ARG_NPN, sys::ARG_NPN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_NPN_layout() {
    assert_eq!(size_of_val(&ARG_NPN), size_of_val(&sys::ARG_NPN));
    assert_eq!(align_of_val(&ARG_NPN), align_of_val(&sys::ARG_NPN));
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_NPP_eq() {
    assert_eq!(ARG_NPP, sys::ARG_NPP);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_NPP_layout() {
    assert_eq!(size_of_val(&ARG_NPP), size_of_val(&sys::ARG_NPP));
    assert_eq!(align_of_val(&ARG_NPP), align_of_val(&sys::ARG_NPP));
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_PNN_eq() {
    assert_eq!(ARG_PNN, sys::ARG_PNN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_PNN_layout() {
    assert_eq!(size_of_val(&ARG_PNN), size_of_val(&sys::ARG_PNN));
    assert_eq!(align_of_val(&ARG_PNN), align_of_val(&sys::ARG_PNN));
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_PNP_eq() {
    assert_eq!(ARG_PNP, sys::ARG_PNP);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_PNP_layout() {
    assert_eq!(size_of_val(&ARG_PNP), size_of_val(&sys::ARG_PNP));
    assert_eq!(align_of_val(&ARG_PNP), align_of_val(&sys::ARG_PNP));
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_PPN_eq() {
    assert_eq!(ARG_PPN, sys::ARG_PPN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_PPN_layout() {
    assert_eq!(size_of_val(&ARG_PPN), size_of_val(&sys::ARG_PPN));
    assert_eq!(align_of_val(&ARG_PPN), align_of_val(&sys::ARG_PPN));
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_PPP_eq() {
    assert_eq!(ARG_PPP, sys::ARG_PPP);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_PPP_layout() {
    assert_eq!(size_of_val(&ARG_PPP), size_of_val(&sys::ARG_PPP));
    assert_eq!(align_of_val(&ARG_PPP), align_of_val(&sys::ARG_PPP));
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_PPPP_eq() {
    assert_eq!(ARG_PPPP, sys::ARG_PPPP);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_PPPP_layout() {
    assert_eq!(size_of_val(&ARG_PPPP), size_of_val(&sys::ARG_PPPP));
    assert_eq!(align_of_val(&ARG_PPPP), align_of_val(&sys::ARG_PPPP));
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_PPPPP_eq() {
    assert_eq!(ARG_PPPPP, sys::ARG_PPPPP);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_PPPPP_layout() {
    assert_eq!(size_of_val(&ARG_PPPPP), size_of_val(&sys::ARG_PPPPP));
    assert_eq!(align_of_val(&ARG_PPPPP), align_of_val(&sys::ARG_PPPPP));
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_PPPPPP_eq() {
    assert_eq!(ARG_PPPPPP, sys::ARG_PPPPPP);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_PPPPPP_layout() {
    assert_eq!(size_of_val(&ARG_PPPPPP), size_of_val(&sys::ARG_PPPPPP));
    assert_eq!(align_of_val(&ARG_PPPPPP), align_of_val(&sys::ARG_PPPPPP));
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_PPPPPPP_eq() {
    assert_eq!(ARG_PPPPPPP, sys::ARG_PPPPPPP);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_PPPPPPP_layout() {
    assert_eq!(size_of_val(&ARG_PPPPPPP), size_of_val(&sys::ARG_PPPPPPP));
    assert_eq!(align_of_val(&ARG_PPPPPPP), align_of_val(&sys::ARG_PPPPPPP));
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_PPPPPPPP_eq() {
    assert_eq!(ARG_PPPPPPPP, sys::ARG_PPPPPPPP);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARG_PPPPPPPP_layout() {
    assert_eq!(size_of_val(&ARG_PPPPPPPP), size_of_val(&sys::ARG_PPPPPPPP));
    assert_eq!(
        align_of_val(&ARG_PPPPPPPP),
        align_of_val(&sys::ARG_PPPPPPPP)
    );
}
