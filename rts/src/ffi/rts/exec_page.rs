use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(libraries)]
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

#[ffi(libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn allocateExecPage() -> *mut ExecPage {
    sys! {
        allocateExecPage().cast()
    }
}

#[ffi(libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn freezeExecPage(page: *mut ExecPage) {
    sys! {
        freezeExecPage(page as * mut sys::ExecPage)
    }
}
