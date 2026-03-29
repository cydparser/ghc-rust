use crate::arena::{Arena, arenaAlloc, arenaFree, newArena};
use crate::eventlog::event_log::postInitEvent;
use crate::ffi::rts::flags::{COST_CENTRES_ALL, COST_CENTRES_JSON, RtsFlags};
use crate::ffi::rts::messages::debugBelch;
use crate::ffi::rts::prof::ccs::{
    CostCentre, CostCentre_, CostCentreStack, CostCentreStack_, IndexTable, IndexTable_,
    startProfTimer, stopProfTimer,
};
use crate::ffi::rts::prog_name;
use crate::ffi::rts::storage::closure_macros::{
    UNTAG_CONST_CLOSURE, doingRetainerProfiling, get_itbl, itbl_to_con_itbl, stack_frame_sizeW,
};
use crate::ffi::rts::storage::closures::{StgUnderflowFrame, StgUpdateFrame};
use crate::ffi::rts::storage::tso::StgStack;
use crate::ffi::rts::types::{StgClosure, StgTSO};
use crate::ffi::stg::regs::StgRegTable;
use crate::ffi::stg::types::{StgBool, StgInt, StgWord, StgWord32, StgWord64};
use crate::fs::__rts_fopen;
use crate::prelude::*;
use crate::printer::closure_type_names;
use crate::prof_heap::strMatchesSelector;
use crate::profiler_report::writeCCSReport;
use crate::profiler_report_json::writeCCSReportJson;
use crate::profiling::ProfilerTotals;
use crate::rts_utils::stgMallocBytes;
use crate::trace::{traceHeapProfCostCentre, traceProfBegin};

#[cfg(test)]
mod tests;

/// cbindgen:no-export
pub(crate) struct ProfilerTotals {
    pub(crate) total_alloc: u64,
    pub(crate) total_prof_ticks: u32,
}

static mut prof_arena: *mut Arena = null_mut::<Arena>();

static mut CC_ID: u32 = 1;

static mut CCS_ID: u32 = 1;

static mut DUMPED_CC_ID: u32 = 0;

static mut prof_filename: *mut c_char = null_mut::<c_char>();

static mut prof_file: *mut FILE = null_mut::<FILE>();

static mut CC_LIST: *mut CostCentre = null_mut::<CostCentre>();

static mut CCS_LIST: *mut CostCentreStack = null_mut::<CostCentreStack>();

static mut CC_MAIN: [CostCentre; 1] = [CostCentre_ {
    ccID: 0,
    label: c"MAIN".as_ptr(),
    module: c"MAIN".as_ptr(),
    srcloc: c"<built-in>".as_ptr(),
    mem_alloc: 0,
    time_ticks: 0,
    is_caf: 0,
    link: null_mut::<CostCentre_>(),
}];

static mut CC_SYSTEM: [CostCentre; 1] = [CostCentre_ {
    ccID: 0,
    label: c"SYSTEM".as_ptr(),
    module: c"SYSTEM".as_ptr(),
    srcloc: c"<built-in>".as_ptr(),
    mem_alloc: 0,
    time_ticks: 0,
    is_caf: 0,
    link: null_mut::<CostCentre_>(),
}];

static mut CC_GC: [CostCentre; 1] = [CostCentre_ {
    ccID: 0,
    label: c"GC".as_ptr(),
    module: c"GC".as_ptr(),
    srcloc: c"<built-in>".as_ptr(),
    mem_alloc: 0,
    time_ticks: 0,
    is_caf: 0,
    link: null_mut::<CostCentre_>(),
}];

static mut CC_OVERHEAD: [CostCentre; 1] = [CostCentre_ {
    ccID: 0,
    label: c"OVERHEAD_of".as_ptr(),
    module: c"PROFILING".as_ptr(),
    srcloc: c"<built-in>".as_ptr(),
    mem_alloc: 0,
    time_ticks: 0,
    is_caf: 0,
    link: null_mut::<CostCentre_>(),
}];

static mut CC_DONT_CARE: [CostCentre; 1] = [CostCentre_ {
    ccID: 0,
    label: c"DONT_CARE".as_ptr(),
    module: c"MAIN".as_ptr(),
    srcloc: c"<built-in>".as_ptr(),
    mem_alloc: 0,
    time_ticks: 0,
    is_caf: 0,
    link: null_mut::<CostCentre_>(),
}];

static mut CC_PINNED: [CostCentre; 1] = [CostCentre_ {
    ccID: 0,
    label: c"PINNED".as_ptr(),
    module: c"SYSTEM".as_ptr(),
    srcloc: c"<built-in>".as_ptr(),
    mem_alloc: 0,
    time_ticks: 0,
    is_caf: 0,
    link: null_mut::<CostCentre_>(),
}];

