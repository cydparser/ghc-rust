use crate::prelude::*;
use crate::rts::storage::info_tables::StgInfoTable;
use crate::stg::types::StgWord;

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

#[cfg(feature = "sys")]
impl From<InfoProv_> for sys::InfoProv_ {
    fn from(x: InfoProv_) -> Self {
        unsafe { transmute(x) }
    }
}

/// - GHC_PLACES: {libraries}
pub type InfoProv = InfoProv_;

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct InfoProvEnt_ {
    info: *const StgInfoTable,
    prov: InfoProv,
}

#[cfg(feature = "sys")]
impl From<InfoProvEnt_> for sys::InfoProvEnt_ {
    fn from(x: InfoProvEnt_) -> Self {
        unsafe { transmute(x) }
    }
}

/// - GHC_PLACES: {libraries, testsuite}
pub type InfoProvEnt = InfoProvEnt_;

pub(crate) type StringIdx = u32;

/// - GHC_PLACES: {testsuite}
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

#[cfg(feature = "sys")]
impl From<IpeBufferEntry> for sys::IpeBufferEntry {
    fn from(x: IpeBufferEntry) -> Self {
        unsafe { transmute(x) }
    }
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

#[cfg(feature = "sys")]
impl From<IpeBufferListNode_> for sys::IpeBufferListNode_ {
    fn from(x: IpeBufferListNode_) -> Self {
        unsafe { transmute(x) }
    }
}

/// - GHC_PLACES: {testsuite}
pub type IpeBufferListNode = IpeBufferListNode_;

#[cfg(feature = "ghc_testsuite")]
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_registerInfoProvList"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn registerInfoProvList(node: *mut IpeBufferListNode) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::registerInfoProvList(node as *mut sys::IpeBufferListNode)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("registerInfoProvList")
}

#[cfg(feature = "ghc_testsuite")]
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_formatClosureDescIpe"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn formatClosureDescIpe(ipe_buf: *const InfoProvEnt, str_buf: *mut c_char) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::formatClosureDescIpe(ipe_buf as *const sys::InfoProvEnt, str_buf)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("formatClosureDescIpe")
}

/// - GHC_PLACES: {libraries, testsuite}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_lookupIPE"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn lookupIPE(info: *const StgInfoTable, out: *mut InfoProvEnt) -> bool {
    #[cfg(feature = "sys")]
    unsafe {
        sys::lookupIPE(
            info as *const sys::StgInfoTable,
            out as *mut sys::InfoProvEnt,
        )
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("lookupIPE")
}
