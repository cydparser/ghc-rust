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
    doingRetainerProfiling, get_itbl, isInherentlyUsed, itbl_to_con_itbl, mut_arr_ptrs_sizeW,
    pap_sizeW, sizeW_fromITBL, small_mut_arr_ptrs_sizeW, stack_sizeW, thunk_sizeW_fromITBL,
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
use crate::ffi::rts::{_assertFail, prog_argc, prog_argv, prog_name, stg_exit};
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
    pub(crate) prim: isize,
    pub(crate) not_used: isize,
    pub(crate) used: isize,
    pub(crate) void_total: isize,
    pub(crate) drag_total: isize,
}

pub(crate) union C2RustUnnamed_12 {
    pub(crate) resid: isize,
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
    pub(crate) time: f64,
    pub(crate) rtime: StgWord64,
    pub(crate) hash: *mut HashTable,
    pub(crate) ctrs: *mut counter,
    pub(crate) arena: *mut Arena,
    pub(crate) prim: isize,
    pub(crate) not_used: isize,
    pub(crate) used: isize,
    pub(crate) void_total: isize,
    pub(crate) drag_total: isize,
}

static mut hp_file: *mut FILE = null_mut::<FILE>();

static mut hp_filename: *mut c_char = null_mut::<c_char>();

static mut saved_locale: locale_t = null_mut::<_xlocale>();

static mut prof_locale: locale_t = null_mut::<_xlocale>();

unsafe fn init_prof_locale() {
    if prof_locale.is_null() {
        prof_locale = newlocale(LC_NUMERIC_MASK, c"POSIX".as_ptr(), null_mut::<_xlocale>());

        if prof_locale.is_null() {
            sysErrorBelch(c"Couldn't allocate heap profiler locale".as_ptr());
        }
    }
}

unsafe fn free_prof_locale() {
    if !prof_locale.is_null() {
        freelocale(prof_locale);
        prof_locale = null_mut::<_xlocale>();
    }
}

unsafe fn set_prof_locale() {
    saved_locale = uselocale(prof_locale);
}

unsafe fn restore_locale() {
    uselocale(saved_locale);
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut era: c_uint = 0;

static mut max_era: u32 = 0;

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
    (*ctr).c.ldv.prim = 0;
    (*ctr).c.ldv.not_used = 0;
    (*ctr).c.ldv.used = 0;
    (*ctr).c.ldv.void_total = 0;
    (*ctr).c.ldv.drag_total = 0;
}

static mut censuses: *mut Census = null_mut::<Census>();

static mut n_censuses: u32 = 0;

unsafe fn closureIdentity(mut p: *const StgClosure) -> *const c_void {
    match RtsFlags.ProfFlags.doHeapProfile {
        1 => return (*p).header.prof.ccs as *const c_void,
        2 => return (*(*(*p).header.prof.ccs).cc).module as *const c_void,
        4 => {
            return (get_itbl(p).offset(1 as i32 as isize) as StgWord)
                .wrapping_add((*get_itbl(p)).prof.closure_desc_off as StgWord)
                as *mut c_char as *const c_void;
        }
        10 => return (*p).header.prof.hp.era as *mut c_void,
        5 => {
            return (get_itbl(p).offset(1 as i32 as isize) as StgWord)
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
                    return (itbl_to_con_itbl(info).offset(1 as i32 as isize) as StgWord)
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
            barf(c"closureIdentity".as_ptr());
        }
    };
}

