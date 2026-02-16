use crate::ffi::rts::storage::info_tables::StgInfoTable;
use crate::ffi::stg::types::StgWord;
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(testsuite)]
#[repr(C)]
#[derive(Debug)]
pub struct InfoProv_ {
    pub(crate) table_name: *const c_char,
    pub(crate) closure_desc: u32,
    pub(crate) ty_desc: *const c_char,
    pub(crate) label: *const c_char,
    pub(crate) unit_id: *const c_char,
    pub(crate) module: *const c_char,
    pub(crate) src_file: *const c_char,
    pub(crate) src_span: *const c_char,
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

#[ffi(compiler, ghc_lib)]
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

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn registerInfoProvList(node: *mut IpeBufferListNode) {
    sys! {
        registerInfoProvList(node as * mut sys::IpeBufferListNode)
    }
}

#[ffi(compiler, ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn lookupIPE(info: *const StgInfoTable, out: *mut InfoProvEnt) -> bool {
    sys! {
        lookupIPE(info as * const sys::StgInfoTable, out as * mut sys::InfoProvEnt)
    }
}
