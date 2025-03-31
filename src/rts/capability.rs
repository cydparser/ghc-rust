#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use std::mem::transmute;

#[cfg(test)]
mod tests;

pub type Capability = Capability_;

// TODO: replace with C implementation in rts/Capability.h
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct Capability_ {
    _unused: [u8; 0],
}

#[cfg(feature = "sys")]
impl From<Capability_> for sys::Capability_ {
    fn from(x: Capability_) -> Self {
        unsafe { transmute(x) }
    }
}
