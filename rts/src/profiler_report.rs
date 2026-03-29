use crate::ffi::rts::flags::{COST_CENTRES_ALL, COST_CENTRES_VERBOSE, RtsFlags, rts_argv};
use crate::ffi::rts::prof::ccs::{CC_LIST, CostCentre, CostCentre_, CostCentreStack, IndexTable};
use crate::ffi::rts::threads::getNumCapabilities;
use crate::ffi::rts::time::{TIME_RESOLUTION, Time};
use crate::ffi::rts::{prog_argv, prog_name};
use crate::ffi::stg::types::{StgFloat, StgInt, StgWord, StgWord64};
use crate::prelude::*;
use crate::profiling::{ProfilerTotals, ignoreCC, ignoreCCS};
use crate::rts_utils::{showStgWord64, time_str};

unsafe fn strlen_utf8(mut s: *mut c_char) -> u32 {
    let mut n: u32 = 0;
    let mut c: u8 = 0;

    while *s as i32 != '\0' as i32 {
        c = *s as u8;

        if (c as i32) < 0x80 || c as i32 > 0xbf {
            n = n.wrapping_add(1);
        }

        s = s.offset(1);
    }

    return n;
}

unsafe fn fprintHeader(
    mut prof_file: *mut FILE,
    mut max_label_len: u32,
    mut max_module_len: u32,
    mut max_src_len: u32,
    mut max_id_len: u32,
) {
    fprintf(
        prof_file,
        c"%-*s %-*s %-*s %-*s %11s  %12s   %12s\n".as_ptr(),
        max_label_len,
        c"".as_ptr(),
        max_module_len,
        c"".as_ptr(),
        max_src_len,
        c"".as_ptr(),
        max_id_len,
        c"".as_ptr(),
        c"".as_ptr(),
        c"individual".as_ptr(),
        c"inherited".as_ptr(),
    );

    fprintf(
        prof_file,
        c"%-*s %-*s %-*s %-*s".as_ptr(),
        max_label_len,
        c"COST CENTRE".as_ptr(),
        max_module_len,
        c"MODULE".as_ptr(),
        max_src_len,
        c"SRC".as_ptr(),
        max_id_len,
        c"no.".as_ptr(),
    );

    fprintf(
        prof_file,
        c" %11s  %5s %6s   %5s %6s".as_ptr(),
        c"entries".as_ptr(),
        c"%time".as_ptr(),
        c"%alloc".as_ptr(),
        c"%time".as_ptr(),
        c"%alloc".as_ptr(),
    );

    if RtsFlags.CcFlags.doCostCentres >= COST_CENTRES_VERBOSE as u32 {
        fprintf(
            prof_file,
            c"  %5s %9s".as_ptr(),
            c"ticks".as_ptr(),
            c"bytes".as_ptr(),
        );
    }

    fprintf(prof_file, c"\n\n".as_ptr());
}

