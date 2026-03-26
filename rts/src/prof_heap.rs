use crate::arena::{Arena, arenaAlloc, arenaFree, newArena};
use crate::capability::getCapability;
use crate::ffi::hs_ffi::HsBool;
use crate::ffi::rts::constants::{
    LDV_CREATE_MASK, LDV_LAST_MASK, LDV_SHIFT, LDV_STATE_CREATE, LDV_STATE_MASK,
};
use crate::ffi::rts::flags::{HEAP_BY_LDV, RtsFlags, rts_argc, rts_argv};
use crate::ffi::rts::messages::{barf, debugBelch, errorBelch, sysErrorBelch};
use crate::ffi::rts::prof::ccs::{CCS_MAIN, CostCentreStack};
use crate::ffi::rts::storage::block::{BF_LARGE, BF_PINNED, bdescr};
use crate::ffi::rts::storage::closure_macros::{
    BLACKHOLE_sizeW, ap_sizeW, ap_stack_sizeW, arr_words_sizeW, bco_sizeW,
    compact_nfdata_full_sizeW, continuation_sizeW, doingErasProfiling, doingLDVProfiling,
    doingRetainerProfiling, get_itbl, itbl_to_con_itbl, mut_arr_ptrs_sizeW, pap_sizeW,
    sizeW_fromITBL, small_mut_arr_ptrs_sizeW, stack_sizeW, thunk_sizeW_fromITBL,
};
use crate::ffi::rts::storage::closure_types::ARR_WORDS;
use crate::ffi::rts::storage::closures::{
    StgAP, StgAP_STACK, StgArrBytes, StgBCO, StgCompactNFData, StgCompactNFDataBlock,
    StgContinuation, StgMutArrPtrs, StgPAP, StgSmallMutArrPtrs,
};
use crate::ffi::rts::storage::gc::generations;
use crate::ffi::rts::storage::tso::StgStack;
use crate::ffi::rts::threads::getNumCapabilities;
use crate::ffi::rts::time::{TIME_RESOLUTION, Time};
use crate::ffi::rts::types::StgClosure;
use crate::ffi::rts::{prog_argc, prog_argv, prog_name, stg_exit};
use crate::ffi::rts_api::{_RTSStats, GCDetails_, RtsOptsAll, getRTSStats};
use crate::ffi::stg::W_;
use crate::ffi::stg::smp::atomic_inc;
use crate::ffi::stg::types::StgWord64;
use crate::ffi::stg::types::{
    StgDouble, StgHalfWord, StgInt, StgPtr, StgVolatilePtr, StgWord, StgWord8, StgWord64,
};
use crate::fs::__rts_fopen;
use crate::hash::HashTable;
use crate::hash::{HashTable, allocHashTable, freeHashTable, insertHashTable, lookupHashTable};
use crate::ldv_profile::LdvCensusKillAll;
use crate::prelude::*;
use crate::printer::closure_type_names;
use crate::retainer_profile::{
    endRetainerProfiling, initRetainerProfiling, isRetainerSetValid, retainerProfile, retainerSetOf,
};
use crate::retainer_set::{RetainerSet, printRetainerSetShort, retainer, rs_MANY};
use crate::rts_flags::rtsConfig;
use crate::rts_utils::{stgFree, stgMallocBytes, stgReallocBytes, time_str};
use crate::sm::gc_thread::{gc_threads, gen_workspace};
use crate::sm::non_moving::{
    NonmovingSegment, nonmoving_alloca_cnt, nonmoving_block_idx, nonmovingClosureMarkedThisCycle,
    nonmovingHeap, nonmovingSegmentBlockCount, nonmovingSegmentBlockSize, nonmovingSegmentGetBlock,
};
use crate::sm::non_moving_mark::{nonmoving_compact_objects, nonmoving_large_objects};
use crate::stats::{stat_endHeapCensus, stat_getElapsedTime, stat_startHeapCensus};
use crate::trace::{
    traceHeapBioProfSampleBegin, traceHeapProfBegin, traceHeapProfSampleBegin,
    traceHeapProfSampleCostCentre, traceHeapProfSampleEnd, traceHeapProfSampleString,
};

#[cfg(test)]
mod tests;

/// cbindgen:no-export
pub(crate) struct C2RustUnnamed_11 {
    pub(crate) prim: ssize_t,
    pub(crate) not_used: ssize_t,
    pub(crate) used: ssize_t,
    pub(crate) void_total: ssize_t,
    pub(crate) drag_total: ssize_t,
}

pub(crate) union C2RustUnnamed_12 {
    pub(crate) resid: ssize_t,
    pub(crate) ldv: C2RustUnnamed_11,
}

pub(crate) type counter = _counter;

/// cbindgen:no-export
pub(crate) struct _counter {
    pub(crate) identity: *const c_void,
    pub(crate) c: C2RustUnnamed_12,
    pub(crate) next: *mut _counter,
}

/// cbindgen:no-export
pub(crate) struct Census {
    pub(crate) time: c_double,
    pub(crate) rtime: StgWord64,
    pub(crate) hash: *mut HashTable,
    pub(crate) ctrs: *mut counter,
    pub(crate) arena: *mut Arena,
    pub(crate) prim: ssize_t,
    pub(crate) not_used: ssize_t,
    pub(crate) used: ssize_t,
    pub(crate) void_total: ssize_t,
    pub(crate) drag_total: ssize_t,
}

static mut hp_file: *mut FILE = null::<FILE>() as *mut FILE;

static mut hp_filename: *mut c_char = null::<c_char>() as *mut c_char;

static mut prof_locale: locale_t = null::<_xlocale>() as *mut _xlocale;

static mut saved_locale: locale_t = null::<_xlocale>() as *mut _xlocale;

#[inline]
unsafe fn init_prof_locale() {
    if prof_locale.is_null() {
        prof_locale = newlocale(
            LC_NUMERIC_MASK,
            b"POSIX\0" as *const u8 as *const c_char,
            null_mut::<_xlocale>(),
        );

        if prof_locale.is_null() {
            sysErrorBelch(
                b"Couldn't allocate heap profiler locale\0" as *const u8 as *const c_char,
            );
        }
    }
}

#[inline]
unsafe fn free_prof_locale() {
    if !prof_locale.is_null() {
        freelocale(prof_locale);
        prof_locale = null_mut::<_xlocale>();
    }
}

#[inline]
unsafe fn set_prof_locale() {
    saved_locale = uselocale(prof_locale);
}

