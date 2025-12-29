use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(ghc_lib)]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExecPage {
    pub contents: c_char,
}

#[cfg(test)]
impl Arbitrary for ExecPage {
    fn arbitrary(g: &mut Gen) -> Self {
        ExecPage {
            contents: Arbitrary::arbitrary(g),
        }
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn allocateExecPage() -> *mut ExecPage {
    sys! {
        allocateExecPage().cast()
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn freezeExecPage(page: *mut ExecPage) {
    sys! {
        freezeExecPage(page as * mut sys::ExecPage)
    }
}
