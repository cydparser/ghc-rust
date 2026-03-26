use crate::ffi::rts::flags::{COST_CENTRES_ALL, COST_CENTRES_VERBOSE, RtsFlags, rts_argv};
use crate::ffi::rts::prof::ccs::{CC_LIST, CostCentre, CostCentre_, CostCentreStack, IndexTable};
use crate::ffi::rts::threads::getNumCapabilities;
use crate::ffi::rts::time::{TIME_RESOLUTION, Time};
use crate::ffi::rts::{prog_argv, prog_name};
use crate::ffi::stg::types::{StgFloat, StgInt, StgWord, StgWord64};
use crate::prelude::*;
use crate::profiling::{ProfilerTotals, ignoreCC, ignoreCCS};
use crate::rts_utils::{showStgWord64, time_str};

unsafe fn strlen_utf8(mut s: *mut c_char) -> uint32_t {
    let mut n: uint32_t = 0 as uint32_t;
    let mut c: c_uchar = 0;

    while *s as c_int != '\0' as i32 {
        c = *s as c_uchar;

        if (c as c_int) < 0x80 as c_int || c as c_int > 0xbf as c_int {
            n = n.wrapping_add(1);
        }

        s = s.offset(1);
    }

    return n;
}

unsafe fn fprintHeader(
    mut prof_file: *mut FILE,
    mut max_label_len: uint32_t,
    mut max_module_len: uint32_t,
    mut max_src_len: uint32_t,
    mut max_id_len: uint32_t,
) {
    fprintf(
        prof_file,
        b"%-*s %-*s %-*s %-*s %11s  %12s   %12s\n\0" as *const u8 as *const c_char,
        max_label_len,
        b"\0" as *const u8 as *const c_char,
        max_module_len,
        b"\0" as *const u8 as *const c_char,
        max_src_len,
        b"\0" as *const u8 as *const c_char,
        max_id_len,
        b"\0" as *const u8 as *const c_char,
        b"\0" as *const u8 as *const c_char,
        b"individual\0" as *const u8 as *const c_char,
        b"inherited\0" as *const u8 as *const c_char,
    );

    fprintf(
        prof_file,
        b"%-*s %-*s %-*s %-*s\0" as *const u8 as *const c_char,
        max_label_len,
        b"COST CENTRE\0" as *const u8 as *const c_char,
        max_module_len,
        b"MODULE\0" as *const u8 as *const c_char,
        max_src_len,
        b"SRC\0" as *const u8 as *const c_char,
        max_id_len,
        b"no.\0" as *const u8 as *const c_char,
    );

    fprintf(
        prof_file,
        b" %11s  %5s %6s   %5s %6s\0" as *const u8 as *const c_char,
        b"entries\0" as *const u8 as *const c_char,
        b"%time\0" as *const u8 as *const c_char,
        b"%alloc\0" as *const u8 as *const c_char,
        b"%time\0" as *const u8 as *const c_char,
        b"%alloc\0" as *const u8 as *const c_char,
    );

    if RtsFlags.CcFlags.doCostCentres >= COST_CENTRES_VERBOSE as uint32_t {
        fprintf(
            prof_file,
            b"  %5s %9s\0" as *const u8 as *const c_char,
            b"ticks\0" as *const u8 as *const c_char,
            b"bytes\0" as *const u8 as *const c_char,
        );
    }

    fprintf(prof_file, b"\n\n\0" as *const u8 as *const c_char);
}

static mut sorted_cc_list: *mut CostCentre = null::<CostCentre>() as *mut CostCentre;

unsafe fn insertCCInSortedList(mut new_cc: *mut CostCentre) {
    let mut prev = null_mut::<*mut CostCentre>();
    let mut cc = null_mut::<CostCentre>();
    prev = &raw mut sorted_cc_list;
    cc = sorted_cc_list;

    while !cc.is_null() {
        if (*new_cc).time_ticks > (*cc).time_ticks {
            (*new_cc).link = cc as *mut CostCentre_;
            *prev = new_cc;
            return;
        } else {
            prev = &raw mut (*cc).link as *mut *mut CostCentre;
        }

        cc = (*cc).link as *mut CostCentre;
    }

    (*new_cc).link = null_mut::<CostCentre_>();
    *prev = new_cc;
}

