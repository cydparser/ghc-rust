use crate::ffi::rts::_assertFail;
use crate::ffi::stg::types::StgWord;
use crate::ffi::stg::types::StgWord;
use crate::hash::{HashSet, HashTable, IterHashFn, MapHashFn, MapHashFnKeys, StrHashTable};
use crate::prelude::*;
use crate::rts_utils::{stgFree, stgMallocBytes};

#[cfg(test)]
mod tests;

pub(crate) type HashTable = hashtable;

pub(crate) type StrHashTable = strhashtable;

pub(crate) type HashFunction = unsafe extern "C" fn(*const HashTable, StgWord) -> c_int;

pub(crate) type CompareFunction = unsafe extern "C" fn(StgWord, StgWord) -> c_int;

pub(crate) type MapHashFn = Option<unsafe extern "C" fn(*mut c_void, StgWord, *const c_void) -> ()>;

pub(crate) type MapHashFnKeys =
    Option<unsafe extern "C" fn(*mut c_void, *mut StgWord, *const c_void) -> ()>;

pub(crate) type IterHashFn =
    Option<unsafe extern "C" fn(*mut c_void, StgWord, *const c_void) -> bool>;

pub(crate) type HashSet = hashtable;

#[inline]
pub(crate) unsafe fn allocStrHashTable() -> *mut StrHashTable {
    return allocHashTable() as *mut StrHashTable;
}

#[inline]
pub(crate) unsafe fn freeStrHashTable(
    mut table: *mut StrHashTable,
    mut freeDataFun: Option<unsafe extern "C" fn(*mut c_void) -> ()>,
) {
    freeHashTable(table as *mut HashTable, freeDataFun);
}

#[inline]
pub(crate) unsafe fn allocHashSet() -> *mut HashSet {
    return allocHashTable() as *mut HashSet;
}

#[inline]
pub(crate) unsafe fn freeHashSet(mut set: *mut HashSet) {
    freeHashTable(set as *mut HashTable, None);
}

#[inline]
pub(crate) unsafe fn insertHashSet(mut set: *mut HashSet, mut key: StgWord) {
    insertHashTable(set as *mut HashTable, key, null::<c_void>());
}

extern "C" {
    pub(crate) fn XXH3_64bits_withSeed(
        input: *const c_void,
        length: usize,
        seed: XXH64_hash_t,
    ) -> XXH64_hash_t;
}

#[ffi(compiler, utils)]
#[repr(C)]
#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub struct hashtable {
    pub split: c_int,
    pub max: c_int,
    pub mask1: c_int,
    pub mask2: c_int,
    pub kcount: c_int,
    pub bcount: c_int,
    pub dir: [*mut *mut HashList; 1024],
    pub freeList: *mut HashList,
    pub chunks: *mut HashListChunk,
}

#[cfg(test)]
impl Arbitrary for hashtable {
    fn arbitrary(g: &mut Gen) -> Self {
        hashtable {
            _address: Arbitrary::arbitrary(g),
        }
    }
}

type HashListChunk = chunklist;

/// cbindgen:no-export
struct chunklist {
    next: *mut chunklist,
}

type HashList = hashlist;

/// cbindgen:no-export
struct hashlist {
    key: StgWord,
    data: *const c_void,
    next: *mut hashlist,
}

/// cbindgen:no-export
struct strhashtable {
    table: hashtable,
}

type XXH64_hash_t = u64;

const HSEGSIZE: i32 = 1024;

const HDIRSIZE: i32 = 1024;

const HLOAD: i32 = 5;

const HCHUNK: usize = (1024 as usize)
    .wrapping_mul(size_of::<W_>() as usize)
    .wrapping_div(size_of::<HashList>() as usize);

unsafe fn hashWord(mut table: *const HashTable, mut key: StgWord) -> i32 {
    let mut bucket: i32 = 0;
    key = key.wrapping_div(size_of::<StgWord>() as StgWord);
    bucket = (key & (*table).mask1 as StgWord) as i32;

    if bucket < (*table).split {
        bucket = (key & (*table).mask2 as StgWord) as i32;
    }

    return bucket;
}

