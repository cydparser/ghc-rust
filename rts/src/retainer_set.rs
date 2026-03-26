use crate::arena::{Arena, arenaAlloc, arenaFree, newArena};
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::prof::ccs::CostCentreStack;
use crate::ffi::rts::prof::ccs::CostCentreStack;
use crate::ffi::stg::W_;
use crate::ffi::stg::types::StgWord;
use crate::ffi::stg::types::{StgWord, StgWord8};
use crate::prelude::*;
use crate::profiling::fprintCCS;
use crate::retainer_set::{_RetainerSet, BINARY_SEARCH_THRESHOLD, RetainerSet, retainer};
use crate::rts_utils::{stgFree, stgMallocBytes};
use crate::trace::traceHeapProfSampleString;

pub(crate) type retainer = *mut CostCentreStack;

/// cbindgen:no-export
pub(crate) struct _RetainerSet {
    pub(crate) num: uint32_t,
    pub(crate) hashKey: StgWord,
    pub(crate) link: *mut _RetainerSet,
    pub(crate) id: c_int,
    pub(crate) element: [retainer; 0],
}

pub(crate) type RetainerSet = _RetainerSet;

pub(crate) const BINARY_SEARCH_THRESHOLD: c_int = 8 as c_int;

#[inline]
pub(crate) unsafe fn isMember(mut r: retainer, mut rs: *mut RetainerSet) -> bool {
    let mut i: c_int = 0;
    let mut left: c_int = 0;
    let mut right: c_int = 0;
    let mut ri = null_mut::<CostCentreStack>();

    if rs == &raw mut rs_MANY {
        return r#true != 0;
    }

    if (*rs).num < BINARY_SEARCH_THRESHOLD as uint32_t {
        i = 0 as c_int;

        while i < (*rs).num as c_int {
            ri = *(&raw mut (*rs).element as *mut retainer).offset(i as isize);

            if r == ri {
                return r#true != 0;
            } else if r < ri {
                return r#false != 0;
            }

            i += 1;
        }
    } else {
        left = 0 as c_int;
        right = (*rs).num.wrapping_sub(1 as uint32_t) as c_int;

        while left <= right {
            i = (left + right) / 2 as c_int;
            ri = *(&raw mut (*rs).element as *mut retainer).offset(i as isize);

            if r == ri {
                return r#true != 0;
            } else if r < ri {
                right = i - 1 as c_int;
            } else {
                left = i + 1 as c_int;
            }
        }
    }

    return r#false != 0;
}

const HASH_TABLE_SIZE: c_int = 255 as c_int;

static mut hashTable: [*mut RetainerSet; 255] = [null::<RetainerSet>() as *mut RetainerSet; 255];

static mut arena: *mut Arena = null::<Arena>() as *mut Arena;

static mut nextId: c_int = 0;

static mut rs_MANY: RetainerSet = _RetainerSet {
    num: 0 as uint32_t,
    hashKey: 0 as StgWord,
    link: null::<_RetainerSet>() as *mut _RetainerSet,
    id: 1 as c_int,
    element: [null::<CostCentreStack>() as *mut CostCentreStack; 0],
};

#[inline]
unsafe fn sizeofRetainerSet(mut elems: c_int) -> size_t {
    return (size_of::<RetainerSet>() as size_t)
        .wrapping_add((elems as size_t).wrapping_mul(size_of::<retainer>() as size_t));
}

unsafe fn initializeAllRetainerSet() {
    let mut i: c_int = 0;
    arena = newArena();
    i = 0 as c_int;

    while i < HASH_TABLE_SIZE {
        hashTable[i as usize] = null_mut::<RetainerSet>();
        i += 1;
    }

    nextId = 2 as c_int;
}

unsafe fn closeAllRetainerSet() {
    arenaFree(arena);
}

unsafe fn singleton(mut r: retainer) -> *mut RetainerSet {
    let mut rs = null_mut::<RetainerSet>();
    let mut hk: StgWord = 0;
    hk = r as StgWord;
    rs = hashTable[hk.wrapping_rem(HASH_TABLE_SIZE as StgWord) as usize];

    while !rs.is_null() {
        if (*rs).num == 1 as uint32_t
            && *(&raw mut (*rs).element as *mut retainer).offset(0 as c_int as isize) == r
        {
            return rs;
        }

        rs = (*rs).link as *mut RetainerSet;
    }

    rs = arenaAlloc(arena, sizeofRetainerSet(1 as c_int)) as *mut RetainerSet;
    (*rs).num = 1 as uint32_t;
    (*rs).hashKey = hk;
    (*rs).link =
        hashTable[hk.wrapping_rem(HASH_TABLE_SIZE as StgWord) as usize] as *mut _RetainerSet;

    let fresh5 = nextId;
    nextId = nextId + 1;
    (*rs).id = fresh5;

    let ref mut fresh6 = *(&raw mut (*rs).element as *mut retainer).offset(0 as c_int as isize);
    *fresh6 = r;
    hashTable[hk.wrapping_rem(HASH_TABLE_SIZE as StgWord) as usize] = rs;

    return rs;
}

