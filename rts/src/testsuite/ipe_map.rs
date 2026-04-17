use crate::capability::Capability;
use crate::ffi::rts::ipe::{
    InfoProv_, InfoProvEnt, InfoProvEnt_, IpeBufferEntry, IpeBufferListNode, StringIdx,
    formatClosureDescIpe, lookupIPE, registerInfoProvList,
};
use crate::ffi::rts::messages::{barf, errorBelch};
use crate::ffi::rts::storage::closure_macros::{UNTAG_CLOSURE, get_itbl};
use crate::ffi::rts::types::{StgClosure, StgInfoTable};
use crate::ffi::stg::types::StgWord;
use crate::hs_ffi::{HsInt, HsInt8, HsInt16, HsInt32, hs_exit, hs_init};
use crate::prelude::*;
use crate::rts_api::{
    HaskellObj, rts_lock, rts_mkInt, rts_mkInt8, rts_mkInt16, rts_mkInt32, rts_unlock,
};

unsafe fn main_0(mut argc: i32, mut argv: *mut *mut c_char) -> i32 {
    hs_init(&raw mut argc, &raw mut argv);

    let mut cap = rts_lock();
    shouldFindNothingInAnEmptyIPEMap(cap);

    let mut fortyTwo = shouldFindOneIfItHasBeenRegistered(cap);
    shouldFindTwoIfTwoHaveBeenRegistered(cap, fortyTwo);
    shouldFindTwoFromTheSameList(cap);
    shouldDealWithAnEmptyList(cap, fortyTwo);
    rts_unlock(cap);
    hs_exit();

    return 0;
}