unsafe fn hashBuffer(mut table: *const HashTable, mut buf: *const c_void, mut len: usize) -> i32 {
    let mut key: *const c_char = buf as *mut c_char;
    let mut h = XXH3_64bits_withSeed(key as *const c_void, len, 1048583) as StgWord;
    let mut bucket = (h & (*table).mask1 as StgWord) as i32;

    if bucket < (*table).split {
        bucket = (h & (*table).mask2 as StgWord) as i32;
    }

    return bucket;
}

unsafe fn hashStr(mut table: *const HashTable, mut w: StgWord) -> i32 {
    let mut key: *const c_char = w as *mut c_char;

    return hashBuffer(table, key as *const c_void, strlen(key));
}

unsafe fn compareWord(mut key1: StgWord, mut key2: StgWord) -> i32 {
    return (key1 == key2) as i32;
}

unsafe fn compareStr(mut key1: StgWord, mut key2: StgWord) -> i32 {
    return (strcmp(key1 as *mut c_char, key2 as *mut c_char) == 0) as i32;
}

unsafe fn allocSegment(mut table: *mut HashTable, mut segment: i32) {
    (*table).dir[segment as usize] = stgMallocBytes(
        (HSEGSIZE as usize).wrapping_mul(size_of::<*mut HashList>() as usize),
        c"allocSegment".as_ptr(),
    ) as *mut *mut HashList;
}

unsafe fn expand(mut table: *mut HashTable, mut f: Option<HashFunction>) {
    let mut oldsegment: i32 = 0;
    let mut oldindex: i32 = 0;
    let mut newbucket: i32 = 0;
    let mut newsegment: i32 = 0;
    let mut newindex: i32 = 0;
    let mut hl = null_mut::<HashList>();
    let mut next = null_mut::<HashList>();
    let mut old = null_mut::<HashList>();
    let mut new = null_mut::<HashList>();

    if (*table).split + (*table).max >= HDIRSIZE * HSEGSIZE {
        return;
    }

    oldsegment = (*table).split / HSEGSIZE;
    oldindex = (*table).split % HSEGSIZE;
    newbucket = (*table).max + (*table).split;
    newsegment = newbucket / HSEGSIZE;
    newindex = newbucket % HSEGSIZE;

    if newindex == 0 {
        allocSegment(table, newsegment);
    }

    (*table).split += 1;

    if (*table).split == (*table).max {
        (*table).split = 0;
        (*table).max *= 2;
        (*table).mask1 = (*table).mask2;
        (*table).mask2 = (*table).mask2 << 1 | 1;
    }

    (*table).bcount += 1;
    new = null_mut::<HashList>();
    old = new;
    hl = *(*table).dir[oldsegment as usize].offset(oldindex as isize);

    while !hl.is_null() {
        next = (*hl).next as *mut HashList;

        if f.expect("non-null function pointer")(table, (*hl).key) == newbucket {
            (*hl).next = new as *mut hashlist;
            new = hl;
        } else {
            (*hl).next = old as *mut hashlist;
            old = hl;
        }

        hl = next;
    }

    let ref mut fresh13 = *(*table).dir[oldsegment as usize].offset(oldindex as isize);
    *fresh13 = old;

    let ref mut fresh14 = *(*table).dir[newsegment as usize].offset(newindex as isize);
    *fresh14 = new;
}

unsafe fn lookupHashTable_inlined(
    mut table: *const HashTable,
    mut key: StgWord,
    mut f: Option<HashFunction>,
    mut cmp: Option<CompareFunction>,
) -> *mut c_void {
    let mut bucket: i32 = 0;
    let mut segment: i32 = 0;
    let mut index: i32 = 0;
    let mut hl = null_mut::<HashList>();
    bucket = f.expect("non-null function pointer")(table, key);
    segment = bucket / HSEGSIZE;
    index = bucket % HSEGSIZE;
    hl = *(*table).dir[segment as usize].offset(index as isize);

    while !hl.is_null() {
        if cmp.expect("non-null function pointer")((*hl).key, key) != 0 {
            return (*hl).data as *mut c_void;
        }

        hl = (*hl).next as *mut HashList;
    }

    return NULL;
}

