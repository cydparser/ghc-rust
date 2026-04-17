use crate::capability::Capability;
use crate::ffi::rts::ipe::{IpeBufferEntry, IpeBufferListNode, StringIdx};
use crate::ffi::rts::storage::closure_macros::get_itbl;
use crate::ffi::rts::types::{StgClosure, StgInfoTable};
use crate::ffi::stg::types::StgWord;
use crate::hs_ffi::HsInt;
use crate::prelude::*;
use crate::rts_api::rts_mkInt;

unsafe fn init_string_table(mut st: *mut StringTable) {
    (*st).size = 128;
    (*st).n = 0;
    (*st).buffer = malloc((*st).size) as *mut c_char;
}

unsafe fn add_string(mut st: *mut StringTable, mut s: *const c_char) -> u32 {
    let len = strlen(s) as usize;
    let n: u32 = (*st).n as u32;

    if (*st).n.wrapping_add(len).wrapping_add(1 as usize) > (*st).size {
        let new_size: usize = (2 as usize).wrapping_mul((*st).size).wrapping_add(len);
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
    (*st).n = (*st).n.wrapping_add(1 as usize);

    return n;
}

unsafe fn makeAnyProvEntry(
    mut cap: *mut Capability,
    mut st: *mut StringTable,
    mut i: i32,
) -> IpeBufferEntry {
    let mut provEnt = IpeBufferEntry {
        table_name: 0,
        closure_desc: 0,
        ty_desc: 0,
        label: 0,
        src_file: 0,
        src_span: 0,
    };

    let mut tableNameLength = strlen(c"table_name_".as_ptr())
        .wrapping_add(3 as usize)
        .wrapping_add(1 as usize) as u32;

    let mut tableName =
        malloc((size_of::<c_char>() as usize).wrapping_mul(tableNameLength as usize))
            as *mut c_char;

    snprintf(
        tableName,
        tableNameLength as usize,
        c"table_name_%03i".as_ptr(),
        i,
    );
    provEnt.table_name = add_string(st, tableName) as StringIdx;
    provEnt.closure_desc = i as u32;

    let mut tyDescLength = strlen(c"ty_desc_".as_ptr())
        .wrapping_add(3 as usize)
        .wrapping_add(1 as usize) as u32;

    let mut tyDesc =
        malloc((size_of::<c_char>() as usize).wrapping_mul(tyDescLength as usize)) as *mut c_char;

    snprintf(tyDesc, tyDescLength as usize, c"ty_desc_%03i".as_ptr(), i);
    provEnt.ty_desc = add_string(st, tyDesc) as StringIdx;

    let mut labelLength = strlen(c"label_".as_ptr())
        .wrapping_add(3 as usize)
        .wrapping_add(1 as usize) as u32;

    let mut label =
        malloc((size_of::<c_char>() as usize).wrapping_mul(labelLength as usize)) as *mut c_char;

    snprintf(label, labelLength as usize, c"label_%03i".as_ptr(), i);
    provEnt.label = add_string(st, label) as StringIdx;

    let mut srcFileLength = strlen(c"src_file_".as_ptr())
        .wrapping_add(3 as usize)
        .wrapping_add(1 as usize) as u32;

    let mut srcFile =
        malloc((size_of::<c_char>() as usize).wrapping_mul(srcFileLength as usize)) as *mut c_char;

    snprintf(
        srcFile,
        srcFileLength as usize,
        c"src_file_%03i".as_ptr(),
        i,
    );
    provEnt.src_file = add_string(st, srcFile) as StringIdx;

    let mut srcSpanLength = strlen(c"src_span_".as_ptr())
        .wrapping_add(3 as usize)
        .wrapping_add(1 as usize) as u32;

    let mut srcSpan =
        malloc((size_of::<c_char>() as usize).wrapping_mul(srcSpanLength as usize)) as *mut c_char;

    snprintf(
        srcSpan,
        srcSpanLength as usize,
        c"src_span_%03i".as_ptr(),
        i,
    );
    provEnt.src_span = add_string(st, srcSpan) as StringIdx;

    return provEnt;
}

unsafe fn makeAnyProvEntries(
    mut cap: *mut Capability,
    mut start: i32,
    mut end: i32,
) -> *mut IpeBufferListNode {
    let n = end - start;
    let mut node = malloc(size_of::<IpeBufferListNode>() as usize) as *mut IpeBufferListNode;

    (*node).tables = malloc((size_of::<*mut StgInfoTable>() as usize).wrapping_mul(n as usize))
        as *mut *const StgInfoTable;

    (*node).entries = malloc((size_of::<IpeBufferEntry>() as usize).wrapping_mul(n as usize))
        as *mut IpeBufferEntry;

    let mut st = StringTable {
        buffer: null_mut::<c_char>(),
        n: 0,
        size: 0,
    };

    init_string_table(&raw mut st);

    let mut unitIdLength = strlen(c"unit_id_".as_ptr())
        .wrapping_add(3 as usize)
        .wrapping_add(1 as usize) as u32;

    let mut unitId =
        malloc((size_of::<c_char>() as usize).wrapping_mul(unitIdLength as usize)) as *mut c_char;

    snprintf(
        unitId,
        unitIdLength as usize,
        c"unit_id_%03i".as_ptr(),
        start,
    );
    (*node).unit_id = add_string(&raw mut st, unitId) as StringIdx;

    let mut moduleLength = strlen(c"module_".as_ptr())
        .wrapping_add(3 as usize)
        .wrapping_add(1 as usize) as u32;

    let mut module =
        malloc((size_of::<c_char>() as usize).wrapping_mul(moduleLength as usize)) as *mut c_char;

    snprintf(
        module,
        moduleLength as usize,
        c"module_%03i".as_ptr(),
        start,
    );
    (*node).module_name = add_string(&raw mut st, module) as StringIdx;

    let mut i = start;

    while i < end {
        let mut closure = rts_mkInt(cap, 42);
        let ref mut fresh5 = *(*node).tables.offset(i as isize);
        *fresh5 = get_itbl(closure as *const StgClosure);
        *(*node).entries.offset(i as isize) = makeAnyProvEntry(cap, &raw mut st, i);
        i += 1;
    }

    (*node).next = null_mut::<IpeBufferListNode_>();
    (*node).compressed = 0;
    (*node).count = n as StgWord;
    (*node).string_table = st.buffer;

    return node;
}