#[inline]
unsafe fn restore_locale() {
    uselocale(saved_locale);
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut era: c_uint = 0;

static mut max_era: uint32_t = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut user_era: StgWord = 0;

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn setUserEra(mut w: StgWord) {
    user_era = w;
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getUserEra() -> StgWord {
    return user_era;
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn incrementUserEra(mut w: StgWord) -> StgWord {
    return atomic_inc(&raw mut user_era as StgVolatilePtr, w);
}

#[inline]
unsafe fn initLDVCtr(mut ctr: *mut counter) {
    (*ctr).c.ldv.prim = 0 as ssize_t;
    (*ctr).c.ldv.not_used = 0 as ssize_t;
    (*ctr).c.ldv.used = 0 as ssize_t;
    (*ctr).c.ldv.void_total = 0 as ssize_t;
    (*ctr).c.ldv.drag_total = 0 as ssize_t;
}

static mut censuses: *mut Census = null::<Census>() as *mut Census;

static mut n_censuses: uint32_t = 0 as uint32_t;

unsafe fn closureIdentity(mut p: *const StgClosure) -> *const c_void {
    match RtsFlags.ProfFlags.doHeapProfile {
        1 => return (*p).header.prof.ccs as *const c_void,
        2 => return (*(*(*p).header.prof.ccs).cc).module as *const c_void,
        4 => {
            return (get_itbl(p).offset(1 as c_int as isize) as StgWord)
                .wrapping_add((*get_itbl(p)).prof.closure_desc_off as StgWord)
                as *mut c_char as *const c_void;
        }
        10 => return (*p).header.prof.hp.era as *mut c_void,
        5 => {
            return (get_itbl(p).offset(1 as c_int as isize) as StgWord)
                .wrapping_add((*get_itbl(p)).prof.closure_type_off as StgWord)
                as *mut c_char as *const c_void;
        }
        6 => {
            if isRetainerSetValid(p) {
                return retainerSetOf(p) as *const c_void;
            } else {
                return null::<c_void>();
            }
        }
        8 => {
            let mut info = null::<StgInfoTable>();
            info = get_itbl(p);

            match (*info).r#type {
                1 | 2 | 3 | 4 | 5 | 6 | 7 => {
                    return (itbl_to_con_itbl(info).offset(1 as c_int as isize) as StgWord)
                        .wrapping_add((*itbl_to_con_itbl(info)).con_desc as StgWord)
                        as *const c_char as *const c_void;
                }
                _ => {
                    return *(&raw mut closure_type_names as *mut *const c_char)
                        .offset((*info).r#type as isize)
                        as *const c_void;
                }
            }
        }
        9 => return get_itbl(p) as *const c_void,
        _ => {
            barf(b"closureIdentity\0" as *const u8 as *const c_char);
        }
    };
}

unsafe fn LDV_recordDead(mut c: *const StgClosure, mut size: uint32_t) {
    let mut id = null::<c_void>();
    let mut t: uint32_t = 0;
    let mut ctr = null_mut::<counter>();

    if era > 0 as c_uint && closureSatisfiesConstraints(c) as c_int != 0 {
        size = (size as c_ulong).wrapping_sub(
            (size_of::<StgProfHeader>() as usize)
                .wrapping_add(size_of::<W_>() as usize)
                .wrapping_sub(1 as usize)
                .wrapping_div(size_of::<W_>() as usize) as c_ulong,
        ) as uint32_t as uint32_t;

        if (*(c as *mut StgClosure)).header.prof.hp.ldvw & LDV_STATE_MASK as StgWord
            == LDV_STATE_CREATE as StgWord
        {
            t = (((*(c as *mut StgClosure)).header.prof.hp.ldvw & LDV_CREATE_MASK as StgWord)
                >> LDV_SHIFT) as uint32_t;

            if t < era as uint32_t {
                if RtsFlags.ProfFlags.bioSelector.is_null() {
                    (*censuses.offset(t as isize)).void_total += size as ssize_t;
                    (*censuses.offset(era as isize)).void_total -= size as ssize_t;
                } else {
                    id = closureIdentity(c);
                    ctr = lookupHashTable((*censuses.offset(t as isize)).hash, id as StgWord)
                        as *mut counter;

                    if ctr.is_null() {
                        barf(
                            b"LDV_recordDead: Failed to find counter for closure %p\0" as *const u8
                                as *const c_char,
                            c,
                        );
                    }

                    (*ctr).c.ldv.void_total += size as ssize_t;
                    ctr = lookupHashTable((*censuses.offset(era as isize)).hash, id as StgWord)
                        as *mut counter;

                    if ctr.is_null() {
                        ctr = arenaAlloc(
                            (*censuses.offset(era as isize)).arena,
                            size_of::<counter>() as size_t,
                        ) as *mut counter;

                        initLDVCtr(ctr);

                        insertHashTable(
                            (*censuses.offset(era as isize)).hash,
                            id as StgWord,
                            ctr as *const c_void,
                        );

                        (*ctr).identity = id;
                        (*ctr).next = (*censuses.offset(era as isize)).ctrs as *mut _counter;

                        let ref mut fresh5 = (*censuses.offset(era as isize)).ctrs;
                        *fresh5 = ctr;
                    }

                    (*ctr).c.ldv.void_total -= size as ssize_t;
                }
            }
        } else {
            t = ((*(c as *mut StgClosure)).header.prof.hp.ldvw & LDV_LAST_MASK as StgWord)
                as uint32_t;

            if t.wrapping_add(1 as uint32_t) < era as uint32_t {
                if RtsFlags.ProfFlags.bioSelector.is_null() {
                    (*censuses.offset(t.wrapping_add(1 as uint32_t) as isize)).drag_total +=
                        size as ssize_t;
                    (*censuses.offset(era as isize)).drag_total -= size as ssize_t;
                } else {
                    let mut id_0 = null::<c_void>();
                    id_0 = closureIdentity(c);

                    ctr = lookupHashTable(
                        (*censuses.offset(t.wrapping_add(1 as uint32_t) as isize)).hash,
                        id_0 as StgWord,
                    ) as *mut counter;

                    (*ctr).c.ldv.drag_total += size as ssize_t;
                    ctr = lookupHashTable((*censuses.offset(era as isize)).hash, id_0 as StgWord)
                        as *mut counter;

                    if ctr.is_null() {
                        ctr = arenaAlloc(
                            (*censuses.offset(era as isize)).arena,
                            size_of::<counter>() as size_t,
                        ) as *mut counter;

                        initLDVCtr(ctr);

                        insertHashTable(
                            (*censuses.offset(era as isize)).hash,
                            id_0 as StgWord,
                            ctr as *const c_void,
                        );

                        (*ctr).identity = id_0;
                        (*ctr).next = (*censuses.offset(era as isize)).ctrs as *mut _counter;

                        let ref mut fresh6 = (*censuses.offset(era as isize)).ctrs;
                        *fresh6 = ctr;
                    }

                    (*ctr).c.ldv.drag_total -= size as ssize_t;
                }
            }
        }
    }
}

#[inline]
unsafe fn initEra(mut census: *mut Census) {
    if !(*census).hash.is_null() {
        freeHashTable((*census).hash, None);
    }

    if !(*census).arena.is_null() {
        arenaFree((*census).arena);
    }

    (*census).hash = allocHashTable();
    (*census).ctrs = null_mut::<counter>();
    (*census).arena = newArena();
    (*census).not_used = 0 as ssize_t;
    (*census).used = 0 as ssize_t;
    (*census).prim = 0 as ssize_t;
    (*census).void_total = 0 as ssize_t;
    (*census).drag_total = 0 as ssize_t;
}

#[inline]
unsafe fn freeEra(mut census: *mut Census) {
    arenaFree((*census).arena);
    freeHashTable((*census).hash, None);
}

unsafe fn nextEra() {
    if user_era > 0 as StgWord && RtsFlags.ProfFlags.incrementUserEra as c_int != 0 {
        user_era = user_era.wrapping_add(1);
    }

    if doingLDVProfiling() {
        era = era.wrapping_add(1);

        if era as uint32_t == max_era {
            errorBelch(b"Maximum number of censuses reached.\0" as *const u8 as *const c_char);

            if rtsConfig.rts_opts_suggestions == r#true as HsBool {
                if rtsConfig.rts_opts_enabled as c_uint == RtsOptsAll as c_int as c_uint {
                    errorBelch(
                        b"Use `+RTS -i' to reduce censuses.\0" as *const u8 as *const c_char,
                    );
                } else {
                    errorBelch(
                        b"Relink with -rtsopts and use `+RTS -i' to reduce censuses.\0" as *const u8
                            as *const c_char,
                    );
                }
            }

            stg_exit(EXIT_FAILURE);
        }

        if era as uint32_t == n_censuses {
            n_censuses = n_censuses.wrapping_mul(2 as uint32_t);

            censuses = stgReallocBytes(
                censuses as *mut c_void,
                (size_of::<Census>() as size_t).wrapping_mul(n_censuses as size_t),
                b"nextEra\0" as *const u8 as *const c_char as *mut c_char,
            ) as *mut Census;

            memset(
                censuses.offset(era as isize) as *mut Census as *mut c_void,
                0 as c_int,
                (size_of::<Census>() as size_t)
                    .wrapping_mul(n_censuses as size_t)
                    .wrapping_div(2 as size_t),
            );
        }
    }

    initEra(censuses.offset(era as isize) as *mut Census);
}

unsafe fn printEscapedString(mut string: *const c_char) {
    let mut p = string;

    while *p as c_int != '\0' as i32 {
        if *p as c_int == '"' as i32 {
            fputc('"' as i32, hp_file);
        }

        fputc(*p as c_int, hp_file);
        p = p.offset(1);
    }
}

unsafe fn printSample(mut beginSample: bool, mut sampleValue: StgDouble) {
    fprintf(
        hp_file,
        b"%s %f\n\0" as *const u8 as *const c_char,
        if beginSample as c_int != 0 {
            b"BEGIN_SAMPLE\0" as *const u8 as *const c_char
        } else {
            b"END_SAMPLE\0" as *const u8 as *const c_char
        },
        sampleValue,
    );

    if !beginSample {
        fflush(hp_file);
    }
}

unsafe fn freeHeapProfiling() {
    free_prof_locale();
}

unsafe fn initHeapProfiling() {
    if RtsFlags.ProfFlags.doHeapProfile == 0 {
        return;
    }

    init_prof_locale();
    set_prof_locale();

    let mut stem = null_mut::<c_char>();

    if !RtsFlags.CcFlags.outputFileNameStem.is_null() {
        stem = stgMallocBytes(
            strlen(RtsFlags.CcFlags.outputFileNameStem).wrapping_add(1 as size_t),
            b"initHeapProfiling\0" as *const u8 as *const c_char as *mut c_char,
        ) as *mut c_char;

        strcpy(stem, RtsFlags.CcFlags.outputFileNameStem);
    } else {
        stem = stgMallocBytes(
            strlen(prog_name).wrapping_add(1 as size_t),
            b"initHeapProfiling\0" as *const u8 as *const c_char as *mut c_char,
        ) as *mut c_char;

        strcpy(stem, prog_name);
    }

    if RtsFlags.ProfFlags.doHeapProfile != 0 {
        hp_filename = stgMallocBytes(
            strlen(stem).wrapping_add(6 as size_t),
            b"hpFileName\0" as *const u8 as *const c_char as *mut c_char,
        ) as *mut c_char;

        sprintf(hp_filename, b"%s.hp\0" as *const u8 as *const c_char, stem);
        hp_file = __rts_fopen(hp_filename, b"w+\0" as *const u8 as *const c_char);

        if hp_file.is_null() {
            debugBelch(
                b"Can't open profiling report file %s\n\0" as *const u8 as *const c_char,
                hp_filename,
            );

            RtsFlags.ProfFlags.doHeapProfile = 0 as uint32_t;
            stgFree(stem as *mut c_void);
            return;
        }
    }

    stgFree(stem as *mut c_void);

    if doingLDVProfiling() as c_int != 0 && doingRetainerProfiling() as c_int != 0 {
        errorBelch(b"cannot mix -hb and -hr\0" as *const u8 as *const c_char);
        stg_exit(EXIT_FAILURE);
    }

    if doingErasProfiling() {
        user_era = 1 as StgWord;
    }

    if doingLDVProfiling() {
        era = 1 as c_uint;
        n_censuses = 32 as uint32_t;
    } else {
        era = 0 as c_uint;
        n_censuses = 1 as uint32_t;
    }

    max_era = ((1 as c_int) << LDV_SHIFT) as uint32_t;

    censuses = stgMallocBytes(
        (size_of::<Census>() as size_t).wrapping_mul(n_censuses as size_t),
        b"initHeapProfiling\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut Census;

    let mut i = 0 as c_uint;

    while (i as uint32_t) < n_censuses {
        let ref mut fresh8 = (*censuses.offset(i as isize)).arena;
        *fresh8 = null_mut::<Arena>();

        let ref mut fresh9 = (*censuses.offset(i as isize)).hash;
        *fresh9 = null_mut::<HashTable>();
        i = i.wrapping_add(1);
    }

    initEra(censuses.offset(era as isize) as *mut Census);
    fprintf(hp_file, b"JOB \"\0" as *const u8 as *const c_char);
    printEscapedString(prog_name);

    let mut i_0 = 1 as c_int;

    while i_0 < prog_argc {
        fputc(' ' as i32, hp_file);
        printEscapedString(*prog_argv.offset(i_0 as isize));
        i_0 += 1;
    }

    fprintf(hp_file, b" +RTS\0" as *const u8 as *const c_char);

    let mut i_1 = 0 as c_int;

    while i_1 < rts_argc {
        fputc(' ' as i32, hp_file);
        printEscapedString(*rts_argv.offset(i_1 as isize));
        i_1 += 1;
    }

    fprintf(hp_file, b"\"\n\0" as *const u8 as *const c_char);

    fprintf(
        hp_file,
        b"DATE \"%s\"\n\0" as *const u8 as *const c_char,
        time_str(),
    );

    fprintf(
        hp_file,
        b"SAMPLE_UNIT \"seconds\"\n\0" as *const u8 as *const c_char,
    );

    fprintf(
        hp_file,
        b"VALUE_UNIT \"bytes\"\n\0" as *const u8 as *const c_char,
    );

    printSample(r#true != 0, 0 as c_int as StgDouble);
    printSample(r#false != 0, 0 as c_int as StgDouble);

    if doingRetainerProfiling() {
        initRetainerProfiling();
    }

    restore_locale();
    traceHeapProfBegin(0 as StgWord8);
}

unsafe fn endHeapProfiling() {
    if RtsFlags.ProfFlags.doHeapProfile == 0 {
        return;
    }

    set_prof_locale();

    if doingRetainerProfiling() {
        endRetainerProfiling();
    } else if doingLDVProfiling() {
        let mut t: uint32_t = 0;
        LdvCensusKillAll();
        aggregateCensusInfo();
        t = 1 as uint32_t;

        while t < era as uint32_t {
            dumpCensus(censuses.offset(t as isize) as *mut Census);
            t = t.wrapping_add(1);
        }

        if !RtsFlags.ProfFlags.bioSelector.is_null() {
            t = 1 as uint32_t;

            while t <= era as uint32_t {
                freeEra(censuses.offset(t as isize) as *mut Census);
                t = t.wrapping_add(1);
            }
        } else {
            freeEra(censuses.offset(era as isize) as *mut Census);
        }
    } else {
        freeEra(censuses.offset(0 as c_int as isize) as *mut Census);
    }

    stgFree(censuses as *mut c_void);

    let mut stats = _RTSStats {
        gcs: 0,
        major_gcs: 0,
        allocated_bytes: 0,
        max_live_bytes: 0,
        max_large_objects_bytes: 0,
        max_compact_bytes: 0,
        max_slop_bytes: 0,
        max_mem_in_use_bytes: 0,
        cumulative_live_bytes: 0,
        copied_bytes: 0,
        par_copied_bytes: 0,
        cumulative_par_max_copied_bytes: 0,
        cumulative_par_balanced_copied_bytes: 0,
        init_cpu_ns: 0,
        init_elapsed_ns: 0,
        mutator_cpu_ns: 0,
        mutator_elapsed_ns: 0,
        gc_cpu_ns: 0,
        gc_elapsed_ns: 0,
        cpu_ns: 0,
        elapsed_ns: 0,
        gc: GCDetails_ {
            r#gen: 0,
            threads: 0,
            allocated_bytes: 0,
            live_bytes: 0,
            large_objects_bytes: 0,
            compact_bytes: 0,
            slop_bytes: 0,
            mem_in_use_bytes: 0,
            copied_bytes: 0,
            block_fragmentation_bytes: 0,
            par_max_copied_bytes: 0,
            par_balanced_copied_bytes: 0,
            sync_elapsed_ns: 0,
            cpu_ns: 0,
            elapsed_ns: 0,
            nonmoving_gc_sync_cpu_ns: 0,
            nonmoving_gc_sync_elapsed_ns: 0,
            nonmoving_gc_cpu_ns: 0,
            nonmoving_gc_elapsed_ns: 0,
        },
        any_work: 0,
        scav_find_work: 0,
        max_n_todo_overflow: 0,
        nonmoving_gc_sync_cpu_ns: 0,
        nonmoving_gc_sync_elapsed_ns: 0,
        nonmoving_gc_sync_max_elapsed_ns: 0,
        nonmoving_gc_cpu_ns: 0,
        nonmoving_gc_elapsed_ns: 0,
        nonmoving_gc_max_elapsed_ns: 0,
    };

    getRTSStats(&raw mut stats);

    let mut mut_time: Time = stats.mutator_cpu_ns;
    let mut seconds = mut_time as StgDouble / TIME_RESOLUTION as StgDouble;
    printSample(r#true != 0, seconds);
    printSample(r#false != 0, seconds);
    fclose(hp_file);
    restore_locale();
}

unsafe fn buf_append(mut p: *mut c_char, mut q: *const c_char, mut end: *mut c_char) -> size_t {
    let mut m: c_int = 0;
    m = 0 as c_int;

    while p < end {
        *p = *q;

        if *q as c_int == '\0' as i32 {
            break;
        }

        p = p.offset(1);
        q = q.offset(1);
        m += 1;
    }

    return m as size_t;
}

unsafe fn fprint_ccs(mut fp: *mut FILE, mut ccs: *mut CostCentreStack, mut max_length: uint32_t) {
    let vla = max_length.wrapping_add(1 as uint32_t) as usize;
    let mut buf: Vec<c_char> = ::std::vec::from_elem(0, vla);
    let mut p = null_mut::<c_char>();
    let mut buf_end = null_mut::<c_char>();

    if ccs == &raw mut CCS_MAIN as *mut CostCentreStack {
        fprintf(fp, b"MAIN\0" as *const u8 as *const c_char);
        return;
    }

    fprintf(fp, b"(%lld)\0" as *const u8 as *const c_char, (*ccs).ccsID);
    p = buf.as_mut_ptr();
    buf_end = buf
        .as_mut_ptr()
        .offset(max_length as isize)
        .offset(1 as c_int as isize);

    while !ccs.is_null() && ccs != &raw mut CCS_MAIN as *mut CostCentreStack {
        if strcmp((*(*ccs).cc).label, b"CAF\0" as *const u8 as *const c_char) == 0 {
            p = p.offset(buf_append(p, (*(*ccs).cc).module, buf_end) as isize);
            p = p.offset(buf_append(p, b".CAF\0" as *const u8 as *const c_char, buf_end) as isize);
        } else {
            p = p.offset(buf_append(p, (*(*ccs).cc).label, buf_end) as isize);

            if !(*ccs).prevStack.is_null()
                && (*ccs).prevStack != &raw mut CCS_MAIN as *mut CostCentreStack
            {
                p = p.offset(buf_append(p, b"/\0" as *const u8 as *const c_char, buf_end) as isize);
            }
        }

        if p >= buf_end {
            sprintf(
                buf.as_mut_ptr()
                    .offset(max_length as isize)
                    .offset(-(4 as c_int as isize)),
                b"...\0" as *const u8 as *const c_char,
            );

            break;
        } else {
            ccs = (*ccs).prevStack as *mut CostCentreStack;
        }
    }

    fprintf(fp, b"%s\0" as *const u8 as *const c_char, buf.as_mut_ptr());
}

unsafe fn strMatchesSelector(mut str: *const c_char, mut sel: *const c_char) -> bool {
    let mut p = null::<c_char>();

    loop {
        p = str;

        while *p as c_int != '\0' as i32
            && *sel as c_int != ',' as i32
            && *sel as c_int != '\0' as i32
            && *p as c_int == *sel as c_int
        {
            p = p.offset(1);
            sel = sel.offset(1);
        }

        if *p as c_int == '\0' as i32
            && (*sel as c_int == ',' as i32 || *sel as c_int == '\0' as i32)
        {
            return r#true != 0;
        }

        while *sel as c_int != ',' as i32 && *sel as c_int != '\0' as i32 {
            sel = sel.offset(1);
        }

        if *sel as c_int == ',' as i32 {
            sel = sel.offset(1);
        }

        if *sel as c_int == '\0' as i32 {
            return r#false != 0;
        }
    }
}

unsafe fn closureSatisfiesConstraints(mut p: *const StgClosure) -> bool {
    let mut b: bool = false;

    if (*(*p).header.prof.ccs).selected == 0 {
        return r#false != 0;
    }

    if !RtsFlags.ProfFlags.descrSelector.is_null() {
        b = strMatchesSelector(
            (get_itbl(p as *mut StgClosure).offset(1 as c_int as isize) as StgWord)
                .wrapping_add((*get_itbl(p as *mut StgClosure)).prof.closure_desc_off as StgWord)
                as *mut c_char,
            RtsFlags.ProfFlags.descrSelector,
        );

        if !b {
            return r#false != 0;
        }
    }

    if !RtsFlags.ProfFlags.typeSelector.is_null() {
        b = strMatchesSelector(
            (get_itbl(p as *mut StgClosure).offset(1 as c_int as isize) as StgWord)
                .wrapping_add((*get_itbl(p as *mut StgClosure)).prof.closure_type_off as StgWord)
                as *mut c_char,
            RtsFlags.ProfFlags.typeSelector,
        );

        if !b {
            return r#false != 0;
        }
    }

    if RtsFlags.ProfFlags.eraSelector != 0 {
        return (*p).header.prof.hp.era == RtsFlags.ProfFlags.eraSelector;
    }

    if !RtsFlags.ProfFlags.retainerSelector.is_null() {
        let mut rs = null_mut::<RetainerSet>();
        let mut i: uint32_t = 0;

        if isRetainerSetValid(p as *mut StgClosure) {
            rs = retainerSetOf(p as *mut StgClosure);

            if !rs.is_null() {
                i = 0 as uint32_t;

                while i < (*rs).num {
                    b = strMatchesSelector(
                        (*(**(&raw mut (*rs).element as *mut retainer).offset(i as isize)).cc)
                            .label,
                        RtsFlags.ProfFlags.retainerSelector,
                    );

                    if b {
                        return r#true != 0;
                    }

                    i = i.wrapping_add(1);
                }
            }
        }

        return r#false != 0;
    }

    return r#true != 0;
}

unsafe fn aggregateCensusInfo() {
    let mut acc = null_mut::<HashTable>();
    let mut t: uint32_t = 0;
    let mut c = null_mut::<counter>();
    let mut d = null_mut::<counter>();
    let mut ctrs = null_mut::<counter>();
    let mut arena = null_mut::<Arena>();

    if !doingLDVProfiling() {
        return;
    }

    if RtsFlags.ProfFlags.doHeapProfile == HEAP_BY_LDV as uint32_t {
        let mut void_total: c_long = 0;
        let mut drag_total: c_long = 0;
        void_total = 0 as c_long;
        drag_total = 0 as c_long;
        t = 1 as uint32_t;

        while t < era as uint32_t {
            void_total += (*censuses.offset(t as isize)).void_total as c_long;
            drag_total += (*censuses.offset(t as isize)).drag_total as c_long;
            (*censuses.offset(t as isize)).void_total = void_total as ssize_t;
            (*censuses.offset(t as isize)).drag_total = drag_total as ssize_t;
            t = t.wrapping_add(1);
        }

        return;
    }

    arena = newArena();
    acc = allocHashTable();
    ctrs = null_mut::<counter>();
    t = 1 as uint32_t;

    while t < era as uint32_t {
        c = ctrs;

        while !c.is_null() {
            d = lookupHashTable(
                (*censuses.offset(t as isize)).hash,
                (*c).identity as StgWord,
            ) as *mut counter;

            if !d.is_null() {
                (*d).c.ldv.void_total += (*c).c.ldv.void_total;
                (*d).c.ldv.drag_total += (*c).c.ldv.drag_total;
                (*c).c.ldv.void_total = (*d).c.ldv.void_total;
                (*c).c.ldv.drag_total = (*d).c.ldv.drag_total;
            }

            c = (*c).next as *mut counter;
        }

        c = (*censuses.offset(t as isize)).ctrs;

        while !c.is_null() {
            d = lookupHashTable(acc, (*c).identity as StgWord) as *mut counter;

            if d.is_null() {
                d = arenaAlloc(arena, size_of::<counter>() as size_t) as *mut counter;
                initLDVCtr(d);
                insertHashTable(acc, (*c).identity as StgWord, d as *const c_void);
                (*d).identity = (*c).identity;
                (*d).next = ctrs as *mut _counter;
                ctrs = d;
                (*d).c.ldv.void_total = (*c).c.ldv.void_total;
                (*d).c.ldv.drag_total = (*c).c.ldv.drag_total;
            }

            c = (*c).next as *mut counter;
        }

        t = t.wrapping_add(1);
    }

    freeHashTable(acc, None);
    arenaFree(arena);
}

unsafe fn dumpCensus(mut census: *mut Census) {
    let mut ctr = null_mut::<counter>();
    let mut count: ssize_t = 0;
    set_prof_locale();
    printSample(r#true != 0, (*census).time as StgDouble);

    if RtsFlags.ProfFlags.doHeapProfile == HEAP_BY_LDV as uint32_t {
        traceHeapBioProfSampleBegin(era as StgInt, (*census).rtime);
    } else {
        traceHeapProfSampleBegin(era as StgInt);
    }

    if RtsFlags.ProfFlags.doHeapProfile == HEAP_BY_LDV as uint32_t {
        fprintf(
            hp_file,
            b"VOID\t%llu\n\0" as *const u8 as *const c_char,
            ((*census).void_total as usize).wrapping_mul(size_of::<W_>() as usize) as uint64_t,
        );

        fprintf(
            hp_file,
            b"LAG\t%llu\n\0" as *const u8 as *const c_char,
            (((*census).not_used - (*census).void_total) as usize)
                .wrapping_mul(size_of::<W_>() as usize) as uint64_t,
        );

        fprintf(
            hp_file,
            b"USE\t%llu\n\0" as *const u8 as *const c_char,
            (((*census).used - (*census).drag_total) as usize)
                .wrapping_mul(size_of::<W_>() as usize) as uint64_t,
        );

        fprintf(
            hp_file,
            b"INHERENT_USE\t%llu\n\0" as *const u8 as *const c_char,
            ((*census).prim as usize).wrapping_mul(size_of::<W_>() as usize) as uint64_t,
        );

        fprintf(
            hp_file,
            b"DRAG\t%llu\n\0" as *const u8 as *const c_char,
            ((*census).drag_total as usize).wrapping_mul(size_of::<W_>() as usize) as uint64_t,
        );

        traceHeapProfSampleString(
            0 as StgWord8,
            b"VOID\0" as *const u8 as *const c_char,
            ((*census).void_total as usize).wrapping_mul(size_of::<W_>() as usize) as StgWord,
        );

        traceHeapProfSampleString(
            0 as StgWord8,
            b"LAG\0" as *const u8 as *const c_char,
            (((*census).not_used - (*census).void_total) as usize)
                .wrapping_mul(size_of::<W_>() as usize) as StgWord,
        );

        traceHeapProfSampleString(
            0 as StgWord8,
            b"USE\0" as *const u8 as *const c_char,
            (((*census).used - (*census).drag_total) as usize)
                .wrapping_mul(size_of::<W_>() as usize) as StgWord,
        );

        traceHeapProfSampleString(
            0 as StgWord8,
            b"INHERENT_USE\0" as *const u8 as *const c_char,
            ((*census).prim as usize).wrapping_mul(size_of::<W_>() as usize) as StgWord,
        );

        traceHeapProfSampleString(
            0 as StgWord8,
            b"DRAG\0" as *const u8 as *const c_char,
            ((*census).drag_total as usize).wrapping_mul(size_of::<W_>() as usize) as StgWord,
        );

        traceHeapProfSampleEnd(era as StgInt);
        printSample(r#false != 0, (*census).time as StgDouble);
        return;
    }

    ctr = (*census).ctrs;

    while !ctr.is_null() {
        if !RtsFlags.ProfFlags.bioSelector.is_null() {
            count = 0 as ssize_t;

            if strMatchesSelector(
                b"lag\0" as *const u8 as *const c_char,
                RtsFlags.ProfFlags.bioSelector,
            ) {
                count += (*ctr).c.ldv.not_used - (*ctr).c.ldv.void_total;
            }

            if strMatchesSelector(
                b"drag\0" as *const u8 as *const c_char,
                RtsFlags.ProfFlags.bioSelector,
            ) {
                count += (*ctr).c.ldv.drag_total;
            }

            if strMatchesSelector(
                b"void\0" as *const u8 as *const c_char,
                RtsFlags.ProfFlags.bioSelector,
            ) {
                count += (*ctr).c.ldv.void_total;
            }

            if strMatchesSelector(
                b"use\0" as *const u8 as *const c_char,
                RtsFlags.ProfFlags.bioSelector,
            ) {
                count += (*ctr).c.ldv.used - (*ctr).c.ldv.drag_total;
            }
        } else {
            count = (*ctr).c.resid;
        }

        if !(count == 0 as ssize_t) {
            let mut str: [c_char; 100] = [0; 100];
            let mut str_era: [c_char; 100] = [0; 100];

            match RtsFlags.ProfFlags.doHeapProfile {
                8 => {
                    fprintf(
                        hp_file,
                        b"%s\0" as *const u8 as *const c_char,
                        (*ctr).identity as *mut c_char,
                    );

                    traceHeapProfSampleString(
                        0 as StgWord8,
                        (*ctr).identity as *mut c_char,
                        (count as usize).wrapping_mul(size_of::<W_>() as usize) as StgWord,
                    );
                }
                9 => {
                    fprintf(
                        hp_file,
                        b"%p\0" as *const u8 as *const c_char,
                        (*ctr).identity,
                    );

                    str = [0; 100];

                    sprintf(
                        &raw mut str as *mut c_char,
                        b"%p\0" as *const u8 as *const c_char,
                        (*ctr).identity,
                    );

                    traceHeapProfSampleString(
                        0 as StgWord8,
                        &raw mut str as *mut c_char,
                        (count as usize).wrapping_mul(size_of::<W_>() as usize) as StgWord,
                    );
                }
                1 => {
                    fprint_ccs(
                        hp_file,
                        (*ctr).identity as *mut CostCentreStack,
                        RtsFlags.ProfFlags.ccsLength,
                    );

                    traceHeapProfSampleCostCentre(
                        0 as StgWord8,
                        (*ctr).identity as *mut CostCentreStack,
                        (count as usize).wrapping_mul(size_of::<W_>() as usize) as StgWord,
                    );
                }
                10 => {
                    fprintf(
                        hp_file,
                        b"%llu\0" as *const u8 as *const c_char,
                        (*ctr).identity as StgWord,
                    );

                    str_era = [0; 100];

                    sprintf(
                        &raw mut str_era as *mut c_char,
                        b"%llu\0" as *const u8 as *const c_char,
                        (*ctr).identity as StgWord,
                    );

                    traceHeapProfSampleString(
                        0 as StgWord8,
                        &raw mut str_era as *mut c_char,
                        (count as usize).wrapping_mul(size_of::<W_>() as usize) as StgWord,
                    );
                }
                2 | 4 | 5 => {
                    fprintf(
                        hp_file,
                        b"%s\0" as *const u8 as *const c_char,
                        (*ctr).identity as *mut c_char,
                    );

                    traceHeapProfSampleString(
                        0 as StgWord8,
                        (*ctr).identity as *mut c_char,
                        (count as usize).wrapping_mul(size_of::<W_>() as usize) as StgWord,
                    );
                }
                6 => {
                    let mut rs = (*ctr).identity as *mut RetainerSet;

                    if rs == &raw mut rs_MANY {
                        fprintf(hp_file, b"MANY\0" as *const u8 as *const c_char);
                    } else {
                        if (*rs).id > 0 as c_int {
                            (*rs).id = -(*rs).id;
                        }

                        printRetainerSetShort(
                            hp_file,
                            rs,
                            (count as W_).wrapping_mul(size_of::<W_>() as W_),
                            RtsFlags.ProfFlags.ccsLength,
                        );
                    }
                }
                _ => {
                    barf(b"dumpCensus; doHeapProfile\0" as *const u8 as *const c_char);
                }
            }

            fprintf(
                hp_file,
                b"\t%llu\n\0" as *const u8 as *const c_char,
                (count as W_).wrapping_mul(size_of::<W_>() as W_),
            );
        }

        ctr = (*ctr).next as *mut counter;
    }

    traceHeapProfSampleEnd(era as StgInt);
    printSample(r#false != 0, (*census).time as StgDouble);
    restore_locale();
}

#[inline]
unsafe fn heapInsertNewCounter(mut census: *mut Census, mut identity: StgWord) -> *mut counter {
    let mut ctr = arenaAlloc((*census).arena, size_of::<counter>() as size_t) as *mut counter;
    initLDVCtr(ctr);
    insertHashTable((*census).hash, identity, ctr as *const c_void);
    (*ctr).identity = identity as *mut c_void;
    (*ctr).next = (*census).ctrs as *mut _counter;
    (*census).ctrs = ctr;

    return ctr;
}

unsafe fn heapProfObject(
    mut census: *mut Census,
    mut p: *mut StgClosure,
    mut size: size_t,
    mut prim: bool,
) {
    let mut identity = null::<c_void>();
    let mut real_size: size_t = 0;
    let mut ctr = null_mut::<counter>();
    identity = null::<c_void>();

    real_size = size.wrapping_sub(
        (size_of::<StgProfHeader>() as size_t)
            .wrapping_add(size_of::<W_>() as size_t)
            .wrapping_sub(1 as size_t)
            .wrapping_div(size_of::<W_>() as size_t),
    );

    if closureSatisfiesConstraints(p) {
        if RtsFlags.ProfFlags.doHeapProfile == HEAP_BY_LDV as uint32_t {
            if prim {
                (*census).prim =
                    ((*census).prim as size_t).wrapping_add(real_size) as ssize_t as ssize_t;
            } else if (*p).header.prof.hp.ldvw & LDV_STATE_MASK as StgWord
                == LDV_STATE_CREATE as StgWord
            {
                (*census).not_used =
                    ((*census).not_used as size_t).wrapping_add(real_size) as ssize_t as ssize_t;
            } else {
                (*census).used =
                    ((*census).used as size_t).wrapping_add(real_size) as ssize_t as ssize_t;
            }
        } else {
            identity = closureIdentity(p);

            if !identity.is_null() {
                ctr = lookupHashTable((*census).hash, identity as StgWord) as *mut counter;

                if !ctr.is_null() {
                    if !RtsFlags.ProfFlags.bioSelector.is_null() {
                        if prim {
                            (*ctr).c.ldv.prim =
                                ((*ctr).c.ldv.prim as size_t).wrapping_add(real_size) as ssize_t
                                    as ssize_t;
                        } else if (*p).header.prof.hp.ldvw & LDV_STATE_MASK as StgWord
                            == LDV_STATE_CREATE as StgWord
                        {
                            (*ctr).c.ldv.not_used = ((*ctr).c.ldv.not_used as size_t)
                                .wrapping_add(real_size)
                                as ssize_t
                                as ssize_t;
                        } else {
                            (*ctr).c.ldv.used =
                                ((*ctr).c.ldv.used as size_t).wrapping_add(real_size) as ssize_t
                                    as ssize_t;
                        }
                    } else {
                        (*ctr).c.resid = ((*ctr).c.resid as size_t).wrapping_add(real_size)
                            as ssize_t as ssize_t;
                    }
                } else {
                    ctr = heapInsertNewCounter(census, identity as StgWord);

                    if !RtsFlags.ProfFlags.bioSelector.is_null() {
                        if prim {
                            (*ctr).c.ldv.prim = real_size as ssize_t;
                        } else if (*p).header.prof.hp.ldvw & LDV_STATE_MASK as StgWord
                            == LDV_STATE_CREATE as StgWord
                        {
                            (*ctr).c.ldv.not_used = real_size as ssize_t;
                        } else {
                            (*ctr).c.ldv.used = real_size as ssize_t;
                        }
                    } else {
                        (*ctr).c.resid = real_size as ssize_t;
                    }
                }
            }
        }
    }
}

unsafe fn heapCensusCompactList(mut census: *mut Census, mut bd: *mut bdescr) {
    while !bd.is_null() {
        let mut block = (*bd).start as *mut StgCompactNFDataBlock;
        let mut str = (*block).owner as *mut StgCompactNFData;

        heapProfObject(
            census,
            str as *mut StgClosure,
            compact_nfdata_full_sizeW(str) as size_t,
            r#true != 0,
        );

        bd = (*bd).link as *mut bdescr;
    }
}

unsafe fn heapCensusBlock(mut census: *mut Census, mut bd: *mut bdescr) {
    let mut p = (*bd).start;

    if (*bd).flags as c_int & BF_PINNED != 0 {
        while p < (*bd).c2rust_unnamed.free && *p == 0 {
            p = p.offset(1);
        }
    }

    while p < (*bd).c2rust_unnamed.free {
        let mut info = get_itbl(p as *const StgClosure);
        let mut prim = r#false != 0;
        let mut size: size_t = 0;

        match (*info).r#type {
            15 => {
                size = thunk_sizeW_fromITBL(info) as size_t;
            }
            19 | 20 | 18 => {
                size = (size_of::<StgThunkHeader>() as usize)
                    .wrapping_add(size_of::<W_>() as usize)
                    .wrapping_sub(1 as usize)
                    .wrapping_div(size_of::<W_>() as usize)
                    .wrapping_add(2 as usize) as size_t;
            }
            16 | 17 | 22 => {
                size = (size_of::<StgThunkHeader>() as usize)
                    .wrapping_add(size_of::<W_>() as usize)
                    .wrapping_sub(1 as usize)
                    .wrapping_div(size_of::<W_>() as usize)
                    .wrapping_add(1 as usize) as size_t;
            }
            8 | 38 | 37 | 9 | 10 | 12 | 13 | 11 | 1 | 7 | 2 | 3 | 5 | 6 | 4 => {
                size = sizeW_fromITBL(info) as size_t;
            }
            27 => {
                size = BLACKHOLE_sizeW() as size_t;
            }
            23 => {
                prim = r#true != 0;
                size = bco_sizeW(p as *mut StgBCO) as size_t;
            }
            39 | 40 | 41 | 49 | 50 | 51 | 47 | 48 => {
                prim = r#true != 0;
                size = sizeW_fromITBL(info) as size_t;
            }
            24 => {
                size = ap_sizeW(p as *mut StgAP) as size_t;
            }
            25 => {
                size = pap_sizeW(p as *mut StgPAP) as size_t;
            }
            26 => {
                size = ap_stack_sizeW(p as *mut StgAP_STACK) as size_t;
            }
            42 => {
                prim = r#true != 0;
                size = arr_words_sizeW(p as *mut StgArrBytes) as size_t;
            }
            43 | 44 | 46 | 45 => {
                prim = r#true != 0;
                size = mut_arr_ptrs_sizeW(p as *mut StgMutArrPtrs) as size_t;
            }
            59 | 60 | 62 | 61 => {
                prim = r#true != 0;
                size = small_mut_arr_ptrs_sizeW(p as *mut StgSmallMutArrPtrs) as size_t;
            }
            52 => {
                prim = r#true != 0;
                size = (size_of::<StgTSO>() as usize)
                    .wrapping_add(size_of::<W_>() as usize)
                    .wrapping_sub(1 as usize)
                    .wrapping_div(size_of::<W_>() as usize) as size_t;
            }
            53 => {
                prim = r#true != 0;
                size = stack_sizeW(p as *mut StgStack) as size_t;
            }
            54 => {
                prim = r#true != 0;
                size = (size_of::<StgTRecChunk>() as usize)
                    .wrapping_add(size_of::<W_>() as usize)
                    .wrapping_sub(1 as usize)
                    .wrapping_div(size_of::<W_>() as usize) as size_t;
            }
            64 => {
                size = continuation_sizeW(p as *mut StgContinuation) as size_t;
            }
            63 => {
                barf(
                    b"heapCensus, found compact object in the wrong list\0" as *const u8
                        as *const c_char,
                );
            }
            _ => {
                barf(
                    b"heapCensus, unknown object: %d\0" as *const u8 as *const c_char,
                    (*info).r#type,
                );
            }
        }

        heapProfObject(census, p as *mut StgClosure, size, prim);
        p = p.offset(size as isize);

        while p < (*bd).c2rust_unnamed.free && *p == 0 {
            p = p.offset(1);
        }
    }
}

unsafe fn closureIsPrim(mut p: StgPtr) -> bool {
    let mut prim = r#false != 0;
    let mut info = get_itbl(p as *const StgClosure);

    match (*info).r#type {
        15 | 19 | 20 | 18 | 16 | 17 | 22 | 8 | 38 | 37 | 9 | 10 | 12 | 13 | 11 | 1 | 7 | 2 | 3
        | 5 | 6 | 4 | 27 | 24 | 25 | 26 | 64 => {
            prim = r#false != 0;
        }

        23 | 39 | 40 | 41 | 49 | 50 | 51 | 47 | 48 | 42 | 43 | 44 | 46 | 45 | 59 | 60 | 62 | 61
        | 52 | 53 | 54 => {
            prim = r#true != 0;
        }
        63 => {
            barf(
                b"heapCensus, found compact object in the wrong list\0" as *const u8
                    as *const c_char,
            );
        }
        _ => {
            barf(
                b"heapCensus, unknown object: %d\0" as *const u8 as *const c_char,
                (*info).r#type,
            );
        }
    }

    return prim;
}

unsafe fn heapCensusSegment(mut census: *mut Census, mut seg: *mut NonmovingSegment) {
    let mut block_size = nonmovingSegmentBlockSize(seg);
    let mut block_count = nonmovingSegmentBlockCount(seg);
    let mut b = 0 as c_uint;

    while b < block_count {
        let mut p = nonmovingSegmentGetBlock(seg, b as nonmoving_block_idx) as StgPtr;

        if nonmovingClosureMarkedThisCycle(p) {
            heapProfObject(
                census,
                p as *mut StgClosure,
                (block_size as size_t).wrapping_div(size_of::<W_>() as size_t),
                closureIsPrim(p),
            );
        }

        b = b.wrapping_add(1);
    }
}

unsafe fn heapCensusSegmentList(mut census: *mut Census, mut seg: *mut NonmovingSegment) {
    while !seg.is_null() {
        heapCensusSegment(census, seg);
        seg = (*seg).link;
    }
}

unsafe fn heapCensusChain(mut census: *mut Census, mut bd: *mut bdescr) {
    let mut current_block_3: u64;

    while !bd.is_null() {
        if (*bd).flags as c_int & BF_LARGE != 0 {
            let mut p = (*bd).start;

            while p < (*bd).c2rust_unnamed.free && *p == 0 {
                p = p.offset(1);
            }

            if (*get_itbl(p as *mut StgClosure)).r#type == ARR_WORDS as StgHalfWord {
                let mut size: size_t = arr_words_sizeW(p as *mut StgArrBytes) as size_t;
                let mut prim = r#true != 0;
                heapProfObject(census, p as *mut StgClosure, size, prim);
                current_block_3 = 792017965103506125;
            } else {
                current_block_3 = 7351195479953500246;
            }
        } else {
            current_block_3 = 7351195479953500246;
        }

        match current_block_3 {
            7351195479953500246 => {
                heapCensusBlock(census, bd);
            }
            _ => {}
        }

        bd = (*bd).link as *mut bdescr;
    }
}

unsafe fn heapCensus(mut t: Time) {
    let mut g: uint32_t = 0;
    let mut n: uint32_t = 0;
    let mut census = null_mut::<Census>();
    let mut ws = null_mut::<gen_workspace>();
    census = censuses.offset(era as isize) as *mut Census;
    (*census).time = t as c_double / TIME_RESOLUTION as c_double;
    (*census).rtime = stat_getElapsedTime() as StgWord64;

    if doingRetainerProfiling() {
        retainerProfile();
    }

    stat_startHeapCensus();
    g = 0 as uint32_t;

    while g < RtsFlags.GcFlags.generations {
        heapCensusChain(census, (*generations.offset(g as isize)).blocks);
        heapCensusChain(census, (*generations.offset(g as isize)).large_objects);
        heapCensusCompactList(census, (*generations.offset(g as isize)).compact_objects);
        n = 0 as uint32_t;

        while n < getNumCapabilities() as uint32_t {
            ws = (&raw mut (**gc_threads.offset(n as isize)).gens as *mut gen_workspace)
                .offset(g as isize) as *mut gen_workspace;
            heapCensusChain(census, (*ws).0.todo_bd);
            heapCensusChain(census, (*ws).0.part_list);
            heapCensusChain(census, (*ws).0.scavd_list);
            n = n.wrapping_add(1);
        }

        g = g.wrapping_add(1);
    }

    if RtsFlags.GcFlags.useNonmoving {
        let mut i = 0 as c_uint;

        while i < nonmoving_alloca_cnt as c_uint {
            heapCensusSegmentList(
                census,
                (*nonmovingHeap.allocators.offset(i as isize)).filled,
            );

            heapCensusSegmentList(
                census,
                (*nonmovingHeap.allocators.offset(i as isize)).saved_filled,
            );

            heapCensusSegmentList(
                census,
                (*nonmovingHeap.allocators.offset(i as isize)).active,
            );

            heapCensusChain(census, nonmoving_large_objects);
            heapCensusCompactList(census, nonmoving_compact_objects);

            let mut j = 0 as c_uint;

            while j < getNumCapabilities() {
                let mut cap = getCapability(j as uint32_t);
                heapCensusSegment(census, *(*cap).current_segments.offset(i as isize));
                j = j.wrapping_add(1);
            }

            i = i.wrapping_add(1);
        }
    }

    if !doingLDVProfiling() {
        dumpCensus(census);
    }

    if RtsFlags.ProfFlags.bioSelector.is_null() {
        freeEra(census);
        (*census).hash = null_mut::<HashTable>();
        (*census).arena = null_mut::<Arena>();
    }

    nextEra();
    stat_endHeapCensus();
}