unsafe fn lookupIPE_(mut r#where: *const c_char, mut itbl: *const StgInfoTable) -> InfoProvEnt {
    let mut ent = InfoProvEnt_ {
        info: null::<StgInfoTable>(),
        prov: InfoProv_ {
            table_name: null::<c_char>(),
            closure_desc: 0,
            ty_desc: null::<c_char>(),
            label: null::<c_char>(),
            unit_id: null::<c_char>(),
            module: null::<c_char>(),
            src_file: null::<c_char>(),
            src_span: null::<c_char>(),
        },
    };

    if !lookupIPE(itbl, &raw mut ent) {
        barf(c"%s: Expected to find IPE entry".as_ptr(), r#where);
    }

    return ent;
}

unsafe fn shouldFindNothingInAnEmptyIPEMap(mut cap: *mut Capability) {
    let mut fortyTwo = UNTAG_CLOSURE(rts_mkInt(cap, 42) as *mut StgClosure) as HaskellObj;

    let mut ent = InfoProvEnt_ {
        info: null::<StgInfoTable>(),
        prov: InfoProv_ {
            table_name: null::<c_char>(),
            closure_desc: 0,
            ty_desc: null::<c_char>(),
            label: null::<c_char>(),
            unit_id: null::<c_char>(),
            module: null::<c_char>(),
            src_file: null::<c_char>(),
            src_span: null::<c_char>(),
        },
    };

    if lookupIPE(get_itbl(fortyTwo as *const StgClosure), &raw mut ent) {
        barf(c"Found entry in an empty IPE map!".as_ptr());
    }
}

unsafe fn shouldFindOneIfItHasBeenRegistered(mut cap: *mut Capability) -> HaskellObj {
    let mut node = malloc(size_of::<IpeBufferListNode>() as usize) as *mut IpeBufferListNode;
    (*node).tables = malloc(size_of::<*mut StgInfoTable>() as usize) as *mut *const StgInfoTable;
    (*node).entries = malloc(size_of::<IpeBufferEntry>() as usize) as *mut IpeBufferEntry;

    let mut st = StringTable {
        buffer: null_mut::<c_char>(),
        n: 0,
        size: 0,
    };

    init_string_table(&raw mut st);
    (*node).unit_id = add_string(&raw mut st, c"unit-id".as_ptr()) as StringIdx;
    (*node).module_name = add_string(&raw mut st, c"TheModule".as_ptr()) as StringIdx;

    let mut fortyTwo = UNTAG_CLOSURE(rts_mkInt(cap, 42) as *mut StgClosure) as HaskellObj;
    (*node).next = null_mut::<IpeBufferListNode_>();
    (*node).compressed = 0;
    (*node).count = 1;

    let ref mut fresh5 = *(*node).tables.offset(0);
    *fresh5 = get_itbl(fortyTwo as *const StgClosure);
    *(*node).entries.offset(0) = makeAnyProvEntry(cap, &raw mut st, 42);
    (*node).entries_size = size_of::<IpeBufferEntry>() as StgWord;
    (*node).string_table = st.buffer;
    (*node).string_table_size = st.size as StgWord;
    registerInfoProvList(node);

    let result = lookupIPE_(
        c"shouldFindOneIfItHasBeenRegistered".as_ptr(),
        get_itbl(fortyTwo as *const StgClosure),
    ) as InfoProvEnt;

    let mut closure_desc_buf: [c_char; 11] = [0; 11];
    formatClosureDescIpe(&raw const result, &raw mut closure_desc_buf as *mut c_char);
    assertStringsEqual(result.prov.table_name, c"table_name_042".as_ptr());
    assertStringsEqual(&raw mut closure_desc_buf as *mut c_char, c"42".as_ptr());
    assertStringsEqual(result.prov.ty_desc, c"ty_desc_042".as_ptr());
    assertStringsEqual(result.prov.label, c"label_042".as_ptr());
    assertStringsEqual(result.prov.unit_id, c"unit-id".as_ptr());
    assertStringsEqual(result.prov.module, c"TheModule".as_ptr());
    assertStringsEqual(result.prov.src_file, c"src_file_042".as_ptr());
    assertStringsEqual(result.prov.src_span, c"src_span_042".as_ptr());

    return fortyTwo;
}

unsafe fn shouldFindTwoIfTwoHaveBeenRegistered(mut cap: *mut Capability, mut fortyTwo: HaskellObj) {
    let mut node = malloc(size_of::<IpeBufferListNode>() as usize) as *mut IpeBufferListNode;
    (*node).tables = malloc(size_of::<*mut StgInfoTable>() as usize) as *mut *const StgInfoTable;
    (*node).entries = malloc(size_of::<IpeBufferEntry>() as usize) as *mut IpeBufferEntry;

    let mut st = StringTable {
        buffer: null_mut::<c_char>(),
        n: 0,
        size: 0,
    };

    init_string_table(&raw mut st);
    (*node).unit_id = add_string(&raw mut st, c"unit-id".as_ptr()) as StringIdx;
    (*node).module_name = add_string(&raw mut st, c"TheModule".as_ptr()) as StringIdx;

    let mut twentyThree = UNTAG_CLOSURE(rts_mkInt8(cap, 23) as *mut StgClosure) as HaskellObj;
    (*node).next = null_mut::<IpeBufferListNode_>();
    (*node).compressed = 0;
    (*node).count = 1;

    let ref mut fresh6 = *(*node).tables.offset(0);
    *fresh6 = get_itbl(twentyThree as *const StgClosure);
    *(*node).entries.offset(0) = makeAnyProvEntry(cap, &raw mut st, 23);
    (*node).entries_size = size_of::<IpeBufferEntry>() as StgWord;
    (*node).string_table = st.buffer;
    (*node).string_table_size = st.size as StgWord;
    registerInfoProvList(node);

    let mut resultFortyTwo = lookupIPE_(
        c"shouldFindTwoIfTwoHaveBeenRegistered".as_ptr(),
        get_itbl(fortyTwo as *const StgClosure),
    );

    assertStringsEqual(resultFortyTwo.prov.table_name, c"table_name_042".as_ptr());

    let mut resultTwentyThree = lookupIPE_(
        c"shouldFindTwoIfTwoHaveBeenRegistered".as_ptr(),
        get_itbl(twentyThree as *const StgClosure),
    );

    assertStringsEqual(
        resultTwentyThree.prov.table_name,
        c"table_name_023".as_ptr(),
    );
}

unsafe fn shouldFindTwoFromTheSameList(mut cap: *mut Capability) {
    let mut node = malloc(size_of::<IpeBufferListNode>() as usize) as *mut IpeBufferListNode;

    (*node).tables = malloc((size_of::<*mut StgInfoTable>() as usize).wrapping_mul(2 as usize))
        as *mut *const StgInfoTable;

    (*node).entries = malloc((size_of::<IpeBufferEntry>() as usize).wrapping_mul(2 as usize))
        as *mut IpeBufferEntry;

    let mut st = StringTable {
        buffer: null_mut::<c_char>(),
        n: 0,
        size: 0,
    };

    init_string_table(&raw mut st);

    let mut one = UNTAG_CLOSURE(rts_mkInt16(cap, 1) as *mut StgClosure) as HaskellObj;
    let mut two = UNTAG_CLOSURE(rts_mkInt32(cap, 2) as *mut StgClosure) as HaskellObj;
    (*node).next = null_mut::<IpeBufferListNode_>();
    (*node).compressed = 0;
    (*node).count = 2;

    let ref mut fresh7 = *(*node).tables.offset(0);
    *fresh7 = get_itbl(one as *const StgClosure);

    let ref mut fresh8 = *(*node).tables.offset(1);
    *fresh8 = get_itbl(two as *const StgClosure);
    *(*node).entries.offset(0) = makeAnyProvEntry(cap, &raw mut st, 1);
    *(*node).entries.offset(1) = makeAnyProvEntry(cap, &raw mut st, 2);
    (*node).entries_size =
        (size_of::<IpeBufferEntry>() as usize).wrapping_mul(2 as usize) as StgWord;
    (*node).string_table = st.buffer;
    (*node).string_table_size = st.size as StgWord;
    registerInfoProvList(node);

    let mut resultOne = lookupIPE_(
        c"shouldFindTwoFromTheSameList".as_ptr(),
        get_itbl(one as *const StgClosure),
    );

    assertStringsEqual(resultOne.prov.table_name, c"table_name_001".as_ptr());

    let mut resultTwo = lookupIPE_(
        c"shouldFindTwoFromTheSameList".as_ptr(),
        get_itbl(two as *const StgClosure),
    );

    assertStringsEqual(resultTwo.prov.table_name, c"table_name_002".as_ptr());
}

unsafe fn shouldDealWithAnEmptyList(mut cap: *mut Capability, mut fortyTwo: HaskellObj) {
    let mut node = malloc(size_of::<IpeBufferListNode>() as usize) as *mut IpeBufferListNode;
    (*node).count = 0;
    (*node).next = null_mut::<IpeBufferListNode_>();
    (*node).string_table = c"".as_ptr();
    registerInfoProvList(node);

    let mut resultFortyTwo = lookupIPE_(
        c"shouldDealWithAnEmptyList".as_ptr(),
        get_itbl(fortyTwo as *const StgClosure),
    );

    assertStringsEqual(resultFortyTwo.prov.table_name, c"table_name_042".as_ptr());
}

unsafe fn assertStringsEqual(mut s1: *const c_char, mut s2: *const c_char) {
    if strcmp(s1, s2) != 0 {
        errorBelch(c"%s != %s".as_ptr(), s1, s2);
        exit(1);
    }
}

fn main() {
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args()
        .map(|arg| {
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();

    let mut args_ptrs: Vec<*mut c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut c_char)
        .chain(::core::iter::once(null_mut()))
        .collect();

    unsafe {
        ::std::process::exit(main_0(
            (args_ptrs.len() - 1) as i32,
            args_ptrs.as_mut_ptr() as *mut *mut c_char,
        ) as i32)
    }
}