static mut sorted_cc_list: *mut CostCentre = null_mut::<CostCentre>();

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
    let mut max_label_len: u32 = 0;
    let mut max_module_len: u32 = 0;
    let mut max_src_len: u32 = 0;
    sorted_cc_list = null_mut::<CostCentre>();
    max_label_len = 11;
    max_module_len = 6;
    max_src_len = 3;
    cc = CC_LIST;

    while !cc.is_null() {
        next = (*cc).link as *mut CostCentre;

        if (*cc).time_ticks > totals.total_prof_ticks.wrapping_div(100 as u32) as StgWord
            || (*cc).mem_alloc > totals.total_alloc.wrapping_div(100 as u64)
            || RtsFlags.CcFlags.doCostCentres >= COST_CENTRES_ALL as u32
        {
            insertCCInSortedList(cc);

            max_label_len = ({
                let mut _a = strlen_utf8((*cc).label) as u32;
                let mut _b: u32 = max_label_len as u32;

                if _a <= _b { _b } else { _a as u32 }
            });

            max_module_len = ({
                let mut _a = strlen_utf8((*cc).module) as u32;
                let mut _b: u32 = max_module_len as u32;

                if _a <= _b { _b } else { _a as u32 }
            });

            max_src_len = ({
                let mut _a = strlen_utf8((*cc).srcloc) as u32;
                let mut _b: u32 = max_src_len as u32;

                if _a <= _b { _b } else { _a as u32 }
            });
        }

        cc = next;
    }

    fprintf(
        prof_file,
        c"%-*s %-*s %-*s".as_ptr(),
        max_label_len,
        c"COST CENTRE".as_ptr(),
        max_module_len,
        c"MODULE".as_ptr(),
        max_src_len,
        c"SRC".as_ptr(),
    );

    fprintf(
        prof_file,
        c" %6s %6s".as_ptr(),
        c"%time".as_ptr(),
        c"%alloc".as_ptr(),
    );

    if RtsFlags.CcFlags.doCostCentres >= COST_CENTRES_VERBOSE as u32 {
        fprintf(
            prof_file,
            c"  %5s %9s".as_ptr(),
            c"ticks".as_ptr(),
            c"bytes".as_ptr(),
        );
    }

    fprintf(prof_file, c"\n\n".as_ptr());
    cc = sorted_cc_list;

    while !cc.is_null() {
        if !ignoreCC(cc) {
            fprintf(
                prof_file,
                c"%s%*s %s%*s %s%*s".as_ptr(),
                (*cc).label,
                max_label_len.wrapping_sub(strlen_utf8((*cc).label)),
                c"".as_ptr(),
                (*cc).module,
                max_module_len.wrapping_sub(strlen_utf8((*cc).module)),
                c"".as_ptr(),
                (*cc).srcloc,
                max_src_len.wrapping_sub(strlen_utf8((*cc).srcloc)),
                c"".as_ptr(),
            );

            fprintf(
                prof_file,
                c" %6.1f %6.1f".as_ptr(),
                if totals.total_prof_ticks == 0 {
                    0.0f64
                } else {
                    ((*cc).time_ticks as StgFloat / totals.total_prof_ticks as StgFloat * 100)
                        as f64
                },
                if totals.total_alloc == 0 {
                    0.0f64
                } else {
                    ((*cc).mem_alloc as StgFloat / totals.total_alloc as StgFloat * 100) as f64
                },
            );

            if RtsFlags.CcFlags.doCostCentres >= COST_CENTRES_VERBOSE as u32 {
                fprintf(
                    prof_file,
                    c"  %5llu %9llu".as_ptr(),
                    (*cc).time_ticks,
                    (*cc).mem_alloc.wrapping_mul(size_of::<W_>() as StgWord64),
                );
            }

            fprintf(prof_file, c"\n".as_ptr());
        }

        cc = (*cc).link as *mut CostCentre;
    }

    fprintf(prof_file, c"\n\n".as_ptr());
}

unsafe fn numDigits(mut i: StgInt) -> u32 {
    let mut result: u32 = 0;
    result = 1;

    if i < 0 {
        i = 0;
    }

    while i > 9 {
        i /= 10;
        result = result.wrapping_add(1);
    }

    return result;
}