unsafe fn lookupHashTable_(
    mut table: *const HashTable,
    mut key: StgWord,
    mut f: Option<HashFunction>,
    mut cmp: Option<CompareFunction>,
) -> *mut c_void {
    return lookupHashTable_inlined(table, key, f, cmp);
}

unsafe fn lookupHashTable(mut table: *const HashTable, mut key: StgWord) -> *mut c_void {
    return lookupHashTable_inlined(
        table,
        key,
        Some(hashWord as unsafe extern "C" fn(*const HashTable, StgWord) -> c_int),
        Some(compareWord as unsafe extern "C" fn(StgWord, StgWord) -> c_int),
    );
}

unsafe fn lookupStrHashTable(
    mut table: *const StrHashTable,
    mut key: *const c_char,
) -> *mut c_void {
    return lookupHashTable_inlined(
        &raw const (*table).table,
        key as StgWord,
        Some(hashStr as unsafe extern "C" fn(*const HashTable, StgWord) -> c_int),
        Some(compareStr as unsafe extern "C" fn(StgWord, StgWord) -> c_int),
    );
}

unsafe fn keysHashTable(mut table: *mut HashTable, mut keys: *mut StgWord, mut szKeys: i32) -> i32 {
    let mut segment: i32 = 0;
    let mut index: i32 = 0;
    let mut k = 0;
    let mut hl = null_mut::<HashList>();
    segment = ((*table).max + (*table).split - 1) / HSEGSIZE;
    index = ((*table).max + (*table).split - 1) % HSEGSIZE;

    while segment >= 0 && k < szKeys {
        while index >= 0 && k < szKeys {
            hl = *(*table).dir[segment as usize].offset(index as isize);

            while !hl.is_null() && k < szKeys {
                *keys.offset(k as isize) = (*hl).key;
                k += 1;
                hl = (*hl).next as *mut HashList;
            }

            index -= 1;
        }

        segment -= 1;
        index = HSEGSIZE - 1;
    }

    return k;
}

unsafe fn allocHashList(mut table: *mut HashTable) -> *mut HashList {
    if !(*table).freeList.is_null() {
        let mut hl = (*table).freeList;
        (*table).freeList = (*hl).next as *mut HashList;

        return hl;
    } else {
        let mut cl = stgMallocBytes(
            (size_of::<HashListChunk>() as usize)
                .wrapping_add(HCHUNK.wrapping_mul(size_of::<HashList>() as usize)),
            c"allocHashList".as_ptr(),
        ) as *mut HashListChunk;

        let mut hl_0 = cl.offset(1) as *mut HashListChunk as *mut HashList;
        (*cl).next = (*table).chunks as *mut chunklist;
        (*table).chunks = cl;
        (*table).freeList = hl_0.offset(1);

        let mut p = (*table).freeList;

        while p < hl_0.offset(HCHUNK as isize).offset(-1) {
            (*p).next = p.offset(1) as *mut hashlist;
            p = p.offset(1);
        }

        (*p).next = null_mut::<hashlist>();

        return hl_0;
    };
}

unsafe fn freeHashList(mut table: *mut HashTable, mut hl: *mut HashList) {
    (*hl).next = (*table).freeList as *mut hashlist;
    (*table).freeList = hl;
}

unsafe fn insertHashTable_inlined(
    mut table: *mut HashTable,
    mut key: StgWord,
    mut data: *const c_void,
    mut f: Option<HashFunction>,
) {
    let mut bucket: i32 = 0;
    let mut segment: i32 = 0;
    let mut index: i32 = 0;
    let mut hl = null_mut::<HashList>();
    (*table).kcount += 1;

    if (*table).kcount >= HLOAD * (*table).bcount {
        expand(table, f);
    }

    bucket = f.expect("non-null function pointer")(table, key);
    segment = bucket / HSEGSIZE;
    index = bucket % HSEGSIZE;
    hl = allocHashList(table);
    (*hl).key = key;
    (*hl).data = data;
    (*hl).next = *(*table).dir[segment as usize].offset(index as isize) as *mut hashlist;

    let ref mut fresh12 = *(*table).dir[segment as usize].offset(index as isize);
    *fresh12 = hl;
}