static mut CC_IDLE: [CostCentre; 1] = [CostCentre_ {
    ccID: 0,
    label: c"IDLE".as_ptr(),
    module: c"IDLE".as_ptr(),
    srcloc: c"<built-in>".as_ptr(),
    mem_alloc: 0,
    time_ticks: 0,
    is_caf: 0,
    link: null_mut::<CostCentre_>(),
}];

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
pub static mut CCS_MAIN: [CostCentreStack; 1] = unsafe {
    [CostCentreStack_ {
        ccsID: 0,
        cc: &raw const CC_MAIN as *mut CostCentre,
        prevStack: null_mut::<CostCentreStack_>(),
        indexTable: null_mut::<IndexTable_>(),
        root: null_mut::<CostCentreStack_>(),
        depth: 0,
        scc_count: 0,
        selected: 0,
        time_ticks: 0,
        mem_alloc: 0,
        inherited_alloc: 0,
        inherited_ticks: 0,
    }]
};

#[ffi(compiler, ghc_lib)]
#[unsafe(no_mangle)]
pub static mut CCS_SYSTEM: [CostCentreStack; 1] = unsafe {
    [CostCentreStack_ {
        ccsID: 0,
        cc: &raw const CC_SYSTEM as *mut CostCentre,
        prevStack: null_mut::<CostCentreStack_>(),
        indexTable: null_mut::<IndexTable_>(),
        root: null_mut::<CostCentreStack_>(),
        depth: 0,
        scc_count: 0,
        selected: 0,
        time_ticks: 0,
        mem_alloc: 0,
        inherited_alloc: 0,
        inherited_ticks: 0,
    }]
};

static mut CCS_GC: [CostCentreStack; 1] = unsafe {
    [CostCentreStack_ {
        ccsID: 0,
        cc: &raw const CC_GC as *mut CostCentre,
        prevStack: null_mut::<CostCentreStack_>(),
        indexTable: null_mut::<IndexTable_>(),
        root: null_mut::<CostCentreStack_>(),
        depth: 0,
        scc_count: 0,
        selected: 0,
        time_ticks: 0,
        mem_alloc: 0,
        inherited_alloc: 0,
        inherited_ticks: 0,
    }]
};

static mut CCS_OVERHEAD: [CostCentreStack; 1] = unsafe {
    [CostCentreStack_ {
        ccsID: 0,
        cc: &raw const CC_OVERHEAD as *mut CostCentre,
        prevStack: null_mut::<CostCentreStack_>(),
        indexTable: null_mut::<IndexTable_>(),
        root: null_mut::<CostCentreStack_>(),
        depth: 0,
        scc_count: 0,
        selected: 0,
        time_ticks: 0,
        mem_alloc: 0,
        inherited_alloc: 0,
        inherited_ticks: 0,
    }]
};

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut CCS_DONT_CARE: [CostCentreStack; 1] = unsafe {
    [CostCentreStack_ {
        ccsID: 0,
        cc: &raw const CC_DONT_CARE as *mut CostCentre,
        prevStack: null_mut::<CostCentreStack_>(),
        indexTable: null_mut::<IndexTable_>(),
        root: null_mut::<CostCentreStack_>(),
        depth: 0,
        scc_count: 0,
        selected: 0,
        time_ticks: 0,
        mem_alloc: 0,
        inherited_alloc: 0,
        inherited_ticks: 0,
    }]
};

static mut CCS_PINNED: [CostCentreStack; 1] = unsafe {
    [CostCentreStack_ {
        ccsID: 0,
        cc: &raw const CC_PINNED as *mut CostCentre,
        prevStack: null_mut::<CostCentreStack_>(),
        indexTable: null_mut::<IndexTable_>(),
        root: null_mut::<CostCentreStack_>(),
        depth: 0,
        scc_count: 0,
        selected: 0,
        time_ticks: 0,
        mem_alloc: 0,
        inherited_alloc: 0,
        inherited_ticks: 0,
    }]
};

static mut CCS_IDLE: [CostCentreStack; 1] = unsafe {
    [CostCentreStack_ {
        ccsID: 0,
        cc: &raw const CC_IDLE as *mut CostCentre,
        prevStack: null_mut::<CostCentreStack_>(),
        indexTable: null_mut::<IndexTable_>(),
        root: null_mut::<CostCentreStack_>(),
        depth: 0,
        scc_count: 0,
        selected: 0,
        time_ticks: 0,
        mem_alloc: 0,
        inherited_alloc: 0,
        inherited_ticks: 0,
    }]
};

