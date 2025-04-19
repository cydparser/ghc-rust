//! These constants are only used when generating a C header.

#[cfg(test)]
mod tests;

// TODO: Set to 1 when defined(UnregisterisedCompiler) || defined(javascript_HOST_ARCH)
pub const MACHREGS_NO_REGS: u32 = 0;

// Target Architecture Constants

#[cfg(target_arch = "aarch64")]
pub const MACHREGS_aarch64: u32 = 1;

#[cfg(target_arch = "arm")]
pub const MACHREGS_arm: u32 = 1;

// TODO: There is not a target_arch for i386
// #[cfg(target_arch = "i386")]
// pub const MACHREGS_i386: u32 = 1;

#[cfg(target_arch = "loongarch64")]
pub const MACHREGS_loongarch64: u32 = 1;

#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
pub const MACHREGS_powerpc: u32 = 1;

#[cfg(target_arch = "riscv64")]
pub const MACHREGS_riscv64: u32 = 1;

#[cfg(target_arch = "s390x")]
pub const MACHREGS_s390x: u32 = 1;

#[cfg(target_arch = "wasm32")]
pub const MACHREGS_wasm32: u32 = 1;

#[cfg(target_arch = "x86_64")]
pub const MACHREGS_x86_64: u32 = 1;

// Target OS Constants

#[cfg(target_os = "macos")]
pub const MACHREGS_darwin: u32 = 1;