unsafe fn insertHashTable_(
    mut table: *mut HashTable,
    mut key: StgWord,
    mut data: *const c_void,
    mut f: Option<HashFunction>,
) {
    return insertHashTable_inlined(table, key, data, f);
}

unsafe fn insertHashTable(mut table: *mut HashTable, mut key: StgWord, mut data: *const c_void) {
    insertHashTable_inlined(
        table,
        key,
        data,
        Some(hashWord as unsafe extern "C" fn(*const HashTable, StgWord) -> c_int),
    );
}

unsafe fn insertStrHashTable(
    mut table: *mut StrHashTable,
    mut key: *const c_char,
    mut data: *const c_void,
) {
    insertHashTable_inlined(
        &raw mut (*table).table,
        key as StgWord,
        data,
        Some(hashStr as unsafe extern "C" fn(*const HashTable, StgWord) -> c_int),
    );
}

unsafe fn removeHashTable_inlined(
    mut table: *mut HashTable,
    mut key: StgWord,
    mut data: *const c_void,
    mut f: Option<HashFunction>,
    mut cmp: Option<CompareFunction>,
) -> *mut c_void {
    let mut bucket: i32 = 0;
    let mut segment: i32 = 0;
    let mut index: i32 = 0;
    let mut hl = null_mut::<HashList>();
    let mut prev = null_mut::<HashList>();
    bucket = f.expect("non-null function pointer")(table, key);
    segment = bucket / HSEGSIZE;
    index = bucket % HSEGSIZE;
    hl = *(*table).dir[segment as usize].offset(index as isize);

    while !hl.is_null() {
        if cmp.expect("non-null function pointer")((*hl).key, key) != 0
            && (data.is_null() || (*hl).data == data)
        {
            if prev.is_null() {
                let ref mut fresh15 = *(*table).dir[segment as usize].offset(index as isize);
                *fresh15 = (*hl).next as *mut HashList;
            } else {
                (*prev).next = (*hl).next;
            }

            freeHashList(table, hl);
            (*table).kcount -= 1;

            return (*hl).data as *mut c_void;
        }

        prev = hl;
        hl = (*hl).next as *mut HashList;
    }

    if data.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"/Users/cyd/src/ghc/rts/Hash.c".as_ptr(), 405);
    }

    return NULL;
}

unsafe fn removeHashTable_(
    mut table: *mut HashTable,
    mut key: StgWord,
    mut data: *const c_void,
    mut f: Option<HashFunction>,
    mut cmp: Option<CompareFunction>,
) -> *mut c_void {
    return removeHashTable_inlined(table, key, data, f, cmp);
}

unsafe fn removeHashTable(
    mut table: *mut HashTable,
    mut key: StgWord,
    mut data: *const c_void,
) -> *mut c_void {
    return removeHashTable_inlined(
        table,
        key,
        data,
        Some(hashWord as unsafe extern "C" fn(*const HashTable, StgWord) -> c_int),
        Some(compareWord as unsafe extern "C" fn(StgWord, StgWord) -> c_int),
    );
}

unsafe fn removeStrHashTable(
    mut table: *mut StrHashTable,
    mut key: *const c_char,
    mut data: *const c_void,
) -> *mut c_void {
    return removeHashTable_inlined(
        &raw mut (*table).table,
        key as StgWord,
        data,
        Some(hashStr as unsafe extern "C" fn(*const HashTable, StgWord) -> c_int),
        Some(compareStr as unsafe extern "C" fn(StgWord, StgWord) -> c_int),
    );
}