unsafe fn reportPerCCCosts(mut prof_file: *mut FILE, mut totals: ProfilerTotals) {
    let mut cc = null_mut::<CostCentre>();
    let mut next = null_mut::<CostCentre>();
    let mut max_label_len: uint32_t = 0;
    let mut max_module_len: uint32_t = 0;
    let mut max_src_len: uint32_t = 0;
    sorted_cc_list = null_mut::<CostCentre>();
    max_label_len = 11 as uint32_t;
    max_module_len = 6 as uint32_t;
    max_src_len = 3 as uint32_t;
    cc = CC_LIST;

    while !cc.is_null() {
        next = (*cc).link as *mut CostCentre;

        if (*cc).time_ticks > totals.total_prof_ticks.wrapping_div(100 as c_uint) as StgWord
            || (*cc).mem_alloc > totals.total_alloc.wrapping_div(100 as uint64_t)
            || RtsFlags.CcFlags.doCostCentres >= COST_CENTRES_ALL as uint32_t
        {
            insertCCInSortedList(cc);

            max_label_len = ({
                let mut _a = strlen_utf8((*cc).label) as uint32_t;
                let mut _b: uint32_t = max_label_len as uint32_t;

                if _a <= _b { _b } else { _a as uint32_t }
            });

            max_module_len = ({
                let mut _a = strlen_utf8((*cc).module) as uint32_t;
                let mut _b: uint32_t = max_module_len as uint32_t;

                if _a <= _b { _b } else { _a as uint32_t }
            });

            max_src_len = ({
                let mut _a = strlen_utf8((*cc).srcloc) as uint32_t;
                let mut _b: uint32_t = max_src_len as uint32_t;

                if _a <= _b { _b } else { _a as uint32_t }
            });
        }

        cc = next;
    }

    fprintf(
        prof_file,
        b"%-*s %-*s %-*s\0" as *const u8 as *const c_char,
        max_label_len,
        b"COST CENTRE\0" as *const u8 as *const c_char,
        max_module_len,
        b"MODULE\0" as *const u8 as *const c_char,
        max_src_len,
        b"SRC\0" as *const u8 as *const c_char,
    );

    fprintf(
        prof_file,
        b" %6s %6s\0" as *const u8 as *const c_char,
        b"%time\0" as *const u8 as *const c_char,
        b"%alloc\0" as *const u8 as *const c_char,
    );

    if RtsFlags.CcFlags.doCostCentres >= COST_CENTRES_VERBOSE as uint32_t {
        fprintf(
            prof_file,
            b"  %5s %9s\0" as *const u8 as *const c_char,
            b"ticks\0" as *const u8 as *const c_char,
            b"bytes\0" as *const u8 as *const c_char,
        );
    }

    fprintf(prof_file, b"\n\n\0" as *const u8 as *const c_char);
    cc = sorted_cc_list;

    while !cc.is_null() {
        if !ignoreCC(cc) {
            fprintf(
                prof_file,
                b"%s%*s %s%*s %s%*s\0" as *const u8 as *const c_char,
                (*cc).label,
                max_label_len.wrapping_sub(strlen_utf8((*cc).label)),
                b"\0" as *const u8 as *const c_char,
                (*cc).module,
                max_module_len.wrapping_sub(strlen_utf8((*cc).module)),
                b"\0" as *const u8 as *const c_char,
                (*cc).srcloc,
                max_src_len.wrapping_sub(strlen_utf8((*cc).srcloc)),
                b"\0" as *const u8 as *const c_char,
            );

            fprintf(
                prof_file,
                b" %6.1f %6.1f\0" as *const u8 as *const c_char,
                if totals.total_prof_ticks == 0 as c_uint {
                    0.0f64
                } else {
                    ((*cc).time_ticks as StgFloat / totals.total_prof_ticks as StgFloat
                        * 100 as c_int as StgFloat) as c_double
                },
                if totals.total_alloc == 0 as uint64_t {
                    0.0f64
                } else {
                    ((*cc).mem_alloc as StgFloat / totals.total_alloc as StgFloat
                        * 100 as c_int as StgFloat) as c_double
                },
            );

            if RtsFlags.CcFlags.doCostCentres >= COST_CENTRES_VERBOSE as uint32_t {
                fprintf(
                    prof_file,
                    b"  %5llu %9llu\0" as *const u8 as *const c_char,
                    (*cc).time_ticks,
                    (*cc).mem_alloc.wrapping_mul(size_of::<W_>() as StgWord64),
                );
            }

            fprintf(prof_file, b"\n\0" as *const u8 as *const c_char);
        }

        cc = (*cc).link as *mut CostCentre;
    }

    fprintf(prof_file, b"\n\n\0" as *const u8 as *const c_char);
}

