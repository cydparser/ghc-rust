use super::*;
use crate::prelude::*;

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MACHREGS_NO_REGS() {
    assert_eq!(sys::MACHREGS_NO_REGS, MACHREGS_NO_REGS);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_MACHREGS_arch() {
    #[cfg(target_arch = "aarch64")]
    assert_eq!(sys::MACHREGS_aarch64, MACHREGS_aarch64);

    #[cfg(target_arch = "arm")]
    assert_eq!(sys::MACHREGS_arm, MACHREGS_arm);

    #[cfg(target_arch = "loongarch64")]
    assert_eq!(
        sys::MACHREGS_loongarch64,
        MACHREGS_loongarch64.
    );

    #[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
    assert_eq!(sys::MACHREGS_powerpc, MACHREGS_powerpc);

    #[cfg(target_arch = "riscv64")]
    assert_eq!(sys::MACHREGS_riscv64, MACHREGS_riscv64);

    #[cfg(target_arch = "s390x")]
    assert_eq!(sys::MACHREGS_s390x, MACHREGS_s390x);

    #[cfg(target_arch = "wasm32")]
    assert_eq!(sys::MACHREGS_wasm32, MACHREGS_wasm32);

    #[cfg(target_arch = "x86_64")]
    assert_eq!(sys::MACHREGS_x86_64, MACHREGS_x86_64);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_MACHREGS_os() {
    #[cfg(target_os = "macos")]
    assert_eq!(sys::MACHREGS_darwin, MACHREGS_darwin);
}
