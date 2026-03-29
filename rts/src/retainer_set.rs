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
    pub(crate) num: u32,
    pub(crate) hashKey: StgWord,
    pub(crate) link: *mut _RetainerSet,
    pub(crate) id: i32,
    pub(crate) element: [retainer; 0],
}

pub(crate) type RetainerSet = _RetainerSet;

pub(crate) const BINARY_SEARCH_THRESHOLD: i32 = 8;

#[inline]
pub(crate) unsafe fn isMember(mut r: retainer, mut rs: *mut RetainerSet) -> bool {
    let mut i: i32 = 0;
    let mut left: i32 = 0;
    let mut right: i32 = 0;
    let mut ri = null_mut::<CostCentreStack>();

    if rs == &raw mut rs_MANY {
        return true;
    }

    if (*rs).num < BINARY_SEARCH_THRESHOLD as u32 {
        i = 0;

        while i < (*rs).num as i32 {
            ri = *(&raw mut (*rs).element as *mut retainer).offset(i as isize);

            if r == ri {
                return true;
            } else if r < ri {
                return false;
            }

            i += 1;
        }
    } else {
        left = 0;
        right = (*rs).num.wrapping_sub(1 as u32) as i32;

        while left <= right {
            i = (left + right) / 2;
            ri = *(&raw mut (*rs).element as *mut retainer).offset(i as isize);

            if r == ri {
                return true;
            } else if r < ri {
                right = i - 1;
            } else {
                left = i + 1;
            }
        }
    }

    return false;
}

const HASH_TABLE_SIZE: i32 = 255;

static mut hashTable: [*mut RetainerSet; 255] = [null_mut::<RetainerSet>(); 255];

static mut arena: *mut Arena = null_mut::<Arena>();

static mut nextId: i32 = 0;

static mut rs_MANY: RetainerSet = _RetainerSet {
    num: 0,
    hashKey: 0,
    link: null_mut::<_RetainerSet>(),
    id: 1,
    element: [null_mut::<CostCentreStack>(); 0],
};

#[inline]
unsafe fn sizeofRetainerSet(mut elems: i32) -> usize {
    return (size_of::<RetainerSet>() as usize)
        .wrapping_add((elems as usize).wrapping_mul(size_of::<retainer>() as usize));
}

unsafe fn initializeAllRetainerSet() {
    let mut i: i32 = 0;
    arena = newArena();
    i = 0;

    while i < HASH_TABLE_SIZE {
        hashTable[i as usize] = null_mut::<RetainerSet>();
        i += 1;
    }

    nextId = 2;
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
        if (*rs).num == 1 && *(&raw mut (*rs).element as *mut retainer).offset(0) == r {
            return rs;
        }

        rs = (*rs).link as *mut RetainerSet;
    }

    rs = arenaAlloc(arena, sizeofRetainerSet(1)) as *mut RetainerSet;
    (*rs).num = 1;
    (*rs).hashKey = hk;
    (*rs).link =
        hashTable[hk.wrapping_rem(HASH_TABLE_SIZE as StgWord) as usize] as *mut _RetainerSet;

    let fresh5 = nextId;
    nextId = nextId + 1;
    (*rs).id = fresh5;

    let ref mut fresh6 = *(&raw mut (*rs).element as *mut retainer).offset(0);
    *fresh6 = r;
    hashTable[hk.wrapping_rem(HASH_TABLE_SIZE as StgWord) as usize] = rs;

    return rs;
}