unsafe fn findCCSMaxLens(
    mut ccs: *const CostCentreStack,
    mut indent: u32,
    mut max_label_len: *mut u32,
    mut max_module_len: *mut u32,
    mut max_src_len: *mut u32,
    mut max_id_len: *mut u32,
) {
    let mut cc = null_mut::<CostCentre>();
    let mut i = null_mut::<IndexTable>();
    cc = (*ccs).cc;

    *max_label_len = ({
        let mut _a: u32 = *max_label_len;
        let mut _b: u32 = (indent as u32).wrapping_add(strlen_utf8((*cc).label) as u32);

        if _a <= _b { _b } else { _a as u32 }
    });

    *max_module_len = ({
        let mut _a: u32 = *max_module_len;
        let mut _b = strlen_utf8((*cc).module) as u32;

        if _a <= _b { _b } else { _a as u32 }
    });

    *max_src_len = ({
        let mut _a: u32 = *max_src_len;
        let mut _b = strlen_utf8((*cc).srcloc) as u32;

        if _a <= _b { _b } else { _a as u32 }
    });

    *max_id_len = ({
        let mut _a: u32 = *max_id_len;
        let mut _b = numDigits((*ccs).ccsID) as u32;

        if _a <= _b { _b } else { _a as u32 }
    });

    i = (*ccs).indexTable as *mut IndexTable;

    while !i.is_null() {
        if !(*i).back_edge {
            findCCSMaxLens(
                (*i).ccs,
                indent.wrapping_add(1 as u32),
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
    mut indent: u32,
    mut max_label_len: u32,
    mut max_module_len: u32,
    mut max_src_len: u32,
    mut max_id_len: u32,
) {
    let mut cc = null_mut::<CostCentre>();
    let mut i = null_mut::<IndexTable>();
    cc = (*ccs).cc;

    if !ignoreCCS(ccs) {
        fprintf(
            prof_file,
            c"%*s%s%*s %s%*s %s%*s".as_ptr(),
            indent,
            c"".as_ptr(),
            (*cc).label,
            max_label_len
                .wrapping_sub(indent)
                .wrapping_sub(strlen_utf8((*cc).label)),
            c"".as_ptr(),
            (*cc).module,
            max_module_len.wrapping_sub(strlen_utf8((*cc).module)),
            c"".as_ptr(),
            (*cc).srcloc,
            max_src_len.wrapping_sub(strlen_utf8((*cc).srcloc)),
            c"".as_ptr(),
        );

        fprintf(
            prof_file,
            c" %*lld %11llu  %5.1f  %5.1f   %5.1f  %5.1f".as_ptr(),
            max_id_len,
            (*ccs).ccsID,
            (*ccs).scc_count,
            if totals.total_prof_ticks == 0 {
                0.0f64
            } else {
                (*ccs).time_ticks as f64 / totals.total_prof_ticks as f64 * 100.0f64
            },
            if totals.total_alloc == 0 {
                0.0f64
            } else {
                (*ccs).mem_alloc as f64 / totals.total_alloc as f64 * 100.0f64
            },
            if totals.total_prof_ticks == 0 {
                0.0f64
            } else {
                (*ccs).inherited_ticks as f64 / totals.total_prof_ticks as f64 * 100.0f64
            },
            if totals.total_alloc == 0 {
                0.0f64
            } else {
                (*ccs).inherited_alloc as f64 / totals.total_alloc as f64 * 100.0f64
            },
        );

        if RtsFlags.CcFlags.doCostCentres >= COST_CENTRES_VERBOSE as u32 {
            fprintf(
                prof_file,
                c"  %5llu %9llu".as_ptr(),
                (*ccs).time_ticks,
                (*ccs).mem_alloc.wrapping_mul(size_of::<W_>() as StgWord64),
            );
        }

        fprintf(prof_file, c"\n".as_ptr());
    }

    i = (*ccs).indexTable as *mut IndexTable;

    while !i.is_null() {
        if !(*i).back_edge {
            logCCS(
                prof_file,
                (*i).ccs,
                totals,
                indent.wrapping_add(1 as u32),
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
    let mut max_label_len: u32 = 0;
    let mut max_module_len: u32 = 0;
    let mut max_src_len: u32 = 0;
    let mut max_id_len: u32 = 0;
    max_label_len = 11;
    max_module_len = 6;
    max_src_len = 3;
    max_id_len = 3;

    findCCSMaxLens(
        ccs,
        0,
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
        0,
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
        c"\t%s Time and Allocation Profiling Report  (%s)\n".as_ptr(),
        time_str(),
        c"Final".as_ptr(),
    );

    fprintf(prof_file, c"\n\t  ".as_ptr());
    fprintf(prof_file, c" %s".as_ptr(), prog_name);
    fprintf(prof_file, c" +RTS".as_ptr());

    let mut count = 0;

    while !(*rts_argv.offset(count as isize)).is_null() {
        fprintf(prof_file, c" %s".as_ptr(), *rts_argv.offset(count as isize));
        count += 1;
    }

    fprintf(prof_file, c" -RTS".as_ptr());

    let mut count_0 = 1;

    while !(*prog_argv.offset(count_0 as isize)).is_null() {
        fprintf(
            prof_file,
            c" %s".as_ptr(),
            *prog_argv.offset(count_0 as isize),
        );
        count_0 += 1;
    }

    fprintf(prof_file, c"\n\n".as_ptr());

    fprintf(
        prof_file,
        c"\ttotal time  = %11.2f secs   (%lu ticks @ %d us, %d processor%s)\n".as_ptr(),
        totals.total_prof_ticks as f64 * RtsFlags.MiscFlags.tickInterval as f64
            / (TIME_RESOLUTION as u32).wrapping_mul(getNumCapabilities()) as f64,
        totals.total_prof_ticks as u64,
        (RtsFlags.MiscFlags.tickInterval / 1000) as i32,
        getNumCapabilities(),
        if getNumCapabilities() > 1 {
            c"s".as_ptr()
        } else {
            c"".as_ptr()
        },
    );

    fprintf(
        prof_file,
        c"\ttotal alloc = %11s bytes".as_ptr(),
        showStgWord64(
            (totals.total_alloc as StgWord64).wrapping_mul(size_of::<W_>() as StgWord64),
            &raw mut temp as *mut c_char,
            true,
        ),
    );

    fprintf(prof_file, c"  (excludes profiling overheads)\n\n".as_ptr());
    reportPerCCCosts(prof_file, totals);
    reportCCS(prof_file, stack, totals);
}
