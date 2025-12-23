use crate::ffi::rts::storage::info_tables::StgInfoTable;
use crate::ffi::stg::types::StgWord;
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(compiler, testsuite)]
#[repr(C)]
#[derive(Debug)]
pub struct InfoProv_ {
    pub table_name: *const c_char,
    pub closure_desc: u32,
    pub ty_desc: *const c_char,
    pub label: *const c_char,
    pub unit_id: *const c_char,
    pub module: *const c_char,
    pub src_file: *const c_char,
    pub src_span: *const c_char,
}

#[ffi(compiler, ghc_lib, testsuite)]
pub type InfoProv = InfoProv_;

#[ffi(testsuite)]
#[repr(C)]
#[derive(Debug)]
pub struct InfoProvEnt_ {
    pub(crate) info: *const StgInfoTable,
    pub(crate) prov: InfoProv,
}

#[ffi(compiler, ghc_lib, testsuite)]
pub type InfoProvEnt = InfoProvEnt_;

#[ffi(compiler, libraries, testsuite)]
pub type StringIdx = u32;

#[ffi(testsuite)]
#[repr(C)]
#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub struct IpeBufferEntry {
    pub(crate) table_name: StringIdx,
    pub(crate) closure_desc: u32,
    pub(crate) ty_desc: StringIdx,
    pub(crate) label: StringIdx,
    pub(crate) src_file: StringIdx,
    pub(crate) src_span: StringIdx,
}

#[cfg(test)]
impl Arbitrary for IpeBufferEntry {
    fn arbitrary(g: &mut Gen) -> Self {
        IpeBufferEntry {
            table_name: Arbitrary::arbitrary(g),
            closure_desc: Arbitrary::arbitrary(g),
            ty_desc: Arbitrary::arbitrary(g),
            label: Arbitrary::arbitrary(g),
            src_file: Arbitrary::arbitrary(g),
            src_span: Arbitrary::arbitrary(g),
        }
    }
}

#[ffi(compiler, libraries, testsuite)]
#[repr(C)]
pub struct IpeBufferListNode_ {
    pub next: *mut IpeBufferListNode_,
    pub compressed: StgWord,
    pub count: StgWord,
    pub tables: *mut *const StgInfoTable,
    pub entries: *mut IpeBufferEntry,
    pub entries_size: StgWord,
    pub string_table: *const c_char,
    pub string_table_size: StgWord,
    pub unit_id: StringIdx,
    pub module_name: StringIdx,
}

#[ffi(compiler, testsuite)]
pub type IpeBufferListNode = IpeBufferListNode_;

#[ffi(compiler, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn registerInfoProvList(node: *mut IpeBufferListNode) {
    sys! {
        registerInfoProvList(node as * mut sys::IpeBufferListNode)
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn formatClosureDescIpe(ipe_buf: *const InfoProvEnt, str_buf: *mut c_char) {
    sys! {
        formatClosureDescIpe(ipe_buf as * const sys::InfoProvEnt, str_buf)
    }
}

#[ffi(compiler, ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn lookupIPE(info: *const StgInfoTable, out: *mut InfoProvEnt) -> bool {
    sys! {
        lookupIPE(info as * const sys::StgInfoTable, out as * mut sys::InfoProvEnt)
    }
}
