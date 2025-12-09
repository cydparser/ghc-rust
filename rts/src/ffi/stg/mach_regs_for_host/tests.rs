use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_MACHREGS_NO_REGS_eq() {
    assert_eq!(MACHREGS_NO_REGS, sys::MACHREGS_NO_REGS);
}

#[cfg(feature = "sys")]
#[test]
fn sys_MACHREGS_NO_REGS_layout() {
    assert_eq!(
        size_of_val(&MACHREGS_NO_REGS),
        size_of_val(&sys::MACHREGS_NO_REGS)
    );
    assert_eq!(
        align_of_val(&MACHREGS_NO_REGS),
        align_of_val(&sys::MACHREGS_NO_REGS)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_MACHREGS_aarch64_eq() {
    assert_eq!(MACHREGS_aarch64, sys::MACHREGS_aarch64);
}

#[cfg(feature = "sys")]
#[test]
fn sys_MACHREGS_aarch64_layout() {
    assert_eq!(
        size_of_val(&MACHREGS_aarch64),
        size_of_val(&sys::MACHREGS_aarch64)
    );
    assert_eq!(
        align_of_val(&MACHREGS_aarch64),
        align_of_val(&sys::MACHREGS_aarch64)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_MACHREGS_darwin_eq() {
    assert_eq!(MACHREGS_darwin, sys::MACHREGS_darwin);
}

#[cfg(feature = "sys")]
#[test]
fn sys_MACHREGS_darwin_layout() {
    assert_eq!(
        size_of_val(&MACHREGS_darwin),
        size_of_val(&sys::MACHREGS_darwin)
    );
    assert_eq!(
        align_of_val(&MACHREGS_darwin),
        align_of_val(&sys::MACHREGS_darwin)
    );
}
