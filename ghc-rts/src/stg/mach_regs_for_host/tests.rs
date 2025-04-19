#[cfg(feature = "sys")]
use ghc_rts_sys as sys;

#[cfg(feature = "sys")]
#[test]
fn test_eq_MACHREGS_NO_REGS() {
    assert_eq!(sys::MACHREGS_NO_REGS, super::MACHREGS_NO_REGS.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_MACHREGS_arch() {
    #[cfg(target_arch = "aarch64")]
    assert_eq!(sys::MACHREGS_aarch64, super::MACHREGS_aarch64.into());

    #[cfg(target_arch = "arm")]
    assert_eq!(sys::MACHREGS_arm, super::MACHREGS_arm.into());

    #[cfg(target_arch = "loongarch64")]
    assert_eq!(
        sys::MACHREGS_loongarch64,
        super::MACHREGS_loongarch64.into()
    );

    #[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
    assert_eq!(sys::MACHREGS_powerpc, super::MACHREGS_powerpc.into());

    #[cfg(target_arch = "riscv64")]
    assert_eq!(sys::MACHREGS_riscv64, super::MACHREGS_riscv64.into());

    #[cfg(target_arch = "s390x")]
    assert_eq!(sys::MACHREGS_s390x, super::MACHREGS_s390x.into());

    #[cfg(target_arch = "wasm32")]
    assert_eq!(sys::MACHREGS_wasm32, super::MACHREGS_wasm32.into());

    #[cfg(target_arch = "x86_64")]
    assert_eq!(sys::MACHREGS_x86_64, super::MACHREGS_x86_64.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_MACHREGS_os() {
    #[cfg(target_os = "macos")]
    assert_eq!(sys::MACHREGS_darwin, super::MACHREGS_darwin.into());
}