unsafe fn LDV_recordDead(mut c: *const StgClosure, mut size: u32) {
    let mut id = null::<c_void>();
    let mut t: u32 = 0;
    let mut ctr = null_mut::<counter>();

    if !isInherentlyUsed((*get_itbl(c)).r#type) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/ProfHeap.c".as_ptr(), 263);
    }

    if era > 0 && closureSatisfiesConstraints(c) as i32 != 0 {
        size = (size as u64).wrapping_sub(
            (size_of::<StgProfHeader>() as usize)
                .wrapping_add(size_of::<W_>() as usize)
                .wrapping_sub(1 as usize)
                .wrapping_div(size_of::<W_>() as usize) as u64,
        ) as u32 as u32;

        if ((*(c as *mut StgClosure)).header.prof.hp.ldvw != 0) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/ProfHeap.c".as_ptr(), 267);
        }

        if (*(c as *mut StgClosure)).header.prof.hp.ldvw & LDV_STATE_MASK as StgWord
            == LDV_STATE_CREATE as StgWord
        {
            t = (((*(c as *mut StgClosure)).header.prof.hp.ldvw & LDV_CREATE_MASK as StgWord)
                >> LDV_SHIFT) as u32;

            if t < era as u32 {
                if RtsFlags.ProfFlags.bioSelector.is_null() {
                    (*censuses.offset(t as isize)).void_total += size as isize;
                    (*censuses.offset(era as isize)).void_total -= size as isize;

                    if ((*censuses.offset(t as isize)).void_total
                        <= (*censuses.offset(t as isize)).not_used) as i32
                        as i64
                        != 0
                    {
                    } else {
                        _assertFail(c"rts/ProfHeap.c".as_ptr(), 274);
                    }
                } else {
                    id = closureIdentity(c);

                    ctr = lookupHashTable((*censuses.offset(t as isize)).hash, id as StgWord)
                        as *mut counter;

                    if ctr.is_null() {
                        barf(
                            c"LDV_recordDead: Failed to find counter for closure %p".as_ptr(),
                            c,
                        );
                    }

                    (*ctr).c.ldv.void_total += size as isize;

                    ctr = lookupHashTable((*censuses.offset(era as isize)).hash, id as StgWord)
                        as *mut counter;

                    if ctr.is_null() {
                        ctr = arenaAlloc(
                            (*censuses.offset(era as isize)).arena,
                            size_of::<counter>() as usize,
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

                    (*ctr).c.ldv.void_total -= size as isize;
                }
            }
        } else {
            t = ((*(c as *mut StgClosure)).header.prof.hp.ldvw & LDV_LAST_MASK as StgWord) as u32;

            if t.wrapping_add(1 as u32) < era as u32 {
                if RtsFlags.ProfFlags.bioSelector.is_null() {
                    (*censuses.offset(t.wrapping_add(1 as u32) as isize)).drag_total +=
                        size as isize;
                    (*censuses.offset(era as isize)).drag_total -= size as isize;
                } else {
                    let mut id_0 = null::<c_void>();
                    id_0 = closureIdentity(c);

                    ctr = lookupHashTable(
                        (*censuses.offset(t.wrapping_add(1 as u32) as isize)).hash,
                        id_0 as StgWord,
                    ) as *mut counter;

                    if !ctr.is_null() as i32 as i64 != 0 {
                    } else {
                        _assertFail(c"rts/ProfHeap.c".as_ptr(), 304);
                    }

                    (*ctr).c.ldv.drag_total += size as isize;

                    ctr = lookupHashTable((*censuses.offset(era as isize)).hash, id_0 as StgWord)
                        as *mut counter;

                    if ctr.is_null() {
                        ctr = arenaAlloc(
                            (*censuses.offset(era as isize)).arena,
                            size_of::<counter>() as usize,
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

                    (*ctr).c.ldv.drag_total -= size as isize;
                }
            }
        }
    }
}

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
    (*census).not_used = 0;
    (*census).used = 0;
    (*census).prim = 0;
    (*census).void_total = 0;
    (*census).drag_total = 0;
}

unsafe fn freeEra(mut census: *mut Census) {
    arenaFree((*census).arena);
    freeHashTable((*census).hash, None);
}

unsafe fn nextEra() {
    if user_era > 0 && RtsFlags.ProfFlags.incrementUserEra as i32 != 0 {
        user_era = user_era.wrapping_add(1);
    }

    if doingLDVProfiling() {
        era = era.wrapping_add(1);

        if era as u32 == max_era {
            errorBelch(c"Maximum number of censuses reached.".as_ptr());

            if rtsConfig.rts_opts_suggestions == true {
                if rtsConfig.rts_opts_enabled as u32 == RtsOptsAll as i32 as u32 {
                    errorBelch(c"Use `+RTS -i' to reduce censuses.".as_ptr());
                } else {
                    errorBelch(
                        c"Relink with -rtsopts and use `+RTS -i' to reduce censuses.".as_ptr(),
                    );
                }
            }

            stg_exit(EXIT_FAILURE);
        }

        if era as u32 == n_censuses {
            n_censuses = n_censuses.wrapping_mul(2 as u32);

            censuses = stgReallocBytes(
                censuses as *mut c_void,
                (size_of::<Census>() as usize).wrapping_mul(n_censuses as usize),
                c"nextEra".as_ptr(),
            ) as *mut Census;

            memset(
                censuses.offset(era as isize) as *mut Census as *mut c_void,
                0,
                (size_of::<Census>() as usize)
                    .wrapping_mul(n_censuses as usize)
                    .wrapping_div(2 as usize),
            );
        }
    }

    initEra(censuses.offset(era as isize) as *mut Census);
}

unsafe fn printEscapedString(mut string: *const c_char) {
    let mut p = string;

    while *p as i32 != '\0' as i32 {
        if *p as i32 == '"' as i32 {
            fputc('"' as i32, hp_file);
        }

        fputc(*p as i32, hp_file);
        p = p.offset(1);
    }
}

unsafe fn printSample(mut beginSample: bool, mut sampleValue: StgDouble) {
    fprintf(
        hp_file,
        c"%s %f\n".as_ptr(),
        if beginSample as i32 != 0 {
            c"BEGIN_SAMPLE".as_ptr()
        } else {
            c"END_SAMPLE".as_ptr()
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
            strlen(RtsFlags.CcFlags.outputFileNameStem).wrapping_add(1 as usize),
            c"initHeapProfiling".as_ptr(),
        ) as *mut c_char;

        strcpy(stem, RtsFlags.CcFlags.outputFileNameStem);
    } else {
        stem = stgMallocBytes(
            strlen(prog_name).wrapping_add(1 as usize),
            c"initHeapProfiling".as_ptr(),
        ) as *mut c_char;

        strcpy(stem, prog_name);
    }

    if RtsFlags.ProfFlags.doHeapProfile != 0 {
        hp_filename = stgMallocBytes(
            strlen(stem).wrapping_add(6 as usize),
            c"hpFileName".as_ptr(),
        ) as *mut c_char;

        sprintf(hp_filename, c"%s.hp".as_ptr(), stem);
        hp_file = __rts_fopen(hp_filename, c"w+".as_ptr());

        if hp_file.is_null() {
            debugBelch(
                c"Can't open profiling report file %s\n".as_ptr(),
                hp_filename,
            );
            RtsFlags.ProfFlags.doHeapProfile = 0;
            stgFree(stem as *mut c_void);
            return;
        }
    }

    stgFree(stem as *mut c_void);

    if doingLDVProfiling() as i32 != 0 && doingRetainerProfiling() as i32 != 0 {
        errorBelch(c"cannot mix -hb and -hr".as_ptr());
        stg_exit(EXIT_FAILURE);
    }

    if doingLDVProfiling() as i32 != 0 && RtsFlags.ParFlags.nCapabilities > 1 {
        errorBelch(c"-hb cannot be used with multiple capabilities".as_ptr());
        stg_exit(EXIT_FAILURE);
    }

    if doingErasProfiling() {
        user_era = 1;
    }

    if doingLDVProfiling() {
        era = 1;
        n_censuses = 32;
    } else {
        era = 0;
        n_censuses = 1;
    }

    max_era = (1 << LDV_SHIFT) as u32;

    censuses = stgMallocBytes(
        (size_of::<Census>() as usize).wrapping_mul(n_censuses as usize),
        c"initHeapProfiling".as_ptr(),
    ) as *mut Census;

    let mut i = 0;

    while (i as u32) < n_censuses {
        let ref mut fresh8 = (*censuses.offset(i as isize)).arena;
        *fresh8 = null_mut::<Arena>();

        let ref mut fresh9 = (*censuses.offset(i as isize)).hash;
        *fresh9 = null_mut::<HashTable>();
        i = i.wrapping_add(1);
    }

    initEra(censuses.offset(era as isize) as *mut Census);
    fprintf(hp_file, c"JOB \"".as_ptr());
    printEscapedString(prog_name);

    let mut i_0 = 1;

    while i_0 < prog_argc {
        fputc(' ' as i32, hp_file);
        printEscapedString(*prog_argv.offset(i_0 as isize));
        i_0 += 1;
    }

    fprintf(hp_file, c" +RTS".as_ptr());

    let mut i_1 = 0;

    while i_1 < rts_argc {
        fputc(' ' as i32, hp_file);
        printEscapedString(*rts_argv.offset(i_1 as isize));
        i_1 += 1;
    }

    fprintf(hp_file, c"\"\n".as_ptr());
    fprintf(hp_file, c"DATE \"%s\"\n".as_ptr(), time_str());
    fprintf(hp_file, c"SAMPLE_UNIT \"seconds\"\n".as_ptr());
    fprintf(hp_file, c"VALUE_UNIT \"bytes\"\n".as_ptr());
    printSample(true, 0);
    printSample(false, 0);

    if doingRetainerProfiling() {
        initRetainerProfiling();
    }

    restore_locale();
    traceHeapProfBegin(0);
}

unsafe fn endHeapProfiling() {
    if RtsFlags.ProfFlags.doHeapProfile == 0 {
        return;
    }

    set_prof_locale();

    if doingRetainerProfiling() {
        endRetainerProfiling();
    } else if doingLDVProfiling() {
        let mut t: u32 = 0;
        LdvCensusKillAll();
        aggregateCensusInfo();
        t = 1;

        while t < era as u32 {
            dumpCensus(censuses.offset(t as isize) as *mut Census);
            t = t.wrapping_add(1);
        }

        if !RtsFlags.ProfFlags.bioSelector.is_null() {
            t = 1;

            while t <= era as u32 {
                freeEra(censuses.offset(t as isize) as *mut Census);
                t = t.wrapping_add(1);
            }
        } else {
            freeEra(censuses.offset(era as isize) as *mut Census);
        }
    } else {
        freeEra(censuses.offset(0) as *mut Census);
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
    printSample(true, seconds);
    printSample(false, seconds);
    fclose(hp_file);
    restore_locale();
}

unsafe fn buf_append(mut p: *mut c_char, mut q: *const c_char, mut end: *mut c_char) -> usize {
    let mut m: i32 = 0;
    m = 0;

    while p < end {
        *p = *q;

        if *q as i32 == '\0' as i32 {
            break;
        }

        p = p.offset(1);
        q = q.offset(1);
        m += 1;
    }

    return m as usize;
}

unsafe fn fprint_ccs(mut fp: *mut FILE, mut ccs: *mut CostCentreStack, mut max_length: u32) {
    let vla = max_length.wrapping_add(1 as u32) as usize;
    let mut buf: Vec<c_char> = ::std::vec::from_elem(0, vla);
    let mut p = null_mut::<c_char>();
    let mut buf_end = null_mut::<c_char>();

    if ccs == &raw mut CCS_MAIN as *mut CostCentreStack {
        fprintf(fp, c"MAIN".as_ptr());
        return;
    }

    fprintf(fp, c"(%lld)".as_ptr(), (*ccs).ccsID);
    p = buf.as_mut_ptr();
    buf_end = buf.as_mut_ptr().offset(max_length as isize).offset(1);

    while !ccs.is_null() && ccs != &raw mut CCS_MAIN as *mut CostCentreStack {
        if strcmp((*(*ccs).cc).label, c"CAF".as_ptr()) == 0 {
            p = p.offset(buf_append(p, (*(*ccs).cc).module, buf_end) as isize);
            p = p.offset(buf_append(p, c".CAF".as_ptr(), buf_end) as isize);
        } else {
            p = p.offset(buf_append(p, (*(*ccs).cc).label, buf_end) as isize);

            if !(*ccs).prevStack.is_null()
                && (*ccs).prevStack != &raw mut CCS_MAIN as *mut CostCentreStack
            {
                p = p.offset(buf_append(p, c"/".as_ptr(), buf_end) as isize);
            }
        }

        if p >= buf_end {
            sprintf(
                buf.as_mut_ptr().offset(max_length as isize).offset(-4),
                c"...".as_ptr(),
            );

            break;
        } else {
            ccs = (*ccs).prevStack as *mut CostCentreStack;
        }
    }

    fprintf(fp, c"%s".as_ptr(), buf.as_mut_ptr());
}

unsafe fn strMatchesSelector(mut str: *const c_char, mut sel: *const c_char) -> bool {
    let mut p = null::<c_char>();

    loop {
        p = str;

        while *p as i32 != '\0' as i32
            && *sel as i32 != ',' as i32
            && *sel as i32 != '\0' as i32
            && *p as i32 == *sel as i32
        {
            p = p.offset(1);
            sel = sel.offset(1);
        }

        if *p as i32 == '\0' as i32 && (*sel as i32 == ',' as i32 || *sel as i32 == '\0' as i32) {
            return true;
        }

        while *sel as i32 != ',' as i32 && *sel as i32 != '\0' as i32 {
            sel = sel.offset(1);
        }

        if *sel as i32 == ',' as i32 {
            sel = sel.offset(1);
        }

        if *sel as i32 == '\0' as i32 {
            return false;
        }
    }
}

unsafe fn closureSatisfiesConstraints(mut p: *const StgClosure) -> bool {
    let mut b: bool = false;

    if (*(*p).header.prof.ccs).selected == 0 {
        return false;
    }

    if !RtsFlags.ProfFlags.descrSelector.is_null() {
        b = strMatchesSelector(
            (get_itbl(p as *mut StgClosure).offset(1 as i32 as isize) as StgWord)
                .wrapping_add((*get_itbl(p as *mut StgClosure)).prof.closure_desc_off as StgWord)
                as *mut c_char,
            RtsFlags.ProfFlags.descrSelector,
        );

        if !b {
            return false;
        }
    }

    if !RtsFlags.ProfFlags.typeSelector.is_null() {
        b = strMatchesSelector(
            (get_itbl(p as *mut StgClosure).offset(1 as i32 as isize) as StgWord)
                .wrapping_add((*get_itbl(p as *mut StgClosure)).prof.closure_type_off as StgWord)
                as *mut c_char,
            RtsFlags.ProfFlags.typeSelector,
        );

        if !b {
            return false;
        }
    }

    if RtsFlags.ProfFlags.eraSelector != 0 {
        return (*p).header.prof.hp.era == RtsFlags.ProfFlags.eraSelector;
    }

    if !RtsFlags.ProfFlags.retainerSelector.is_null() {
        let mut rs = null_mut::<RetainerSet>();
        let mut i: u32 = 0;

        if isRetainerSetValid(p as *mut StgClosure) {
            rs = retainerSetOf(p as *mut StgClosure);

            if !rs.is_null() {
                i = 0;

                while i < (*rs).num {
                    b = strMatchesSelector(
                        (*(**(&raw mut (*rs).element as *mut retainer).offset(i as isize)).cc)
                            .label,
                        RtsFlags.ProfFlags.retainerSelector,
                    );

                    if b {
                        return true;
                    }

                    i = i.wrapping_add(1);
                }
            }
        }

        return false;
    }

    return true;
}

unsafe fn aggregateCensusInfo() {
    let mut acc = null_mut::<HashTable>();
    let mut t: u32 = 0;
    let mut c = null_mut::<counter>();
    let mut d = null_mut::<counter>();
    let mut ctrs = null_mut::<counter>();
    let mut arena = null_mut::<Arena>();

    if !doingLDVProfiling() {
        return;
    }

    if RtsFlags.ProfFlags.doHeapProfile == HEAP_BY_LDV as u32 {
        let mut void_total: i64 = 0;
        let mut drag_total: i64 = 0;
        void_total = 0;
        drag_total = 0;
        t = 1;

        while t < era as u32 {
            void_total += (*censuses.offset(t as isize)).void_total as i64;
            drag_total += (*censuses.offset(t as isize)).drag_total as i64;
            (*censuses.offset(t as isize)).void_total = void_total as isize;
            (*censuses.offset(t as isize)).drag_total = drag_total as isize;

            if ((*censuses.offset(t as isize)).void_total
                <= (*censuses.offset(t as isize)).not_used) as i32 as i64
                != 0
            {
            } else {
                _assertFail(c"rts/ProfHeap.c".as_ptr(), 783);
            }

            if ((*censuses.offset(t as isize)).drag_total <= (*censuses.offset(t as isize)).used)
                as i32 as i64
                != 0
            {
            } else {
                _assertFail(c"rts/ProfHeap.c".as_ptr(), 789);
            }

            t = t.wrapping_add(1);
        }

        return;
    }

    arena = newArena();
    acc = allocHashTable();
    ctrs = null_mut::<counter>();
    t = 1;

    while t < era as u32 {
        c = ctrs;

        while !c.is_null() {
            d = lookupHashTable(
                (*censuses.offset(t as isize)).hash,
                (*c).identity as StgWord,
            ) as *mut counter;

            if d.is_null() {
                if ((*c).c.ldv.void_total == 0 && (*c).c.ldv.drag_total == 0) as i32 as i64 != 0 {
                } else {
                    _assertFail(c"rts/ProfHeap.c".as_ptr(), 817);
                }
            } else {
                (*d).c.ldv.void_total += (*c).c.ldv.void_total;
                (*d).c.ldv.drag_total += (*c).c.ldv.drag_total;
                (*c).c.ldv.void_total = (*d).c.ldv.void_total;
                (*c).c.ldv.drag_total = (*d).c.ldv.drag_total;

                if ((*c).c.ldv.void_total >= 0) as i32 as i64 != 0 {
                } else {
                    _assertFail(c"rts/ProfHeap.c".as_ptr(), 828);
                }

                if ((*c).c.ldv.drag_total >= 0) as i32 as i64 != 0 {
                } else {
                    _assertFail(c"rts/ProfHeap.c".as_ptr(), 829);
                }
            }

            c = (*c).next as *mut counter;
        }

        c = (*censuses.offset(t as isize)).ctrs;

        while !c.is_null() {
            d = lookupHashTable(acc, (*c).identity as StgWord) as *mut counter;

            if d.is_null() {
                d = arenaAlloc(arena, size_of::<counter>() as usize) as *mut counter;
                initLDVCtr(d);
                insertHashTable(acc, (*c).identity as StgWord, d as *const c_void);
                (*d).identity = (*c).identity;
                (*d).next = ctrs as *mut _counter;
                ctrs = d;
                (*d).c.ldv.void_total = (*c).c.ldv.void_total;
                (*d).c.ldv.drag_total = (*c).c.ldv.drag_total;
            }

            if ((*c).c.ldv.void_total >= 0) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/ProfHeap.c".as_ptr(), 846);
            }

            if ((*c).c.ldv.drag_total >= 0) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/ProfHeap.c".as_ptr(), 847);
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
    let mut count: isize = 0;
    set_prof_locale();
    printSample(true, (*census).time as StgDouble);

    if RtsFlags.ProfFlags.doHeapProfile == HEAP_BY_LDV as u32 {
        traceHeapBioProfSampleBegin(era as StgInt, (*census).rtime);
    } else {
        traceHeapProfSampleBegin(era as StgInt);
    }

    if RtsFlags.ProfFlags.doHeapProfile == HEAP_BY_LDV as u32 {
        fprintf(
            hp_file,
            c"VOID\t%llu\n".as_ptr(),
            ((*census).void_total as usize).wrapping_mul(size_of::<W_>() as usize) as u64,
        );

        fprintf(
            hp_file,
            c"LAG\t%llu\n".as_ptr(),
            (((*census).not_used - (*census).void_total) as usize)
                .wrapping_mul(size_of::<W_>() as usize) as u64,
        );

        fprintf(
            hp_file,
            c"USE\t%llu\n".as_ptr(),
            (((*census).used - (*census).drag_total) as usize)
                .wrapping_mul(size_of::<W_>() as usize) as u64,
        );

        fprintf(
            hp_file,
            c"INHERENT_USE\t%llu\n".as_ptr(),
            ((*census).prim as usize).wrapping_mul(size_of::<W_>() as usize) as u64,
        );

        fprintf(
            hp_file,
            c"DRAG\t%llu\n".as_ptr(),
            ((*census).drag_total as usize).wrapping_mul(size_of::<W_>() as usize) as u64,
        );

        traceHeapProfSampleString(
            0,
            c"VOID".as_ptr(),
            ((*census).void_total as usize).wrapping_mul(size_of::<W_>() as usize) as StgWord,
        );

        traceHeapProfSampleString(
            0,
            c"LAG".as_ptr(),
            (((*census).not_used - (*census).void_total) as usize)
                .wrapping_mul(size_of::<W_>() as usize) as StgWord,
        );

        traceHeapProfSampleString(
            0,
            c"USE".as_ptr(),
            (((*census).used - (*census).drag_total) as usize)
                .wrapping_mul(size_of::<W_>() as usize) as StgWord,
        );

        traceHeapProfSampleString(
            0,
            c"INHERENT_USE".as_ptr(),
            ((*census).prim as usize).wrapping_mul(size_of::<W_>() as usize) as StgWord,
        );

        traceHeapProfSampleString(
            0,
            c"DRAG".as_ptr(),
            ((*census).drag_total as usize).wrapping_mul(size_of::<W_>() as usize) as StgWord,
        );

        traceHeapProfSampleEnd(era as StgInt);
        printSample(false, (*census).time as StgDouble);
        return;
    }

    ctr = (*census).ctrs;

    while !ctr.is_null() {
        if !RtsFlags.ProfFlags.bioSelector.is_null() {
            count = 0;

            if strMatchesSelector(c"lag".as_ptr(), RtsFlags.ProfFlags.bioSelector) {
                count += (*ctr).c.ldv.not_used - (*ctr).c.ldv.void_total;
            }

            if strMatchesSelector(c"drag".as_ptr(), RtsFlags.ProfFlags.bioSelector) {
                count += (*ctr).c.ldv.drag_total;
            }

            if strMatchesSelector(c"void".as_ptr(), RtsFlags.ProfFlags.bioSelector) {
                count += (*ctr).c.ldv.void_total;
            }

            if strMatchesSelector(c"use".as_ptr(), RtsFlags.ProfFlags.bioSelector) {
                count += (*ctr).c.ldv.used - (*ctr).c.ldv.drag_total;
            }
        } else {
            count = (*ctr).c.resid;
        }

        if (count >= 0) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/ProfHeap.c".as_ptr(), 937);
        }

        if !(count == 0) {
            let mut str: [c_char; 100] = [0; 100];
            let mut str_era: [c_char; 100] = [0; 100];

            match RtsFlags.ProfFlags.doHeapProfile {
                8 => {
                    fprintf(hp_file, c"%s".as_ptr(), (*ctr).identity as *mut c_char);

                    traceHeapProfSampleString(
                        0,
                        (*ctr).identity as *mut c_char,
                        (count as usize).wrapping_mul(size_of::<W_>() as usize) as StgWord,
                    );
                }
                9 => {
                    fprintf(hp_file, c"%p".as_ptr(), (*ctr).identity);
                    str = [0; 100];

                    sprintf(&raw mut str as *mut c_char, c"%p".as_ptr(), (*ctr).identity);

                    traceHeapProfSampleString(
                        0,
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
                        0,
                        (*ctr).identity as *mut CostCentreStack,
                        (count as usize).wrapping_mul(size_of::<W_>() as usize) as StgWord,
                    );
                }
                10 => {
                    fprintf(hp_file, c"%llu".as_ptr(), (*ctr).identity as StgWord);
                    str_era = [0; 100];

                    sprintf(
                        &raw mut str_era as *mut c_char,
                        c"%llu".as_ptr(),
                        (*ctr).identity as StgWord,
                    );

                    traceHeapProfSampleString(
                        0,
                        &raw mut str_era as *mut c_char,
                        (count as usize).wrapping_mul(size_of::<W_>() as usize) as StgWord,
                    );
                }
                2 | 4 | 5 => {
                    fprintf(hp_file, c"%s".as_ptr(), (*ctr).identity as *mut c_char);

                    traceHeapProfSampleString(
                        0,
                        (*ctr).identity as *mut c_char,
                        (count as usize).wrapping_mul(size_of::<W_>() as usize) as StgWord,
                    );
                }
                6 => {
                    let mut rs = (*ctr).identity as *mut RetainerSet;

                    if rs == &raw mut rs_MANY {
                        fprintf(hp_file, c"MANY".as_ptr());
                    } else {
                        if (*rs).id > 0 {
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
                    barf(c"dumpCensus; doHeapProfile".as_ptr());
                }
            }

            fprintf(
                hp_file,
                c"\t%llu\n".as_ptr(),
                (count as W_).wrapping_mul(size_of::<W_>() as W_),
            );
        }

        ctr = (*ctr).next as *mut counter;
    }

    traceHeapProfSampleEnd(era as StgInt);
    printSample(false, (*census).time as StgDouble);
    restore_locale();
}

#[inline]
unsafe fn heapInsertNewCounter(mut census: *mut Census, mut identity: StgWord) -> *mut counter {
    let mut ctr = arenaAlloc((*census).arena, size_of::<counter>() as usize) as *mut counter;
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
    mut size: usize,
    mut prim: bool,
) {
    let mut identity = null::<c_void>();
    let mut real_size: usize = 0;
    let mut ctr = null_mut::<counter>();
    identity = null::<c_void>();
    real_size = size.wrapping_sub(
        (size_of::<StgProfHeader>() as usize)
            .wrapping_add(size_of::<W_>() as usize)
            .wrapping_sub(1 as usize)
            .wrapping_div(size_of::<W_>() as usize),
    );

    if closureSatisfiesConstraints(p) {
        if RtsFlags.ProfFlags.doHeapProfile == HEAP_BY_LDV as u32 {
            if prim {
                (*census).prim =
                    ((*census).prim as usize).wrapping_add(real_size) as isize as isize;
            } else if (*p).header.prof.hp.ldvw & LDV_STATE_MASK as StgWord
                == LDV_STATE_CREATE as StgWord
            {
                (*census).not_used =
                    ((*census).not_used as usize).wrapping_add(real_size) as isize as isize;
            } else {
                (*census).used =
                    ((*census).used as usize).wrapping_add(real_size) as isize as isize;
            }
        } else {
            identity = closureIdentity(p);

            if !identity.is_null() {
                ctr = lookupHashTable((*census).hash, identity as StgWord) as *mut counter;

                if !ctr.is_null() {
                    if !RtsFlags.ProfFlags.bioSelector.is_null() {
                        if prim {
                            (*ctr).c.ldv.prim = ((*ctr).c.ldv.prim as usize).wrapping_add(real_size)
                                as isize as isize;
                        } else if (*p).header.prof.hp.ldvw & LDV_STATE_MASK as StgWord
                            == LDV_STATE_CREATE as StgWord
                        {
                            (*ctr).c.ldv.not_used =
                                ((*ctr).c.ldv.not_used as usize).wrapping_add(real_size) as isize
                                    as isize;
                        } else {
                            (*ctr).c.ldv.used = ((*ctr).c.ldv.used as usize).wrapping_add(real_size)
                                as isize as isize;
                        }
                    } else {
                        (*ctr).c.resid =
                            ((*ctr).c.resid as usize).wrapping_add(real_size) as isize as isize;
                    }
                } else {
                    ctr = heapInsertNewCounter(census, identity as StgWord);

                    if !RtsFlags.ProfFlags.bioSelector.is_null() {
                        if prim {
                            (*ctr).c.ldv.prim = real_size as isize;
                        } else if (*p).header.prof.hp.ldvw & LDV_STATE_MASK as StgWord
                            == LDV_STATE_CREATE as StgWord
                        {
                            (*ctr).c.ldv.not_used = real_size as isize;
                        } else {
                            (*ctr).c.ldv.used = real_size as isize;
                        }
                    } else {
                        (*ctr).c.resid = real_size as isize;
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
            compact_nfdata_full_sizeW(str) as usize,
            true,
        );

        bd = (*bd).link as *mut bdescr;
    }
}

unsafe fn heapCensusBlock(mut census: *mut Census, mut bd: *mut bdescr) {
    let mut p = (*bd).start;

    if (*bd).flags as i32 & BF_PINNED != 0 {
        while p < (*bd).c2rust_unnamed.free && *p == 0 {
            p = p.offset(1);
        }
    }

    while p < (*bd).c2rust_unnamed.free {
        let mut info = get_itbl(p as *const StgClosure);
        let mut prim = false;
        let mut size: usize = 0;

        match (*info).r#type {
            15 => {
                size = thunk_sizeW_fromITBL(info) as usize;
            }
            19 | 20 | 18 => {
                size = (size_of::<StgThunkHeader>() as usize)
                    .wrapping_add(size_of::<W_>() as usize)
                    .wrapping_sub(1 as usize)
                    .wrapping_div(size_of::<W_>() as usize)
                    .wrapping_add(2 as usize) as usize;
            }
            16 | 17 | 22 => {
                size = (size_of::<StgThunkHeader>() as usize)
                    .wrapping_add(size_of::<W_>() as usize)
                    .wrapping_sub(1 as usize)
                    .wrapping_div(size_of::<W_>() as usize)
                    .wrapping_add(1 as usize) as usize;
            }
            8 | 38 | 37 | 9 | 10 | 12 | 13 | 11 | 1 | 7 | 2 | 3 | 5 | 6 | 4 => {
                size = sizeW_fromITBL(info) as usize;
            }
            27 => {
                size = BLACKHOLE_sizeW() as usize;
            }
            23 => {
                prim = true;
                size = bco_sizeW(p as *mut StgBCO) as usize;
            }
            39 | 40 | 41 | 49 | 50 | 51 | 47 | 48 => {
                prim = true;
                size = sizeW_fromITBL(info) as usize;
            }
            24 => {
                size = ap_sizeW(p as *mut StgAP) as usize;
            }
            25 => {
                size = pap_sizeW(p as *mut StgPAP) as usize;
            }
            26 => {
                size = ap_stack_sizeW(p as *mut StgAP_STACK) as usize;
            }
            42 => {
                prim = true;
                size = arr_words_sizeW(p as *mut StgArrBytes) as usize;
            }
            43 | 44 | 46 | 45 => {
                prim = true;
                size = mut_arr_ptrs_sizeW(p as *mut StgMutArrPtrs) as usize;
            }
            59 | 60 | 62 | 61 => {
                prim = true;
                size = small_mut_arr_ptrs_sizeW(p as *mut StgSmallMutArrPtrs) as usize;
            }
            52 => {
                prim = true;
                size = (size_of::<StgTSO>() as usize)
                    .wrapping_add(size_of::<W_>() as usize)
                    .wrapping_sub(1 as usize)
                    .wrapping_div(size_of::<W_>() as usize) as usize;
            }
            53 => {
                prim = true;
                size = stack_sizeW(p as *mut StgStack) as usize;
            }
            54 => {
                prim = true;
                size = (size_of::<StgTRecChunk>() as usize)
                    .wrapping_add(size_of::<W_>() as usize)
                    .wrapping_sub(1 as usize)
                    .wrapping_div(size_of::<W_>() as usize) as usize;
            }
            64 => {
                size = continuation_sizeW(p as *mut StgContinuation) as usize;
            }
            63 => {
                barf(c"heapCensus, found compact object in the wrong list".as_ptr());
            }
            _ => {
                barf(c"heapCensus, unknown object: %d".as_ptr(), (*info).r#type);
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
    let mut prim = false;
    let mut info = get_itbl(p as *const StgClosure);

    match (*info).r#type {
        15 | 19 | 20 | 18 | 16 | 17 | 22 | 8 | 38 | 37 | 9 | 10 | 12 | 13 | 11 | 1 | 7 | 2 | 3
        | 5 | 6 | 4 | 27 | 24 | 25 | 26 | 64 => {
            prim = false;
        }

        23 | 39 | 40 | 41 | 49 | 50 | 51 | 47 | 48 | 42 | 43 | 44 | 46 | 45 | 59 | 60 | 62 | 61
        | 52 | 53 | 54 => {
            prim = true;
        }
        63 => {
            barf(c"heapCensus, found compact object in the wrong list".as_ptr());
        }
        _ => {
            barf(c"heapCensus, unknown object: %d".as_ptr(), (*info).r#type);
        }
    }

    return prim;
}

unsafe fn heapCensusSegment(mut census: *mut Census, mut seg: *mut NonmovingSegment) {
    let mut block_size = nonmovingSegmentBlockSize(seg);
    let mut block_count = nonmovingSegmentBlockCount(seg);
    let mut b = 0;

    while b < block_count {
        let mut p = nonmovingSegmentGetBlock(seg, b as nonmoving_block_idx) as StgPtr;

        if nonmovingClosureMarkedThisCycle(p) {
            heapProfObject(
                census,
                p as *mut StgClosure,
                (block_size as usize).wrapping_div(size_of::<W_>() as usize),
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
        if (*bd).flags as i32 & BF_LARGE != 0 {
            let mut p = (*bd).start;

            while p < (*bd).c2rust_unnamed.free && *p == 0 {
                p = p.offset(1);
            }

            if (*get_itbl(p as *mut StgClosure)).r#type == ARR_WORDS as StgHalfWord {
                let mut size: usize = arr_words_sizeW(p as *mut StgArrBytes) as usize;
                let mut prim = true;
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
    let mut g: u32 = 0;
    let mut n: u32 = 0;
    let mut census = null_mut::<Census>();
    let mut ws = null_mut::<gen_workspace>();
    census = censuses.offset(era as isize) as *mut Census;
    (*census).time = t as f64 / TIME_RESOLUTION as f64;
    (*census).rtime = stat_getElapsedTime() as StgWord64;

    if doingRetainerProfiling() {
        retainerProfile();
    }

    stat_startHeapCensus();
    g = 0;

    while g < RtsFlags.GcFlags.generations {
        heapCensusChain(census, (*generations.offset(g as isize)).blocks);
        heapCensusChain(census, (*generations.offset(g as isize)).large_objects);
        heapCensusCompactList(census, (*generations.offset(g as isize)).compact_objects);
        n = 0;

        while n < getNumCapabilities() as u32 {
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
        let mut i = 0;

        while i < nonmoving_alloca_cnt as u32 {
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

            let mut j = 0;

            while j < getNumCapabilities() {
                let mut cap = getCapability(j as u32);
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
