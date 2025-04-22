use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use libc::{clockid_t, pid_t, pthread_cond_t, pthread_key_t, pthread_mutex_t, pthread_t};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use std::mem::transmute;
#[cfg(feature = "tracing")]
use tracing::instrument;
#[cfg(test)]
mod tests;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct InfoProv_ {
    pub table_name: *const ::core::ffi::c_char,
    pub closure_desc: u32,
    pub ty_desc: *const ::core::ffi::c_char,
    pub label: *const ::core::ffi::c_char,
    pub unit_id: *const ::core::ffi::c_char,
    pub module: *const ::core::ffi::c_char,
    pub src_file: *const ::core::ffi::c_char,
    pub src_span: *const ::core::ffi::c_char,
}

#[cfg(feature = "sys")]
impl From<InfoProv_> for sys::InfoProv_ {
    fn from(x: InfoProv_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for InfoProv_ {
    fn arbitrary(g: &mut Gen) -> Self {
        InfoProv_ {
            table_name: Arbitrary::arbitrary(g),
            closure_desc: Arbitrary::arbitrary(g),
            ty_desc: Arbitrary::arbitrary(g),
            label: Arbitrary::arbitrary(g),
            unit_id: Arbitrary::arbitrary(g),
            module: Arbitrary::arbitrary(g),
            src_file: Arbitrary::arbitrary(g),
            src_span: Arbitrary::arbitrary(g),
        }
    }
}

pub type InfoProv = InfoProv_;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct InfoProvEnt_ {
    pub info: *const StgInfoTable,
    pub prov: InfoProv,
}

#[cfg(feature = "sys")]
impl From<InfoProvEnt_> for sys::InfoProvEnt_ {
    fn from(x: InfoProvEnt_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for InfoProvEnt_ {
    fn arbitrary(g: &mut Gen) -> Self {
        InfoProvEnt_ {
            info: Arbitrary::arbitrary(g),
            prov: Arbitrary::arbitrary(g),
        }
    }
}

pub type InfoProvEnt = InfoProvEnt_;

pub(crate) type StringIdx = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
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

#[repr(C)]
pub(crate) struct IpeBufferListNode_ {
    pub next: *mut IpeBufferListNode_,
    pub compressed: StgWord,
    pub count: StgWord,
    pub tables: *mut *const StgInfoTable,
    pub entries: *mut IpeBufferEntry,
    pub entries_size: StgWord,
    pub string_table: *const ::core::ffi::c_char,
    pub string_table_size: StgWord,
    pub unit_id: StringIdx,
    pub module_name: StringIdx,
}

#[cfg(feature = "sys")]
impl From<IpeBufferListNode_> for sys::IpeBufferListNode_ {
    fn from(x: IpeBufferListNode_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for IpeBufferListNode_ {
    fn arbitrary(g: &mut Gen) -> Self {
        IpeBufferListNode_ {
            next: Arbitrary::arbitrary(g),
            compressed: Arbitrary::arbitrary(g),
            count: Arbitrary::arbitrary(g),
            tables: Arbitrary::arbitrary(g),
            entries: Arbitrary::arbitrary(g),
            entries_size: Arbitrary::arbitrary(g),
            string_table: Arbitrary::arbitrary(g),
            string_table_size: Arbitrary::arbitrary(g),
            unit_id: Arbitrary::arbitrary(g),
            module_name: Arbitrary::arbitrary(g),
        }
    }
}

pub type IpeBufferListNode = IpeBufferListNode_;

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn registerInfoProvList(node: *mut IpeBufferListNode) {
    unsafe { sys::registerInfoProvList(node) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn formatClosureDescIpe(
    ipe_buf: *const InfoProvEnt,
    str_buf: *mut ::core::ffi::c_char,
) {
    unsafe { sys::formatClosureDescIpe(ipe_buf, str_buf) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn lookupIPE(info: *const StgInfoTable, out: *mut InfoProvEnt) -> bool {
    unsafe { transmute(sys::lookupIPE(info, out)) }
}
