use crate::prelude::*;

#[cfg(test)]
mod tests;

/// - GHC_PLACES: {libraries}
#[repr(C)]
#[derive(Debug, PartialEq)]
#[cfg_attr(test, derive(Clone))]
pub struct ExecPage {
    pub contents: c_char,
}

#[cfg(feature = "sys")]
impl From<ExecPage> for sys::ExecPage {
    fn from(x: ExecPage) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for ExecPage {
    fn arbitrary(g: &mut Gen) -> Self {
        ExecPage {
            contents: Arbitrary::arbitrary(g),
        }
    }
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn allocateExecPage() -> *mut ExecPage {
    #[cfg(feature = "sys")]
    unsafe {
        sys::allocateExecPage() as *mut ExecPage
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("allocateExecPage")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn freezeExecPage(page: *mut ExecPage) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::freezeExecPage(page as *mut sys::ExecPage)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("freezeExecPage")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn freeExecPage(page: *mut ExecPage) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::freeExecPage(page as *mut sys::ExecPage)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("freeExecPage")
}