unsafe fn addElement(mut r: retainer, mut rs: *mut RetainerSet) -> *mut RetainerSet {
    let mut i: u32 = 0;
    let mut nl: u32 = 0;
    let mut nrs = null_mut::<RetainerSet>();
    let mut hk: StgWord = 0;

    if rs == &raw mut rs_MANY || (*rs).num == RtsFlags.ProfFlags.maxRetainerSetSize {
        return &raw mut rs_MANY;
    }

    nl = 0;

    while nl < (*rs).num {
        if r < *(&raw mut (*rs).element as *mut retainer).offset(nl as isize) {
            break;
        }

        nl = nl.wrapping_add(1);
    }

    hk = (r as StgWord).wrapping_add((*rs).hashKey);
    nrs = hashTable[hk.wrapping_rem(HASH_TABLE_SIZE as StgWord) as usize];

    while !nrs.is_null() {
        if !((*rs).num.wrapping_add(1 as u32) != (*nrs).num) {
            i = 0;

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
                                .offset(i.wrapping_add(1 as u32) as isize)
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
        sizeofRetainerSet((*rs).num.wrapping_add(1 as u32) as i32),
    ) as *mut RetainerSet;
    (*nrs).num = (*rs).num.wrapping_add(1 as u32);
    (*nrs).hashKey = hk;
    (*nrs).link =
        hashTable[hk.wrapping_rem(HASH_TABLE_SIZE as StgWord) as usize] as *mut _RetainerSet;

    let fresh7 = nextId;
    nextId = nextId + 1;
    (*nrs).id = fresh7;
    i = 0;

    while i < nl {
        let ref mut fresh8 = *(&raw mut (*nrs).element as *mut retainer).offset(i as isize);
        *fresh8 = *(&raw mut (*rs).element as *mut retainer).offset(i as isize);
        i = i.wrapping_add(1);
    }

    let ref mut fresh9 = *(&raw mut (*nrs).element as *mut retainer).offset(i as isize);
    *fresh9 = r;

    while i < (*rs).num {
        let ref mut fresh10 =
            *(&raw mut (*nrs).element as *mut retainer).offset(i.wrapping_add(1 as u32) as isize);
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
    mut max_length: u32,
) {
    let vla = max_length.wrapping_add(1 as u32) as usize;
    let mut tmp: Vec<c_char> = ::std::vec::from_elem(0, vla);
    let mut size: u32 = 0;
    let mut j: u32 = 0;
    *tmp.as_mut_ptr().offset(max_length as isize) = '\0' as i32 as c_char;
    sprintf(tmp.as_mut_ptr().offset(0), c"(%d)".as_ptr(), -(*rs).id);
    size = strlen(tmp.as_mut_ptr()) as u32;
    j = 0;

    while j < (*rs).num {
        if j < (*rs).num.wrapping_sub(1 as u32) {
            strncpy(
                tmp.as_mut_ptr().offset(size as isize),
                (*(**(&raw mut (*rs).element as *mut retainer).offset(j as isize)).cc).label,
                max_length.wrapping_sub(size) as usize,
            );

            size = strlen(tmp.as_mut_ptr()) as u32;

            if size == max_length {
                break;
            }

            strncpy(
                tmp.as_mut_ptr().offset(size as isize),
                c",".as_ptr(),
                max_length.wrapping_sub(size) as usize,
            );

            size = strlen(tmp.as_mut_ptr()) as u32;

            if size == max_length {
                break;
            }
        } else {
            strncpy(
                tmp.as_mut_ptr().offset(size as isize),
                (*(**(&raw mut (*rs).element as *mut retainer).offset(j as isize)).cc).label,
                max_length.wrapping_sub(size) as usize,
            );
        }

        j = j.wrapping_add(1);
    }

    fputs(tmp.as_mut_ptr(), f);
    traceHeapProfSampleString(0, tmp.as_mut_ptr(), total_size as StgWord);
}

unsafe fn outputAllRetainerSet(mut prof_file: *mut FILE) {
    let mut i: u32 = 0;
    let mut j: u32 = 0;
    let mut numSet: u32 = 0;
    let mut rs = null_mut::<RetainerSet>();
    let mut rsArray = null_mut::<*mut RetainerSet>();
    let mut tmp = null_mut::<RetainerSet>();
    numSet = 0;
    i = 0;

    while i < HASH_TABLE_SIZE as u32 {
        rs = hashTable[i as usize];

        while !rs.is_null() {
            if (*rs).id < 0 {
                numSet = numSet.wrapping_add(1);
            }

            rs = (*rs).link as *mut RetainerSet;
        }

        i = i.wrapping_add(1);
    }

    if numSet == 0 {
        return;
    }

    rsArray = stgMallocBytes(
        (numSet as usize).wrapping_mul(size_of::<*mut RetainerSet>() as usize),
        c"outputAllRetainerSet()".as_ptr(),
    ) as *mut *mut RetainerSet;

    j = 0;
    i = 0;

    while i < HASH_TABLE_SIZE as u32 {
        rs = hashTable[i as usize];

        while !rs.is_null() {
            if (*rs).id < 0 {
                let ref mut fresh11 = *rsArray.offset(j as isize);
                *fresh11 = rs;
                j = j.wrapping_add(1);
            }

            rs = (*rs).link as *mut RetainerSet;
        }

        i = i.wrapping_add(1);
    }

    i = numSet.wrapping_sub(1 as u32);

    while i > 0 {
        j = 0;

        while j <= i.wrapping_sub(1 as u32) {
            if (**rsArray.offset(j as isize)).id
                < (**rsArray.offset(j.wrapping_add(1 as u32) as isize)).id
            {
                tmp = *rsArray.offset(j as isize);

                let ref mut fresh12 = *rsArray.offset(j as isize);
                *fresh12 = *rsArray.offset(j.wrapping_add(1 as u32) as isize);

                let ref mut fresh13 = *rsArray.offset(j.wrapping_add(1 as u32) as isize);
                *fresh13 = tmp;
            }

            j = j.wrapping_add(1);
        }

        i = i.wrapping_sub(1);
    }

    fprintf(
        prof_file,
        c"\nRetainer sets created during profiling:\n".as_ptr(),
    );
    i = 0;

    while i < numSet {
        fprintf(
            prof_file,
            c"SET %u = {".as_ptr(),
            -(**rsArray.offset(i as isize)).id,
        );
        j = 0;

        while j < (**rsArray.offset(i as isize)).num.wrapping_sub(1 as u32) {
            printRetainer(
                prof_file,
                *(&raw mut (**rsArray.offset(i as isize)).element as *mut retainer)
                    .offset(j as isize),
            );

            fprintf(prof_file, c", ".as_ptr());
            j = j.wrapping_add(1);
        }

        printRetainer(
            prof_file,
            *(&raw mut (**rsArray.offset(i as isize)).element as *mut retainer).offset(j as isize),
        );

        fprintf(prof_file, c"}\n".as_ptr());
        i = i.wrapping_add(1);
    }

    stgFree(rsArray as *mut c_void);
}