unsafe fn addElement(mut r: retainer, mut rs: *mut RetainerSet) -> *mut RetainerSet {
    let mut i: uint32_t = 0;
    let mut nl: uint32_t = 0;
    let mut nrs = null_mut::<RetainerSet>();
    let mut hk: StgWord = 0;

    if rs == &raw mut rs_MANY || (*rs).num == RtsFlags.ProfFlags.maxRetainerSetSize {
        return &raw mut rs_MANY;
    }

    nl = 0 as uint32_t;

    while nl < (*rs).num {
        if r < *(&raw mut (*rs).element as *mut retainer).offset(nl as isize) {
            break;
        }

        nl = nl.wrapping_add(1);
    }

    hk = (r as StgWord).wrapping_add((*rs).hashKey);
    nrs = hashTable[hk.wrapping_rem(HASH_TABLE_SIZE as StgWord) as usize];

    while !nrs.is_null() {
        if !((*rs).num.wrapping_add(1 as uint32_t) != (*nrs).num) {
            i = 0 as uint32_t;

            while i < nl {
                if *(&raw mut (*rs).element as *mut retainer).offset(i as isize)
                    != *(&raw mut (*nrs).element as *mut retainer).offset(i as isize)
                {
                    break;
                }

                i = i.wrapping_add(1);
            }

            if !(i < nl) {
                if !(r != *(&raw mut (*nrs).element as *mut retainer).offset(i as isize)) {
                    while i < (*rs).num {
                        if *(&raw mut (*rs).element as *mut retainer).offset(i as isize)
                            != *(&raw mut (*nrs).element as *mut retainer)
                                .offset(i.wrapping_add(1 as uint32_t) as isize)
                        {
                            break;
                        }

                        i = i.wrapping_add(1);
                    }

                    if !(i < (*rs).num) {
                        return nrs;
                    }
                }
            }
        }

        nrs = (*nrs).link as *mut RetainerSet;
    }

    nrs = arenaAlloc(
        arena,
        sizeofRetainerSet((*rs).num.wrapping_add(1 as uint32_t) as c_int),
    ) as *mut RetainerSet;

    (*nrs).num = (*rs).num.wrapping_add(1 as uint32_t);
    (*nrs).hashKey = hk;
    (*nrs).link =
        hashTable[hk.wrapping_rem(HASH_TABLE_SIZE as StgWord) as usize] as *mut _RetainerSet;

    let fresh7 = nextId;
    nextId = nextId + 1;
    (*nrs).id = fresh7;
    i = 0 as uint32_t;

    while i < nl {
        let ref mut fresh8 = *(&raw mut (*nrs).element as *mut retainer).offset(i as isize);
        *fresh8 = *(&raw mut (*rs).element as *mut retainer).offset(i as isize);
        i = i.wrapping_add(1);
    }

    let ref mut fresh9 = *(&raw mut (*nrs).element as *mut retainer).offset(i as isize);
    *fresh9 = r;

    while i < (*rs).num {
        let ref mut fresh10 = *(&raw mut (*nrs).element as *mut retainer)
            .offset(i.wrapping_add(1 as uint32_t) as isize);
        *fresh10 = *(&raw mut (*rs).element as *mut retainer).offset(i as isize);
        i = i.wrapping_add(1);
    }

    hashTable[hk.wrapping_rem(HASH_TABLE_SIZE as StgWord) as usize] = nrs;

    return nrs;
}

unsafe fn printRetainer(mut f: *mut FILE, mut ccs: retainer) {
    fprintCCS(f, ccs as *mut CostCentreStack);
}