unsafe fn dumpCostCentresToEventLog() {
    let mut cc = null_mut::<CostCentre>();
    let mut next = null_mut::<CostCentre>();
    cc = CC_LIST;

    while !cc.is_null() && (*cc).ccID != DUMPED_CC_ID as StgInt {
        next = (*cc).link as *mut CostCentre;

        traceHeapProfCostCentre(
            (*cc).ccID as StgWord32,
            (*cc).label,
            (*cc).module,
            (*cc).srcloc,
            (*cc).is_caf,
        );

        DUMPED_CC_ID = (if (*cc).ccID < DUMPED_CC_ID as StgInt {
            DUMPED_CC_ID as StgInt
        } else {
            (*cc).ccID
        }) as u32;

        cc = next;
    }
}

unsafe fn initProfiling() {
    prof_arena = newArena();
    initProfilingLogFile();
    registerCC(&raw mut CC_MAIN as *mut CostCentre);
    registerCC(&raw mut CC_SYSTEM as *mut CostCentre);
    registerCC(&raw mut CC_GC as *mut CostCentre);
    registerCC(&raw mut CC_OVERHEAD as *mut CostCentre);
    registerCC(&raw mut CC_DONT_CARE as *mut CostCentre);
    registerCC(&raw mut CC_PINNED as *mut CostCentre);
    registerCC(&raw mut CC_IDLE as *mut CostCentre);
    registerCCS(&raw mut CCS_SYSTEM as *mut CostCentreStack);
    registerCCS(&raw mut CCS_GC as *mut CostCentreStack);
    registerCCS(&raw mut CCS_OVERHEAD as *mut CostCentreStack);
    registerCCS(&raw mut CCS_DONT_CARE as *mut CostCentreStack);
    registerCCS(&raw mut CCS_PINNED as *mut CostCentreStack);
    registerCCS(&raw mut CCS_IDLE as *mut CostCentreStack);
    registerCCS(&raw mut CCS_MAIN as *mut CostCentreStack);
    CCS_LIST = (*CCS_LIST).prevStack as *mut CostCentreStack;

    let ref mut fresh8 = (*(&raw mut CCS_MAIN as *mut CostCentreStack)).prevStack;
    *fresh8 = null_mut::<CostCentreStack_>();

    let ref mut fresh9 = (*(&raw mut CCS_MAIN as *mut CostCentreStack)).root;
    *fresh9 = &raw mut CCS_MAIN as *mut CostCentreStack as *mut CostCentreStack_;
    ccsSetSelected(&raw mut CCS_MAIN as *mut CostCentreStack);
    refreshProfilingCCSs();

    if RtsFlags.CcFlags.doCostCentres != 0 {
        initTimeProfiling();
    }
}

unsafe fn refreshProfilingCCSs() {
    postInitEvent(Some(
        dumpCostCentresToEventLog as unsafe extern "C" fn() -> (),
    ));

    let mut next = null_mut::<CostCentreStack>();
    let mut ccs = CCS_LIST;

    while !ccs.is_null() {
        next = (*ccs).prevStack as *mut CostCentreStack;
        (*ccs).prevStack = null_mut::<CostCentreStack_>();
        actualPush_(&raw mut CCS_MAIN as *mut CostCentreStack, (*ccs).cc, ccs);
        (*ccs).root = ccs as *mut CostCentreStack_;
        ccs = next;
    }

    CCS_LIST = null_mut::<CostCentreStack>();
}

