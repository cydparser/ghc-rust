use crate::ffi::rts::storage::info_tables::StgInfoTable;
use crate::ffi::stg::types::StgWord;
use crate::prelude::*;

#[cfg(test)]
mod tests;

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct InfoProv_ {
    table_name: *const c_char,
    closure_desc: u32,
    ty_desc: *const c_char,
    label: *const c_char,
    unit_id: *const c_char,
    module: *const c_char,
    src_file: *const c_char,
    src_span: *const c_char,
}

#[ffi(compiler, ghc_lib)]
pub type InfoProv = InfoProv_;

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct InfoProvEnt_ {
    info: *const StgInfoTable,
    prov: InfoProv,
}

#[ffi(compiler, ghc_lib, testsuite)]
pub type InfoProvEnt = InfoProvEnt_;

pub(crate) type StringIdx = u32;

#[ffi(testsuite)]
#[repr(C)]
#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub struct IpeBufferEntry {
    pub table_name: StringIdx,
    pub closure_desc: u32,
    pub ty_desc: StringIdx,
    pub label: StringIdx,
    pub src_file: StringIdx,
    pub src_span: StringIdx,
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

/// cbindgen:no-export
#[repr(C)]
pub struct IpeBufferListNode_ {
    next: *mut IpeBufferListNode_,
    compressed: StgWord,
    count: StgWord,
    tables: *mut *const StgInfoTable,
    entries: *mut IpeBufferEntry,
    entries_size: StgWord,
    string_table: *const c_char,
    string_table_size: StgWord,
    unit_id: StringIdx,
    module_name: StringIdx,
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