unsafe fn freeHashTable(
    mut table: *mut HashTable,
    mut freeDataFun: Option<unsafe extern "C" fn(*mut c_void) -> ()>,
) {
    let mut segment = (((*table).max + (*table).split - 1) / HSEGSIZE) as i64;
    let mut index = (((*table).max + (*table).split - 1) % HSEGSIZE) as i64;

    while segment >= 0 {
        if freeDataFun.is_some() {
            while index >= 0 {
                let mut next = null_mut::<HashList>();
                let mut hl = *(*table).dir[segment as usize].offset(index as isize);

                while !hl.is_null() {
                    next = (*hl).next as *mut HashList;
                    Some(freeDataFun.expect("non-null function pointer"))
                        .expect("non-null function pointer")(
                        (*hl).data as *mut c_void
                    );
                    hl = next;
                }

                index -= 1;
            }
        }

        stgFree((*table).dir[segment as usize] as *mut c_void);
        segment -= 1;
        index = (HSEGSIZE - 1) as i64;
    }

    let mut cl = (*table).chunks;

    while !cl.is_null() {
        let mut old = cl;
        cl = (*cl).next as *mut HashListChunk;
        stgFree(old as *mut c_void);
    }

    stgFree(table as *mut c_void);
}

unsafe fn mapHashTable(mut table: *mut HashTable, mut data: *mut c_void, mut r#fn: MapHashFn) {
    let mut segment = (((*table).max + (*table).split - 1) / HSEGSIZE) as i64;
    let mut index = (((*table).max + (*table).split - 1) % HSEGSIZE) as i64;

    while segment >= 0 {
        while index >= 0 {
            let mut hl = *(*table).dir[segment as usize].offset(index as isize);

            while !hl.is_null() {
                r#fn.expect("non-null function pointer")(data, (*hl).key, (*hl).data);
                hl = (*hl).next as *mut HashList;
            }

            index -= 1;
        }

        segment -= 1;
        index = (HSEGSIZE - 1) as i64;
    }
}

unsafe fn mapHashTableKeys(
    mut table: *mut HashTable,
    mut data: *mut c_void,
    mut r#fn: MapHashFnKeys,
) {
    let mut segment = (((*table).max + (*table).split - 1) / HSEGSIZE) as i64;
    let mut index = (((*table).max + (*table).split - 1) % HSEGSIZE) as i64;

    while segment >= 0 {
        while index >= 0 {
            let mut hl = *(*table).dir[segment as usize].offset(index as isize);

            while !hl.is_null() {
                r#fn.expect("non-null function pointer")(data, &raw mut (*hl).key, (*hl).data);

                hl = (*hl).next as *mut HashList;
            }

            index -= 1;
        }

        segment -= 1;
        index = (HSEGSIZE - 1) as i64;
    }
}

unsafe fn iterHashTable(mut table: *mut HashTable, mut data: *mut c_void, mut r#fn: IterHashFn) {
    let mut segment = (((*table).max + (*table).split - 1) / HSEGSIZE) as i64;
    let mut index = (((*table).max + (*table).split - 1) % HSEGSIZE) as i64;

    while segment >= 0 {
        while index >= 0 {
            let mut hl = *(*table).dir[segment as usize].offset(index as isize);

            while !hl.is_null() {
                if !r#fn.expect("non-null function pointer")(data, (*hl).key, (*hl).data) {
                    return;
                }

                hl = (*hl).next as *mut HashList;
            }

            index -= 1;
        }

        segment -= 1;
        index = (HSEGSIZE - 1) as i64;
    }
}

unsafe fn allocHashTable() -> *mut HashTable {
    let mut table = null_mut::<HashTable>();
    let mut hb = null_mut::<*mut HashList>();
    table = stgMallocBytes(size_of::<HashTable>() as usize, c"allocHashTable".as_ptr())
        as *mut HashTable;
    allocSegment(table, 0);
    hb = (*table).dir[0];

    while hb < (*table).dir[0].offset(HSEGSIZE as isize) {
        *hb = null_mut::<HashList>();
        hb = hb.offset(1);
    }

    (*table).split = 0;
    (*table).max = HSEGSIZE;
    (*table).mask1 = HSEGSIZE - 1;
    (*table).mask2 = 2 * HSEGSIZE - 1;
    (*table).kcount = 0;
    (*table).bcount = HSEGSIZE;
    (*table).freeList = null_mut::<HashList>();
    (*table).chunks = null_mut::<HashListChunk>();

    return table;
}

unsafe fn keyCountHashTable(mut table: *mut HashTable) -> i32 {
    return (*table).kcount;
}