unsafe fn numDigits(mut i: StgInt) -> uint32_t {
    let mut result: uint32_t = 0;
    result = 1 as uint32_t;

    if i < 0 as StgInt {
        i = 0 as StgInt;
    }

    while i > 9 as StgInt {
        i /= 10 as StgInt;
        result = result.wrapping_add(1);
    }

    return result;
}

unsafe fn findCCSMaxLens(
    mut ccs: *const CostCentreStack,
    mut indent: uint32_t,
    mut max_label_len: *mut uint32_t,
    mut max_module_len: *mut uint32_t,
    mut max_src_len: *mut uint32_t,
    mut max_id_len: *mut uint32_t,
) {
    let mut cc = null_mut::<CostCentre>();
    let mut i = null_mut::<IndexTable>();
    cc = (*ccs).cc;

    *max_label_len = ({
        let mut _a: uint32_t = *max_label_len;
        let mut _b: uint32_t =
            (indent as uint32_t).wrapping_add(strlen_utf8((*cc).label) as uint32_t);

        if _a <= _b { _b } else { _a as uint32_t }
    });

    *max_module_len = ({
        let mut _a: uint32_t = *max_module_len;
        let mut _b = strlen_utf8((*cc).module) as uint32_t;

        if _a <= _b { _b } else { _a as uint32_t }
    });

    *max_src_len = ({
        let mut _a: uint32_t = *max_src_len;
        let mut _b = strlen_utf8((*cc).srcloc) as uint32_t;

        if _a <= _b { _b } else { _a as uint32_t }
    });

    *max_id_len = ({
        let mut _a: uint32_t = *max_id_len;
        let mut _b = numDigits((*ccs).ccsID) as uint32_t;

        if _a <= _b { _b } else { _a as uint32_t }
    });

    i = (*ccs).indexTable as *mut IndexTable;

    while !i.is_null() {
        if !(*i).back_edge {
            findCCSMaxLens(
                (*i).ccs,
                indent.wrapping_add(1 as uint32_t),
                max_label_len,
                max_module_len,
                max_src_len,
                max_id_len,
            );
        }

        i = (*i).next as *mut IndexTable;
    }
}

unsafe fn logCCS(
    mut prof_file: *mut FILE,
    mut ccs: *const CostCentreStack,
    mut totals: ProfilerTotals,
    mut indent: uint32_t,
    mut max_label_len: uint32_t,
    mut max_module_len: uint32_t,
    mut max_src_len: uint32_t,
    mut max_id_len: uint32_t,
) {
    let mut cc = null_mut::<CostCentre>();
    let mut i = null_mut::<IndexTable>();
    cc = (*ccs).cc;

    if !ignoreCCS(ccs) {
        fprintf(
            prof_file,
            b"%*s%s%*s %s%*s %s%*s\0" as *const u8 as *const c_char,
            indent,
            b"\0" as *const u8 as *const c_char,
            (*cc).label,
            max_label_len
                .wrapping_sub(indent)
                .wrapping_sub(strlen_utf8((*cc).label)),
            b"\0" as *const u8 as *const c_char,
            (*cc).module,
            max_module_len.wrapping_sub(strlen_utf8((*cc).module)),
            b"\0" as *const u8 as *const c_char,
            (*cc).srcloc,
            max_src_len.wrapping_sub(strlen_utf8((*cc).srcloc)),
            b"\0" as *const u8 as *const c_char,
        );

        fprintf(
            prof_file,
            b" %*lld %11llu  %5.1f  %5.1f   %5.1f  %5.1f\0" as *const u8 as *const c_char,
            max_id_len,
            (*ccs).ccsID,
            (*ccs).scc_count,
            if totals.total_prof_ticks == 0 as c_uint {
                0.0f64
            } else {
                (*ccs).time_ticks as c_double / totals.total_prof_ticks as c_double * 100.0f64
            },
            if totals.total_alloc == 0 as uint64_t {
                0.0f64
            } else {
                (*ccs).mem_alloc as c_double / totals.total_alloc as c_double * 100.0f64
            },
            if totals.total_prof_ticks == 0 as c_uint {
                0.0f64
            } else {
                (*ccs).inherited_ticks as c_double / totals.total_prof_ticks as c_double * 100.0f64
            },
            if totals.total_alloc == 0 as uint64_t {
                0.0f64
            } else {
                (*ccs).inherited_alloc as c_double / totals.total_alloc as c_double * 100.0f64
            },
        );

        if RtsFlags.CcFlags.doCostCentres >= COST_CENTRES_VERBOSE as uint32_t {
            fprintf(
                prof_file,
                b"  %5llu %9llu\0" as *const u8 as *const c_char,
                (*ccs).time_ticks,
                (*ccs).mem_alloc.wrapping_mul(size_of::<W_>() as StgWord64),
            );
        }

        fprintf(prof_file, b"\n\0" as *const u8 as *const c_char);
    }

    i = (*ccs).indexTable as *mut IndexTable;

    while !i.is_null() {
        if !(*i).back_edge {
            logCCS(
                prof_file,
                (*i).ccs,
                totals,
                indent.wrapping_add(1 as uint32_t),
                max_label_len,
                max_module_len,
                max_src_len,
                max_id_len,
            );
        }

        i = (*i).next as *mut IndexTable;
    }
}