unsafe fn printRetainerSetShort(
    mut f: *mut FILE,
    mut rs: *mut RetainerSet,
    mut total_size: W_,
    mut max_length: uint32_t,
) {
    let vla = max_length.wrapping_add(1 as uint32_t) as usize;
    let mut tmp: Vec<c_char> = ::std::vec::from_elem(0, vla);
    let mut size: uint32_t = 0;
    let mut j: uint32_t = 0;
    *tmp.as_mut_ptr().offset(max_length as isize) = '\0' as i32 as c_char;

    sprintf(
        tmp.as_mut_ptr().offset(0 as c_int as isize),
        b"(%d)\0" as *const u8 as *const c_char,
        -(*rs).id,
    );

    size = strlen(tmp.as_mut_ptr()) as uint32_t;
    j = 0 as uint32_t;

    while j < (*rs).num {
        if j < (*rs).num.wrapping_sub(1 as uint32_t) {
            strncpy(
                tmp.as_mut_ptr().offset(size as isize),
                (*(**(&raw mut (*rs).element as *mut retainer).offset(j as isize)).cc).label,
                max_length.wrapping_sub(size) as size_t,
            );

            size = strlen(tmp.as_mut_ptr()) as uint32_t;

            if size == max_length {
                break;
            }

            strncpy(
                tmp.as_mut_ptr().offset(size as isize),
                b",\0" as *const u8 as *const c_char,
                max_length.wrapping_sub(size) as size_t,
            );

            size = strlen(tmp.as_mut_ptr()) as uint32_t;

            if size == max_length {
                break;
            }
        } else {
            strncpy(
                tmp.as_mut_ptr().offset(size as isize),
                (*(**(&raw mut (*rs).element as *mut retainer).offset(j as isize)).cc).label,
                max_length.wrapping_sub(size) as size_t,
            );
        }

        j = j.wrapping_add(1);
    }

    fputs(tmp.as_mut_ptr(), f);
    traceHeapProfSampleString(0 as StgWord8, tmp.as_mut_ptr(), total_size as StgWord);
}

unsafe fn outputAllRetainerSet(mut prof_file: *mut FILE) {
    let mut i: uint32_t = 0;
    let mut j: uint32_t = 0;
    let mut numSet: uint32_t = 0;
    let mut rs = null_mut::<RetainerSet>();
    let mut rsArray = null_mut::<*mut RetainerSet>();
    let mut tmp = null_mut::<RetainerSet>();
    numSet = 0 as uint32_t;
    i = 0 as uint32_t;

    while i < HASH_TABLE_SIZE as uint32_t {
        rs = hashTable[i as usize];

        while !rs.is_null() {
            if (*rs).id < 0 as c_int {
                numSet = numSet.wrapping_add(1);
            }

            rs = (*rs).link as *mut RetainerSet;
        }

        i = i.wrapping_add(1);
    }

    if numSet == 0 as uint32_t {
        return;
    }

    rsArray = stgMallocBytes(
        (numSet as size_t).wrapping_mul(size_of::<*mut RetainerSet>() as size_t),
        b"outputAllRetainerSet()\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut *mut RetainerSet;

    j = 0 as uint32_t;
    i = 0 as uint32_t;

    while i < HASH_TABLE_SIZE as uint32_t {
        rs = hashTable[i as usize];

        while !rs.is_null() {
            if (*rs).id < 0 as c_int {
                let ref mut fresh11 = *rsArray.offset(j as isize);
                *fresh11 = rs;
                j = j.wrapping_add(1);
            }

            rs = (*rs).link as *mut RetainerSet;
        }

        i = i.wrapping_add(1);
    }

    i = numSet.wrapping_sub(1 as uint32_t);

    while i > 0 as uint32_t {
        j = 0 as uint32_t;

        while j <= i.wrapping_sub(1 as uint32_t) {
            if (**rsArray.offset(j as isize)).id
                < (**rsArray.offset(j.wrapping_add(1 as uint32_t) as isize)).id
            {
                tmp = *rsArray.offset(j as isize);

                let ref mut fresh12 = *rsArray.offset(j as isize);
                *fresh12 = *rsArray.offset(j.wrapping_add(1 as uint32_t) as isize);

                let ref mut fresh13 = *rsArray.offset(j.wrapping_add(1 as uint32_t) as isize);
                *fresh13 = tmp;
            }

            j = j.wrapping_add(1);
        }

        i = i.wrapping_sub(1);
    }

    fprintf(
        prof_file,
        b"\nRetainer sets created during profiling:\n\0" as *const u8 as *const c_char,
    );

    i = 0 as uint32_t;

    while i < numSet {
        fprintf(
            prof_file,
            b"SET %u = {\0" as *const u8 as *const c_char,
            -(**rsArray.offset(i as isize)).id,
        );

        j = 0 as uint32_t;

        while j
            < (**rsArray.offset(i as isize))
                .num
                .wrapping_sub(1 as uint32_t)
        {
            printRetainer(
                prof_file,
                *(&raw mut (**rsArray.offset(i as isize)).element as *mut retainer)
                    .offset(j as isize),
            );

            fprintf(prof_file, b", \0" as *const u8 as *const c_char);
            j = j.wrapping_add(1);
        }

        printRetainer(
            prof_file,
            *(&raw mut (**rsArray.offset(i as isize)).element as *mut retainer).offset(j as isize),
        );

        fprintf(prof_file, b"}\n\0" as *const u8 as *const c_char);
        i = i.wrapping_add(1);
    }

    stgFree(rsArray as *mut c_void);
}
