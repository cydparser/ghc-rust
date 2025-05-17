use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(test)]
use crate::utils::test::{Arbitrary, Gen, HasReferences};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use libc::{clockid_t, pid_t, pthread_cond_t, pthread_key_t, pthread_mutex_t, pthread_t};
use std::ffi::{c_char, c_int, c_uint, c_void};
use std::mem::transmute;
use std::ptr::{null, null_mut};
use std::slice;
#[cfg(feature = "tracing")]
use tracing::instrument;
#[cfg(test)]
mod tests;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
pub(crate) struct InfoProv_ {
    pub table_name: *const c_char,
    pub closure_desc: u32,
    pub ty_desc: *const c_char,
    pub label: *const c_char,
    pub unit_id: *const c_char,
    pub module: *const c_char,
    pub src_file: *const c_char,
    pub src_span: *const c_char,
}

#[cfg(feature = "sys")]
impl From<InfoProv_> for sys::InfoProv_ {
    fn from(x: InfoProv_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct InfoProv_Owned {
    pub closure_desc: u32,
}

#[cfg(test)]
impl Arbitrary for InfoProv_Owned {
    fn arbitrary(g: &mut Gen) -> Self {
        InfoProv_Owned {
            closure_desc: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct InfoProv_Pointees {
    pub table_name: c_char,
    pub ty_desc: c_char,
    pub label: c_char,
    pub unit_id: c_char,
    pub module: c_char,
    pub src_file: c_char,
    pub src_span: c_char,
}

#[cfg(test)]
impl Arbitrary for InfoProv_Pointees {
    fn arbitrary(g: &mut Gen) -> Self {
        InfoProv_Pointees {
            table_name: Arbitrary::arbitrary(g),
            ty_desc: Arbitrary::arbitrary(g),
            label: Arbitrary::arbitrary(g),
            unit_id: Arbitrary::arbitrary(g),
            module: Arbitrary::arbitrary(g),
            src_file: Arbitrary::arbitrary(g),
            src_span: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for InfoProv_ {
    type Owned = InfoProv_Owned;
    type Pointees = InfoProv_Pointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            closure_desc: owned.closure_desc,
            table_name: unsafe { &raw mut (*pointees).table_name },
            ty_desc: unsafe { &raw mut (*pointees).ty_desc },
            label: unsafe { &raw mut (*pointees).label },
            unit_id: unsafe { &raw mut (*pointees).unit_id },
            module: unsafe { &raw mut (*pointees).module },
            src_file: unsafe { &raw mut (*pointees).src_file },
            src_span: unsafe { &raw mut (*pointees).src_span },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            closure_desc: self.closure_desc,
        }
    }
}

pub type InfoProv = InfoProv_;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
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
#[derive(Clone)]
struct InfoProvEnt_Owned {
    pub prov: InfoProv,
}

#[cfg(test)]
impl Arbitrary for InfoProvEnt_Owned {
    fn arbitrary(g: &mut Gen) -> Self {
        InfoProvEnt_Owned {
            prov: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct InfoProvEnt_Pointees {
    pub info: StgInfoTable,
}

#[cfg(test)]
impl Arbitrary for InfoProvEnt_Pointees {
    fn arbitrary(g: &mut Gen) -> Self {
        InfoProvEnt_Pointees {
            info: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for InfoProvEnt_ {
    type Owned = InfoProvEnt_Owned;
    type Pointees = InfoProvEnt_Pointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            prov: owned.prov.clone(),
            info: unsafe { &raw mut (*pointees).info },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            prov: self.prov.clone(),
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
///cbindgen:no-export
pub(crate) struct IpeBufferListNode_ {
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

#[cfg(feature = "sys")]
impl From<IpeBufferListNode_> for sys::IpeBufferListNode_ {
    fn from(x: IpeBufferListNode_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct IpeBufferListNode_Owned {
    pub compressed: StgWord,
    pub count: StgWord,
    pub entries_size: StgWord,
    pub string_table_size: StgWord,
    pub unit_id: StringIdx,
    pub module_name: StringIdx,
}

#[cfg(test)]
impl Arbitrary for IpeBufferListNode_Owned {
    fn arbitrary(g: &mut Gen) -> Self {
        IpeBufferListNode_Owned {
            compressed: Arbitrary::arbitrary(g),
            count: Arbitrary::arbitrary(g),
            entries_size: Arbitrary::arbitrary(g),
            string_table_size: Arbitrary::arbitrary(g),
            unit_id: Arbitrary::arbitrary(g),
            module_name: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct IpeBufferListNode_Pointees {
    pub next: IpeBufferListNode_,
    pub tables: *const StgInfoTable,
    pub entries: IpeBufferEntry,
    pub string_table: c_char,
}

#[cfg(test)]
impl Arbitrary for IpeBufferListNode_Pointees {
    fn arbitrary(g: &mut Gen) -> Self {
        IpeBufferListNode_Pointees {
            next: Arbitrary::arbitrary(g),
            tables: Arbitrary::arbitrary(g),
            entries: Arbitrary::arbitrary(g),
            string_table: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for IpeBufferListNode_ {
    type Owned = IpeBufferListNode_Owned;
    type Pointees = IpeBufferListNode_Pointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            compressed: owned.compressed,
            count: owned.count,
            entries_size: owned.entries_size,
            string_table_size: owned.string_table_size,
            unit_id: owned.unit_id,
            module_name: owned.module_name,
            next: unsafe { &raw mut (*pointees).next },
            tables: unsafe { &raw mut (*pointees).tables },
            entries: unsafe { &raw mut (*pointees).entries },
            string_table: unsafe { &raw mut (*pointees).string_table },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            compressed: self.compressed,
            count: self.count,
            entries_size: self.entries_size,
            string_table_size: self.string_table_size,
            unit_id: self.unit_id,
            module_name: self.module_name,
        }
    }
}

pub type IpeBufferListNode = IpeBufferListNode_;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_registerInfoProvList"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn registerInfoProvList(node: *mut IpeBufferListNode) {
    unsafe { sys::registerInfoProvList(node as *mut sys::IpeBufferListNode) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_formatClosureDescIpe"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn formatClosureDescIpe(ipe_buf: *const InfoProvEnt, str_buf: *mut c_char) {
    unsafe { sys::formatClosureDescIpe(ipe_buf as *const sys::InfoProvEnt, str_buf) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_lookupIPE"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn lookupIPE(info: *const StgInfoTable, out: *mut InfoProvEnt) -> bool {
    unsafe {
        transmute(sys::lookupIPE(
            info as *const sys::StgInfoTable,
            out as *mut sys::InfoProvEnt,
        ))
    }
}
