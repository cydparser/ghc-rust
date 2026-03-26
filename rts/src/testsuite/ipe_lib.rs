use crate::ffi::hs_ffi::HsInt;
use crate::ffi::rts::ipe::{IpeBufferEntry, IpeBufferListNode, StringIdx};
use crate::ffi::rts::storage::closure_macros::get_itbl;
use crate::ffi::rts::types::{StgClosure, StgInfoTable};
use crate::ffi::rts_api::{Capability, rts_mkInt};
use crate::ffi::stg::types::StgWord;
use crate::prelude::*;

unsafe fn init_string_table(mut st: *mut StringTable) {
    (*st).size = 128 as size_t;
    (*st).n = 0 as size_t;
    (*st).buffer = malloc((*st).size) as *mut c_char;
}

unsafe fn add_string(mut st: *mut StringTable, mut s: *const c_char) -> uint32_t {
    let len = strlen(s) as size_t;
    let n: uint32_t = (*st).n as uint32_t;

    if (*st).n.wrapping_add(len).wrapping_add(1 as size_t) > (*st).size {
        let new_size: size_t = (2 as size_t).wrapping_mul((*st).size).wrapping_add(len);
        (*st).buffer = realloc((*st).buffer as *mut c_void, new_size) as *mut c_char;
        (*st).size = new_size;
    }

    memcpy(
        (*st).buffer.offset((*st).n as isize) as *mut c_char as *mut c_void,
        s as *const c_void,
        len,
    );

    (*st).n = (*st).n.wrapping_add(len);
    *(*st).buffer.offset((*st).n as isize) = '\0' as i32 as c_char;
    (*st).n = (*st).n.wrapping_add(1 as size_t);

    return n;
}

unsafe fn makeAnyProvEntry(
    mut cap: *mut Capability,
    mut st: *mut StringTable,
    mut i: c_int,
) -> IpeBufferEntry {
    let mut provEnt = IpeBufferEntry {
        table_name: 0,
        closure_desc: 0,
        ty_desc: 0,
        label: 0,
        src_file: 0,
        src_span: 0,
    };

    let mut tableNameLength = strlen(b"table_name_\0" as *const u8 as *const c_char)
        .wrapping_add(3 as size_t)
        .wrapping_add(1 as size_t) as c_uint;

    let mut tableName =
        malloc((size_of::<c_char>() as size_t).wrapping_mul(tableNameLength as size_t))
            as *mut c_char;

    snprintf(
        tableName,
        tableNameLength as size_t,
        b"table_name_%03i\0" as *const u8 as *const c_char,
        i,
    );

    provEnt.table_name = add_string(st, tableName) as StringIdx;
    provEnt.closure_desc = i as uint32_t;

    let mut tyDescLength = strlen(b"ty_desc_\0" as *const u8 as *const c_char)
        .wrapping_add(3 as size_t)
        .wrapping_add(1 as size_t) as c_uint;

    let mut tyDesc =
        malloc((size_of::<c_char>() as size_t).wrapping_mul(tyDescLength as size_t)) as *mut c_char;

    snprintf(
        tyDesc,
        tyDescLength as size_t,
        b"ty_desc_%03i\0" as *const u8 as *const c_char,
        i,
    );

    provEnt.ty_desc = add_string(st, tyDesc) as StringIdx;

    let mut labelLength = strlen(b"label_\0" as *const u8 as *const c_char)
        .wrapping_add(3 as size_t)
        .wrapping_add(1 as size_t) as c_uint;

    let mut label =
        malloc((size_of::<c_char>() as size_t).wrapping_mul(labelLength as size_t)) as *mut c_char;

    snprintf(
        label,
        labelLength as size_t,
        b"label_%03i\0" as *const u8 as *const c_char,
        i,
    );

    provEnt.label = add_string(st, label) as StringIdx;

    let mut srcFileLength = strlen(b"src_file_\0" as *const u8 as *const c_char)
        .wrapping_add(3 as size_t)
        .wrapping_add(1 as size_t) as c_uint;

    let mut srcFile = malloc((size_of::<c_char>() as size_t).wrapping_mul(srcFileLength as size_t))
        as *mut c_char;

    snprintf(
        srcFile,
        srcFileLength as size_t,
        b"src_file_%03i\0" as *const u8 as *const c_char,
        i,
    );

    provEnt.src_file = add_string(st, srcFile) as StringIdx;

    let mut srcSpanLength = strlen(b"src_span_\0" as *const u8 as *const c_char)
        .wrapping_add(3 as size_t)
        .wrapping_add(1 as size_t) as c_uint;

    let mut srcSpan = malloc((size_of::<c_char>() as size_t).wrapping_mul(srcSpanLength as size_t))
        as *mut c_char;

    snprintf(
        srcSpan,
        srcSpanLength as size_t,
        b"src_span_%03i\0" as *const u8 as *const c_char,
        i,
    );

    provEnt.src_span = add_string(st, srcSpan) as StringIdx;

    return provEnt;
}

unsafe fn makeAnyProvEntries(
    mut cap: *mut Capability,
    mut start: c_int,
    mut end: c_int,
) -> *mut IpeBufferListNode {
    let n = end - start;
    let mut node = malloc(size_of::<IpeBufferListNode>() as size_t) as *mut IpeBufferListNode;
    (*node).tables = malloc((size_of::<*mut StgInfoTable>() as size_t).wrapping_mul(n as size_t))
        as *mut *const StgInfoTable;
    (*node).entries = malloc((size_of::<IpeBufferEntry>() as size_t).wrapping_mul(n as size_t))
        as *mut IpeBufferEntry;

    let mut st = StringTable {
        buffer: null_mut::<c_char>(),
        n: 0,
        size: 0,
    };

    init_string_table(&raw mut st);

    let mut unitIdLength = strlen(b"unit_id_\0" as *const u8 as *const c_char)
        .wrapping_add(3 as size_t)
        .wrapping_add(1 as size_t) as c_uint;

    let mut unitId =
        malloc((size_of::<c_char>() as size_t).wrapping_mul(unitIdLength as size_t)) as *mut c_char;

    snprintf(
        unitId,
        unitIdLength as size_t,
        b"unit_id_%03i\0" as *const u8 as *const c_char,
        start,
    );

    (*node).unit_id = add_string(&raw mut st, unitId) as StringIdx;

    let mut moduleLength = strlen(b"module_\0" as *const u8 as *const c_char)
        .wrapping_add(3 as size_t)
        .wrapping_add(1 as size_t) as c_uint;

    let mut module =
        malloc((size_of::<c_char>() as size_t).wrapping_mul(moduleLength as size_t)) as *mut c_char;

    snprintf(
        module,
        moduleLength as size_t,
        b"module_%03i\0" as *const u8 as *const c_char,
        start,
    );

    (*node).module_name = add_string(&raw mut st, module) as StringIdx;

    let mut i = start;

    while i < end {
        let mut closure = rts_mkInt(cap, 42 as HsInt);
        let ref mut fresh5 = *(*node).tables.offset(i as isize);
        *fresh5 = get_itbl(closure as *const StgClosure);
        *(*node).entries.offset(i as isize) = makeAnyProvEntry(cap, &raw mut st, i);
        i += 1;
    }

    (*node).next = null_mut::<IpeBufferListNode_>();
    (*node).compressed = 0 as StgWord;
    (*node).count = n as StgWord;
    (*node).string_table = st.buffer;

    return node;
}
