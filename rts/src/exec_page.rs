use crate::ffi::rts::exec_page::ExecPage;
use crate::ffi::rts::storage::gc::{AdjustorExecutable, flushExec};
use crate::ffi::stg::W_;
use crate::linker::m_map::{MEM_READ_EXECUTE, mmapAnon, mprotectForLinker, munmapForLinker};
use crate::prelude::*;
use crate::sm::os_mem::getPageSize;

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
    let mut page = mmapAnon(getPageSize()) as *mut ExecPage;

    return page;
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn freezeExecPage(mut page: *mut ExecPage) {
    mprotectForLinker(page as *mut c_void, getPageSize(), MEM_READ_EXECUTE);
    flushExec(getPageSize() as W_, page as AdjustorExecutable);
}

unsafe fn freeExecPage(mut page: *mut ExecPage) {
    munmapForLinker(page as *mut c_void, getPageSize(), c"freeExecPage".as_ptr());
}
