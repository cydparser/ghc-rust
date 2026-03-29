use crate::ffi::rts::_assertFail;
use crate::ffi::rts::constants::CLOSURE_DESC_BUFFER_SIZE;
use crate::ffi::rts::ipe::{
    InfoProv, InfoProv_, InfoProvEnt, InfoProvEnt_, IpeBufferEntry, IpeBufferListNode,
    IpeBufferListNode_, StringIdx,
};
use crate::ffi::rts::messages::barf;
use crate::ffi::rts::types::StgInfoTable;
use crate::ffi::rts::types::StgInfoTable;
use crate::ffi::stg::smp::{cas_ptr, xchg_ptr};
use crate::ffi::stg::types::StgWord;
use crate::ffi::stg::types::StgWord;
use crate::hash::{HashTable, allocHashTable, insertHashTable, lookupHashTable, mapHashTable};
use crate::prelude::*;
use crate::rts_utils::stgMallocBytes;
use crate::trace::traceIPE;

#[cfg(test)]
mod tests;

#[ffi(ghc_lib)]
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

#[ffi(compiler, ghc_lib)]
pub type InfoProv = InfoProv_;

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct InfoProvEnt_ {
    pub(crate) info: *const StgInfoTable,
    pub(crate) prov: InfoProv,
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

#[ffi(compiler, libraries)]
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

/// cbindgen:no-export
struct IpeMapEntry {
    node: *mut IpeBufferListNode,
    idx: u32,
}

static mut ipeMap: *mut HashTable = null_mut::<HashTable>();

static mut ipeBufferList: *mut IpeBufferListNode = null_mut::<IpeBufferListNode>();

unsafe fn initIpe() {}

unsafe fn exitIpe() {}

unsafe fn ipeBufferEntryToIpe(mut node: *const IpeBufferListNode, mut idx: u32) -> InfoProvEnt {
    if ((idx as StgWord) < (*node).count) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/IPE.c".as_ptr(), 100);
    }

    if ((*node).compressed == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/IPE.c".as_ptr(), 101);
    }

    let mut strings = (*node).string_table;
    let mut ent: *const IpeBufferEntry =
        (*node).entries.offset(idx as isize) as *mut IpeBufferEntry;

    return InfoProvEnt_ {
        info: *(*node).tables.offset(idx as isize),
        prov: InfoProv_ {
            table_name: strings.offset((*ent).table_name as isize) as *const c_char,
            closure_desc: (*ent).closure_desc,
            ty_desc: strings.offset((*ent).ty_desc as isize) as *const c_char,
            label: strings.offset((*ent).label as isize) as *const c_char,
            unit_id: strings.offset((*node).unit_id as isize) as *const c_char,
            module: strings.offset((*node).module_name as isize) as *const c_char,
            src_file: strings.offset((*ent).src_file as isize) as *const c_char,
            src_span: strings.offset((*ent).src_span as isize) as *const c_char,
        },
    };
}

unsafe fn traceIPEFromHashTable(mut data: *mut c_void, mut key: StgWord, mut value: *const c_void) {
    let mut map_ent = value as *const IpeMapEntry;
    let ipe = ipeBufferEntryToIpe((*map_ent).node, (*map_ent).idx) as InfoProvEnt;
    traceIPE(&raw const ipe);
}

unsafe fn dumpIPEToEventLog() {
    let mut node = ipeBufferList;

    while !node.is_null() {
        decompressIPEBufferListNodeIfCompressed(node);

        let mut i: u32 = 0;

        while (i as StgWord) < (*node).count {
            let ent = ipeBufferEntryToIpe(node, i) as InfoProvEnt;
            traceIPE(&raw const ent);
            i = i.wrapping_add(1);
        }

        node = (*node).next as *mut IpeBufferListNode;
    }

    if !ipeMap.is_null() {
        mapHashTable(
            ipeMap,
            NULL,
            Some(
                traceIPEFromHashTable
                    as unsafe extern "C" fn(*mut c_void, StgWord, *const c_void) -> (),
            ),
        );
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn registerInfoProvList(mut node: *mut IpeBufferListNode) {
    loop {
        let mut old = ipeBufferList;
        (*node).next = old as *mut IpeBufferListNode_;

        if cas_ptr(
            &raw mut ipeBufferList as *mut *mut c_void,
            old as *mut c_void,
            node as *mut c_void,
        ) == old as *mut c_void
        {
            return;
        }
    }
}

unsafe fn formatClosureDescIpe(mut ipe_buf: *const InfoProvEnt, mut str_buf: *mut c_char) {
    snprintf(
        str_buf,
        CLOSURE_DESC_BUFFER_SIZE as usize,
        c"%u".as_ptr(),
        (*ipe_buf).prov.closure_desc,
    );
}

#[ffi(compiler, ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn lookupIPE(
    mut info: *const StgInfoTable,
    mut out: *mut InfoProvEnt,
) -> bool {
    updateIpeMap();

    let mut map_ent = lookupHashTable(ipeMap, info as StgWord) as *mut IpeMapEntry;

    if !map_ent.is_null() {
        *out = ipeBufferEntryToIpe((*map_ent).node, (*map_ent).idx);

        return true;
    } else {
        return false;
    };
}

unsafe fn updateIpeMap() {
    let mut pending =
        xchg_ptr(&raw mut ipeBufferList as *mut *mut c_void, NULL) as *mut IpeBufferListNode;

    if !ipeMap.is_null() && pending.is_null() {
        return;
    }

    if ipeMap.is_null() {
        ipeMap = allocHashTable();
    }

    while !pending.is_null() {
        let mut node = pending;
        decompressIPEBufferListNodeIfCompressed(node);

        let mut map_ents = stgMallocBytes(
            (*node)
                .count
                .wrapping_mul(size_of::<IpeMapEntry>() as StgWord) as usize,
            c"updateIpeMap: ip_ents".as_ptr(),
        ) as *mut IpeMapEntry;

        let mut i: u32 = 0;

        while (i as StgWord) < (*node).count {
            let mut tbl = *(*node).tables.offset(i as isize);
            let ref mut fresh5 = (*map_ents.offset(i as isize)).node;
            *fresh5 = node;
            (*map_ents.offset(i as isize)).idx = i;

            insertHashTable(
                ipeMap,
                tbl as StgWord,
                map_ents.offset(i as isize) as *mut IpeMapEntry as *const c_void,
            );

            i = i.wrapping_add(1);
        }

        pending = (*node).next as *mut IpeBufferListNode;
    }
}

unsafe fn decompressIPEBufferListNodeIfCompressed(mut node: *mut IpeBufferListNode) {
    if (*node).compressed == 1 {
        (*node).compressed = 0;

        barf(
            c"An IPE buffer list node has been compressed, but the decompression library (zstd) is not available."
                .as_ptr(),
        );
    }
}