unsafe fn reportCCS(
    mut prof_file: *mut FILE,
    mut ccs: *const CostCentreStack,
    mut totals: ProfilerTotals,
) {
    let mut max_label_len: uint32_t = 0;
    let mut max_module_len: uint32_t = 0;
    let mut max_src_len: uint32_t = 0;
    let mut max_id_len: uint32_t = 0;
    max_label_len = 11 as uint32_t;
    max_module_len = 6 as uint32_t;
    max_src_len = 3 as uint32_t;
    max_id_len = 3 as uint32_t;

    findCCSMaxLens(
        ccs,
        0 as uint32_t,
        &raw mut max_label_len,
        &raw mut max_module_len,
        &raw mut max_src_len,
        &raw mut max_id_len,
    );

    fprintHeader(
        prof_file,
        max_label_len,
        max_module_len,
        max_src_len,
        max_id_len,
    );

    logCCS(
        prof_file,
        ccs,
        totals,
        0 as uint32_t,
        max_label_len,
        max_module_len,
        max_src_len,
        max_id_len,
    );
}

unsafe fn writeCCSReport(
    mut prof_file: *mut FILE,
    mut stack: *const CostCentreStack,
    mut totals: ProfilerTotals,
) {
    let mut temp: [c_char; 128] = [0; 128];

    fprintf(
        prof_file,
        b"\t%s Time and Allocation Profiling Report  (%s)\n\0" as *const u8 as *const c_char,
        time_str(),
        b"Final\0" as *const u8 as *const c_char,
    );

    fprintf(prof_file, b"\n\t  \0" as *const u8 as *const c_char);
    fprintf(prof_file, b" %s\0" as *const u8 as *const c_char, prog_name);
    fprintf(prof_file, b" +RTS\0" as *const u8 as *const c_char);

    let mut count = 0 as c_int;

    while !(*rts_argv.offset(count as isize)).is_null() {
        fprintf(
            prof_file,
            b" %s\0" as *const u8 as *const c_char,
            *rts_argv.offset(count as isize),
        );

        count += 1;
    }

    fprintf(prof_file, b" -RTS\0" as *const u8 as *const c_char);

    let mut count_0 = 1 as c_int;

    while !(*prog_argv.offset(count_0 as isize)).is_null() {
        fprintf(
            prof_file,
            b" %s\0" as *const u8 as *const c_char,
            *prog_argv.offset(count_0 as isize),
        );

        count_0 += 1;
    }

    fprintf(prof_file, b"\n\n\0" as *const u8 as *const c_char);

    fprintf(
        prof_file,
        b"\ttotal time  = %11.2f secs   (%lu ticks @ %d us, %d processor%s)\n\0" as *const u8
            as *const c_char,
        totals.total_prof_ticks as c_double * RtsFlags.MiscFlags.tickInterval as c_double
            / (TIME_RESOLUTION as c_uint).wrapping_mul(getNumCapabilities()) as c_double,
        totals.total_prof_ticks as c_ulong,
        (RtsFlags.MiscFlags.tickInterval / 1000 as Time) as c_int,
        getNumCapabilities(),
        if getNumCapabilities() > 1 as c_uint {
            b"s\0" as *const u8 as *const c_char
        } else {
            b"\0" as *const u8 as *const c_char
        },
    );

    fprintf(
        prof_file,
        b"\ttotal alloc = %11s bytes\0" as *const u8 as *const c_char,
        showStgWord64(
            (totals.total_alloc as StgWord64).wrapping_mul(size_of::<W_>() as StgWord64),
            &raw mut temp as *mut c_char,
            r#true != 0,
        ),
    );

    fprintf(
        prof_file,
        b"  (excludes profiling overheads)\n\n\0" as *const u8 as *const c_char,
    );

    reportPerCCCosts(prof_file, totals);
    reportCCS(prof_file, stack, totals);
}