unsafe fn freeProfiling() {
    arenaFree(prof_arena);
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn mkCostCentre(
    mut label: *mut c_char,
    mut module: *mut c_char,
    mut srcloc: *mut c_char,
) -> *mut CostCentre {
    let mut cc = stgMallocBytes(size_of::<CostCentre>() as usize, c"mkCostCentre".as_ptr())
        as *mut CostCentre;

    (*cc).label = label;
    (*cc).module = module;
    (*cc).srcloc = srcloc;
    (*cc).is_caf = 0;
    (*cc).mem_alloc = 0;
    (*cc).time_ticks = 0;
    (*cc).link = null_mut::<CostCentre_>();

    return cc;
}

unsafe fn initProfilingLogFile() {
    let mut stem = null::<c_char>();

    if !RtsFlags.CcFlags.outputFileNameStem.is_null() {
        stem = RtsFlags.CcFlags.outputFileNameStem;
    } else {
        let mut prog =
            arenaAlloc(prof_arena, strlen(prog_name).wrapping_add(1 as usize)) as *mut c_char;
        strcpy(prog, prog_name);
        stem = prog;
    }

    if RtsFlags.CcFlags.doCostCentres == 0 && !doingRetainerProfiling() {
        prof_filename = null_mut::<c_char>();
        prof_file = null_mut::<FILE>();
    } else {
        prof_filename =
            arenaAlloc(prof_arena, strlen(stem).wrapping_add(6 as usize)) as *mut c_char;
        sprintf(prof_filename, c"%s.prof".as_ptr(), stem);
        prof_file = __rts_fopen(prof_filename, c"w+".as_ptr());

        if prof_file.is_null() {
            debugBelch(
                c"Can't open profiling report file %s\n".as_ptr(),
                prof_filename,
            );
            RtsFlags.CcFlags.doCostCentres = 0;

            if doingRetainerProfiling() {
                RtsFlags.ProfFlags.doHeapProfile = 0;
            }
        }
    };
}

unsafe fn initTimeProfiling() {
    traceProfBegin();

    if RtsFlags.ProfFlags.startTimeProfileAtStartup {
        startProfTimer();
    }
}

unsafe fn endProfiling() {
    if RtsFlags.CcFlags.doCostCentres != 0 {
        stopProfTimer();
    }
}

unsafe fn registerCC(mut cc: *mut CostCentre) {
    if (*cc).link.is_null() {
        (*cc).link = CC_LIST as *mut CostCentre_;
        CC_LIST = cc;

        let fresh6 = CC_ID;
        CC_ID = CC_ID.wrapping_add(1);
        (*cc).ccID = fresh6 as StgInt;
    }
}

unsafe fn registerCCS(mut ccs: *mut CostCentreStack) {
    if (*ccs).prevStack.is_null() {
        (*ccs).prevStack = CCS_LIST as *mut CostCentreStack_;
        CCS_LIST = ccs;

        let fresh7 = CCS_ID;
        CCS_ID = CCS_ID.wrapping_add(1);
        (*ccs).ccsID = fresh7 as StgInt;
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn registerCcList(mut cc_list: *mut *mut CostCentre) {
    let mut i = cc_list;

    while !(*i).is_null() {
        registerCC(*i);
        i = i.offset(1);
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn registerCcsList(mut cc_list: *mut *mut CostCentreStack) {
    let mut i = cc_list;

    while !(*i).is_null() {
        registerCCS(*i);
        i = i.offset(1);
    }
}

unsafe fn enterFunEqualStacks(
    mut ccs0: *mut CostCentreStack,
    mut ccsapp: *mut CostCentreStack,
    mut ccsfn: *mut CostCentreStack,
) -> *mut CostCentreStack {
    if ccsapp == ccsfn {
        return ccs0;
    }

    return pushCostCentre(
        enterFunEqualStacks(
            ccs0,
            (*ccsapp).prevStack as *mut CostCentreStack,
            (*ccsfn).prevStack as *mut CostCentreStack,
        ),
        (*ccsfn).cc,
    );
}

unsafe fn enterFunCurShorter(
    mut ccsapp: *mut CostCentreStack,
    mut ccsfn: *mut CostCentreStack,
    mut n: StgWord,
) -> *mut CostCentreStack {
    if n == 0 {
        return enterFunEqualStacks(ccsapp, ccsapp, ccsfn);
    } else {
        return pushCostCentre(
            enterFunCurShorter(
                ccsapp,
                (*ccsfn).prevStack as *mut CostCentreStack,
                n.wrapping_sub(1 as StgWord),
            ),
            (*ccsfn).cc,
        );
    };
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn enterFunCCS(mut reg: *mut StgRegTable, mut ccsfn: *mut CostCentreStack) {
    let mut ccsapp = null_mut::<CostCentreStack>();

    if ccsfn == (*reg).rCCCS {
        return;
    }

    if (*ccsfn).cc.is_null() || (*(*ccsfn).cc).is_caf != 0 {
        return;
    }

    ccsapp = (*reg).rCCCS as *mut CostCentreStack;
    (*reg).rCCCS = &raw mut CCS_OVERHEAD as *mut CostCentreStack as *mut CostCentreStack_;

    if (*ccsfn).root != (*ccsapp).root {
        (*reg).rCCCS = appendCCS(ccsapp, ccsfn) as *mut CostCentreStack_;
        return;
    }

    if (*ccsapp).depth > (*ccsfn).depth {
        let mut i: u32 = 0;
        let mut n: u32 = 0;
        let mut tmp = ccsapp;
        n = (*ccsapp).depth.wrapping_sub((*ccsfn).depth) as u32;
        i = 0;

        while i < n {
            tmp = (*tmp).prevStack as *mut CostCentreStack;
            i = i.wrapping_add(1);
        }

        (*reg).rCCCS = enterFunEqualStacks(ccsapp, tmp, ccsfn) as *mut CostCentreStack_;
        return;
    }

    if (*ccsfn).depth > (*ccsapp).depth {
        (*reg).rCCCS =
            enterFunCurShorter(ccsapp, ccsfn, (*ccsfn).depth.wrapping_sub((*ccsapp).depth))
                as *mut CostCentreStack_;

        return;
    }

    (*reg).rCCCS = enterFunEqualStacks(ccsapp, ccsapp, ccsfn) as *mut CostCentreStack_;
}

unsafe fn ccsSetSelected(mut ccs: *mut CostCentreStack) {
    if !RtsFlags.ProfFlags.modSelector.is_null() {
        if !strMatchesSelector((*(*ccs).cc).module, RtsFlags.ProfFlags.modSelector) {
            (*ccs).selected = 0;
            return;
        }
    }

    if !RtsFlags.ProfFlags.ccSelector.is_null() {
        if !strMatchesSelector((*(*ccs).cc).label, RtsFlags.ProfFlags.ccSelector) {
            (*ccs).selected = 0;
            return;
        }
    }

    if !RtsFlags.ProfFlags.ccsSelector.is_null() {
        let mut c = null_mut::<CostCentreStack>();
        c = ccs;

        while !c.is_null() {
            if strMatchesSelector((*(*c).cc).label, RtsFlags.ProfFlags.ccsSelector) {
                break;
            }

            c = (*c).prevStack as *mut CostCentreStack;
        }

        if c.is_null() {
            (*ccs).selected = 0;
            return;
        }
    }

    (*ccs).selected = 1;
}

unsafe fn appendCCS(
    mut ccs1: *mut CostCentreStack,
    mut ccs2: *mut CostCentreStack,
) -> *mut CostCentreStack {
    if ccs1 == ccs2 {
        return ccs1;
    }

    if ccs2 == &raw mut CCS_MAIN as *mut CostCentreStack || (*(*ccs2).cc).is_caf != 0 {
        return ccs1;
    }

    return pushCostCentre(
        appendCCS(ccs1, (*ccs2).prevStack as *mut CostCentreStack),
        (*ccs2).cc,
    );
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn pushCostCentre(
    mut ccs: *mut CostCentreStack,
    mut cc: *mut CostCentre,
) -> *mut CostCentreStack {
    let mut ret = null_mut::<CostCentreStack>();

    if ccs.is_null() {
        ret = actualPush(ccs, cc);
    } else if (*ccs).cc == cc {
        return ccs;
    } else {
        let mut ixtable = (*ccs).indexTable as *mut IndexTable;
        let mut temp_ccs = isInIndexTable(ixtable, cc);

        if !temp_ccs.is_null() {
            return temp_ccs;
        } else {
            if (*ccs).indexTable != ixtable {
                temp_ccs = isInIndexTable(ixtable, cc);

                if !temp_ccs.is_null() {
                    return temp_ccs;
                }
            }

            temp_ccs = checkLoop(ccs, cc);

            if !temp_ccs.is_null() {
                let mut new_ccs = null_mut::<CostCentreStack>();
                new_ccs = temp_ccs;

                (*ccs).indexTable =
                    addToIndexTable((*ccs).indexTable as *mut IndexTable, new_ccs, cc, true)
                        as *mut IndexTable_;

                ret = new_ccs;
            } else {
                ret = actualPush(ccs, cc);
            }
        }
    }

    return ret;
}

unsafe fn checkLoop(
    mut ccs: *mut CostCentreStack,
    mut cc: *mut CostCentre,
) -> *mut CostCentreStack {
    while !ccs.is_null() {
        if (*ccs).cc == cc {
            return ccs;
        }

        ccs = (*ccs).prevStack as *mut CostCentreStack;
    }

    return null_mut::<CostCentreStack>();
}

unsafe fn actualPush(
    mut ccs: *mut CostCentreStack,
    mut cc: *mut CostCentre,
) -> *mut CostCentreStack {
    let mut new_ccs = null_mut::<CostCentreStack>();
    new_ccs = arenaAlloc(prof_arena, size_of::<CostCentreStack>() as usize) as *mut CostCentreStack;

    return actualPush_(ccs, cc, new_ccs);
}

unsafe fn actualPush_(
    mut ccs: *mut CostCentreStack,
    mut cc: *mut CostCentre,
    mut new_ccs: *mut CostCentreStack,
) -> *mut CostCentreStack {
    let fresh1 = CCS_ID;
    CCS_ID = CCS_ID.wrapping_add(1);
    (*new_ccs).ccsID = fresh1 as StgInt;
    (*new_ccs).cc = cc;
    (*new_ccs).prevStack = ccs as *mut CostCentreStack_;
    (*new_ccs).root = (*ccs).root;
    (*new_ccs).depth = (*ccs).depth.wrapping_add(1 as StgWord);
    (*new_ccs).indexTable = null_mut::<IndexTable_>();
    (*new_ccs).scc_count = 0;
    (*new_ccs).time_ticks = 0;
    (*new_ccs).mem_alloc = 0;
    (*new_ccs).inherited_ticks = 0;
    (*new_ccs).inherited_alloc = 0;
    ccsSetSelected(new_ccs);

    (*ccs).indexTable = addToIndexTable((*ccs).indexTable as *mut IndexTable, new_ccs, cc, false)
        as *mut IndexTable_;

    return new_ccs;
}

unsafe fn isInIndexTable(mut it: *mut IndexTable, mut cc: *mut CostCentre) -> *mut CostCentreStack {
    while !it.is_null() {
        if (*it).cc == cc {
            return (*it).ccs;
        } else {
            it = (*it).next as *mut IndexTable;
        }
    }

    return null_mut::<CostCentreStack>();
}

unsafe fn addToIndexTable(
    mut it: *mut IndexTable,
    mut new_ccs: *mut CostCentreStack,
    mut cc: *mut CostCentre,
    mut back_edge: bool,
) -> *mut IndexTable {
    let mut new_it = null_mut::<IndexTable>();
    new_it = arenaAlloc(prof_arena, size_of::<IndexTable>() as usize) as *mut IndexTable;
    (*new_it).cc = cc;
    (*new_it).ccs = new_ccs;
    (*new_it).next = it as *mut IndexTable_;
    (*new_it).back_edge = back_edge;

    return new_it;
}

unsafe fn ignoreCC(mut cc: *const CostCentre) -> bool {
    return RtsFlags.CcFlags.doCostCentres < COST_CENTRES_ALL as u32
        && (cc == &raw mut CC_OVERHEAD as *mut CostCentre as *const CostCentre
            || cc == &raw mut CC_DONT_CARE as *mut CostCentre as *const CostCentre
            || cc == &raw mut CC_GC as *mut CostCentre as *const CostCentre
            || cc == &raw mut CC_SYSTEM as *mut CostCentre as *const CostCentre
            || cc == &raw mut CC_IDLE as *mut CostCentre as *const CostCentre);
}

unsafe fn ignoreCCS(mut ccs: *const CostCentreStack) -> bool {
    return RtsFlags.CcFlags.doCostCentres < COST_CENTRES_ALL as u32
        && (ccs == &raw mut CCS_OVERHEAD as *mut CostCentreStack as *const CostCentreStack
            || ccs == &raw mut CCS_DONT_CARE as *mut CostCentreStack as *const CostCentreStack
            || ccs == &raw mut CCS_GC as *mut CostCentreStack as *const CostCentreStack
            || ccs == &raw mut CCS_SYSTEM as *mut CostCentreStack as *const CostCentreStack
            || ccs == &raw mut CCS_IDLE as *mut CostCentreStack as *const CostCentreStack);
}

unsafe fn reportCCSProfiling() {
    stopProfTimer();

    if RtsFlags.CcFlags.doCostCentres == 0 {
        return;
    }

    let mut totals = countTickss(&raw mut CCS_MAIN as *mut CostCentreStack);
    aggregateCCCosts(&raw mut CCS_MAIN as *mut CostCentreStack);
    inheritCosts(&raw mut CCS_MAIN as *mut CostCentreStack);

    let mut stack = pruneCCSTree(&raw mut CCS_MAIN as *mut CostCentreStack);
    sortCCSTree(stack);

    if RtsFlags.CcFlags.doCostCentres == COST_CENTRES_JSON as u32 {
        writeCCSReportJson(prof_file, stack, totals);
    } else {
        writeCCSReport(prof_file, stack, totals);
    };
}

unsafe fn countTickss_(mut ccs: *const CostCentreStack, mut totals: *mut ProfilerTotals) {
    if !ignoreCCS(ccs) {
        (*totals).total_alloc = (*totals).total_alloc.wrapping_add((*ccs).mem_alloc as u64);
        (*totals).total_prof_ticks =
            ((*totals).total_prof_ticks as StgWord).wrapping_add((*ccs).time_ticks) as u32 as u32;
    }

    let mut i = (*ccs).indexTable as *mut IndexTable;

    while !i.is_null() {
        if !(*i).back_edge {
            countTickss_((*i).ccs, totals);
        }

        i = (*i).next as *mut IndexTable;
    }
}

unsafe fn countTickss(mut ccs: *const CostCentreStack) -> ProfilerTotals {
    let mut totals = ProfilerTotals {
        total_alloc: 0,
        total_prof_ticks: 0,
    };

    countTickss_(ccs, &raw mut totals);

    return totals;
}

unsafe fn inheritCosts(mut ccs: *mut CostCentreStack) {
    let mut i = null_mut::<IndexTable>();

    if ignoreCCS(ccs) {
        return;
    }

    (*ccs).inherited_ticks = (*ccs).inherited_ticks.wrapping_add((*ccs).time_ticks);
    (*ccs).inherited_alloc = (*ccs).inherited_alloc.wrapping_add((*ccs).mem_alloc);
    i = (*ccs).indexTable as *mut IndexTable;

    while !i.is_null() {
        if !(*i).back_edge {
            inheritCosts((*i).ccs);
            (*ccs).inherited_ticks = (*ccs)
                .inherited_ticks
                .wrapping_add((*(*i).ccs).inherited_ticks);
            (*ccs).inherited_alloc = (*ccs)
                .inherited_alloc
                .wrapping_add((*(*i).ccs).inherited_alloc);
        }

        i = (*i).next as *mut IndexTable;
    }
}

unsafe fn aggregateCCCosts(mut ccs: *mut CostCentreStack) {
    let mut i = null_mut::<IndexTable>();
    (*(*ccs).cc).mem_alloc = (*(*ccs).cc).mem_alloc.wrapping_add((*ccs).mem_alloc);
    (*(*ccs).cc).time_ticks = (*(*ccs).cc).time_ticks.wrapping_add((*ccs).time_ticks);
    i = (*ccs).indexTable as *mut IndexTable;

    while !i.is_null() {
        if !(*i).back_edge {
            aggregateCCCosts((*i).ccs);
        }

        i = (*i).next as *mut IndexTable;
    }
}

unsafe fn pruneCCSTree(mut ccs: *mut CostCentreStack) -> *mut CostCentreStack {
    let mut ccs1 = null_mut::<CostCentreStack>();
    let mut i = null_mut::<IndexTable>();
    let mut prev = null_mut::<*mut IndexTable>();
    prev = &raw mut (*ccs).indexTable as *mut *mut IndexTable;
    i = (*ccs).indexTable as *mut IndexTable;

    while !i.is_null() {
        if !(*i).back_edge {
            ccs1 = pruneCCSTree((*i).ccs);

            if ccs1.is_null() {
                *prev = (*i).next as *mut IndexTable;
            } else {
                prev = &raw mut (*i).next as *mut *mut IndexTable;
            }
        }

        i = (*i).next as *mut IndexTable;
    }

    if RtsFlags.CcFlags.doCostCentres >= COST_CENTRES_ALL as u32
        || !(*ccs).indexTable.is_null()
        || ((*ccs).scc_count != 0 || (*ccs).time_ticks != 0 || (*ccs).mem_alloc != 0)
    {
        return ccs;
    } else {
        return null_mut::<CostCentreStack>();
    };
}

unsafe fn insertIndexTableInSortedList(
    mut tbl: *mut IndexTable,
    mut sortedList: *mut IndexTable,
) -> *mut IndexTable {
    let mut tbl_ticks: StgWord = (*(*tbl).ccs).scc_count as StgWord;
    let mut tbl_label = (*(*(*tbl).ccs).cc).label;
    let mut prev = null_mut::<IndexTable>();
    let mut cursor = sortedList;

    while !cursor.is_null() {
        let mut cursor_ticks: StgWord = (*(*cursor).ccs).scc_count as StgWord;
        let mut cursor_label = (*(*(*cursor).ccs).cc).label;

        if tbl_ticks > cursor_ticks
            || tbl_ticks == cursor_ticks && strcmp(tbl_label, cursor_label) < 0
        {
            if prev.is_null() {
                (*tbl).next = sortedList as *mut IndexTable_;

                return tbl;
            } else {
                (*prev).next = tbl as *mut IndexTable_;
                (*tbl).next = cursor as *mut IndexTable_;

                return sortedList;
            }
        } else {
            prev = cursor;
            cursor = (*cursor).next as *mut IndexTable;
        }
    }

    (*prev).next = tbl as *mut IndexTable_;

    return sortedList;
}

unsafe fn sortCCSTree(mut ccs: *mut CostCentreStack) {
    if (*ccs).indexTable.is_null() {
        return;
    }

    let mut tbl = (*ccs).indexTable as *mut IndexTable;

    while !tbl.is_null() {
        if !(*tbl).back_edge {
            sortCCSTree((*tbl).ccs);
        }

        tbl = (*tbl).next as *mut IndexTable;
    }

    let mut sortedList = (*ccs).indexTable as *mut IndexTable;
    let mut nonSortedList = (*sortedList).next as *mut IndexTable;
    (*sortedList).next = null_mut::<IndexTable_>();

    while !nonSortedList.is_null() {
        let mut nonSortedTail = (*nonSortedList).next as *mut IndexTable;
        (*nonSortedList).next = null_mut::<IndexTable_>();
        sortedList = insertIndexTableInSortedList(nonSortedList, sortedList);
        nonSortedList = nonSortedTail;
    }

    (*ccs).indexTable = sortedList as *mut IndexTable_;
}

unsafe fn fprintCCS(mut f: *mut FILE, mut ccs: *mut CostCentreStack) {
    fprintf(f, c"<".as_ptr());

    while !ccs.is_null() && ccs != &raw mut CCS_MAIN as *mut CostCentreStack {
        fprintf(
            f,
            c"%s.%s".as_ptr(),
            (*(*ccs).cc).module,
            (*(*ccs).cc).label,
        );

        if !(*ccs).prevStack.is_null()
            && (*ccs).prevStack != &raw mut CCS_MAIN as *mut CostCentreStack
        {
            fprintf(f, c",".as_ptr());
        }

        ccs = (*ccs).prevStack as *mut CostCentreStack;
    }

    fprintf(f, c">".as_ptr());
}

unsafe fn fprintCallStack(mut ccs: *mut CostCentreStack) -> bool {
    let mut prev = null_mut::<CostCentreStack>();
    fprintf(
        __stderrp,
        c"%s.%s".as_ptr(),
        (*(*ccs).cc).module,
        (*(*ccs).cc).label,
    );
    prev = (*ccs).prevStack as *mut CostCentreStack;

    while !prev.is_null() && prev != &raw mut CCS_MAIN as *mut CostCentreStack {
        ccs = prev;

        fprintf(
            __stderrp,
            c",\n  called from %s.%s".as_ptr(),
            (*(*ccs).cc).module,
            (*(*ccs).cc).label,
        );

        prev = (*ccs).prevStack as *mut CostCentreStack;
    }

    fprintf(__stderrp, c"\n".as_ptr());

    return strncmp((*(*ccs).cc).label, c"CAF".as_ptr(), 3) == 0;
}

unsafe fn fprintCCS_stderr(
    mut ccs: *mut CostCentreStack,
    mut exception: *mut StgClosure,
    mut tso: *mut StgTSO,
) {
    let mut is_caf: bool = false;
    let mut frame = null_mut::<StgWord>();
    let mut stack = null_mut::<StgStack>();
    let mut prev_ccs = null_mut::<CostCentreStack>();
    let mut depth: u32 = 0;
    let MAX_DEPTH: u32 = 10;
    let mut desc = null::<c_char>();
    let mut info = null::<StgInfoTable>();
    info = get_itbl(UNTAG_CONST_CLOSURE(exception));

    match (*info).r#type {
        1 | 2 | 3 | 4 | 5 | 6 | 7 => {
            desc = (itbl_to_con_itbl(info).offset(1 as i32 as isize) as StgWord)
                .wrapping_add((*itbl_to_con_itbl(info)).con_desc as StgWord)
                as *const c_char;
        }
        _ => {
            desc = *(&raw mut closure_type_names as *mut *const c_char)
                .offset((*info).r#type as isize);
        }
    }

    fprintf(
        __stderrp,
        c"*** Exception (reporting due to +RTS -xc): (%s), stack trace: \n  ".as_ptr(),
        desc,
    );

    is_caf = fprintCallStack(ccs);
    stack = (*tso).stackobj as *mut StgStack;
    frame = (*stack).sp;
    prev_ccs = ccs;

    while is_caf as i32 != 0 && depth < MAX_DEPTH {
        match (*get_itbl(frame as *mut StgClosure)).r#type {
            33 => {
                ccs = (*(frame as *mut StgUpdateFrame)).header.prof.ccs;
                frame = frame.offset(
                    (size_of::<StgUpdateFrame>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as isize,
                );

                if ccs == &raw mut CCS_MAIN as *mut CostCentreStack {
                    break;
                }

                if !(ccs == prev_ccs) {
                    prev_ccs = ccs;
                    fprintf(__stderrp, c"  --> evaluated by: ".as_ptr());
                    is_caf = fprintCallStack(ccs);
                }
            }
            35 => {
                stack = (*(frame as *mut StgUnderflowFrame)).next_chunk as *mut StgStack;
                frame = (*stack).sp;
            }
            36 => {
                break;
            }
            _ => {
                frame = frame.offset(stack_frame_sizeW(frame as *mut StgClosure) as isize);
            }
        }

        depth = depth.wrapping_add(1);
    }
}
