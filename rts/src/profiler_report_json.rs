use crate::ffi::rts::prof::ccs::{CC_LIST, CostCentre, CostCentreStack, IndexTable};
use crate::ffi::rts::threads::getNumCapabilities;
use crate::ffi::rts::time::{TIME_RESOLUTION, Time};
use crate::ffi::rts::{prog_argv, prog_name};
use crate::ffi::stg::types::StgWord64;
use crate::prelude::*;
use crate::profiling::ProfilerTotals;
use crate::rts_flags::{RtsFlags, rts_argv};
use crate::rts_utils::{stgFree, stgMallocBytes, time_str};

unsafe fn escaped_size(mut str: *const c_char) -> usize {
    let mut escaped_size_0: usize = 0;

    while *str as i32 != '\0' as i32 {
        let c = *str as u8;

        match c as i32 {
            34 => {
                escaped_size_0 = escaped_size_0.wrapping_add(2 as usize);
            }
            92 => {
                escaped_size_0 = escaped_size_0.wrapping_add(2 as usize);
            }
            8 => {
                escaped_size_0 = escaped_size_0.wrapping_add(2 as usize);
            }
            12 => {
                escaped_size_0 = escaped_size_0.wrapping_add(2 as usize);
            }
            10 => {
                escaped_size_0 = escaped_size_0.wrapping_add(2 as usize);
            }
            13 => {
                escaped_size_0 = escaped_size_0.wrapping_add(2 as usize);
            }
            9 => {
                escaped_size_0 = escaped_size_0.wrapping_add(2 as usize);
            }
            _ => {
                if c as i32 <= 0x1f {
                    escaped_size_0 = escaped_size_0.wrapping_add(6 as usize);
                } else {
                    escaped_size_0 = escaped_size_0.wrapping_add(1);
                }
            }
        }

        str = str.offset(1);
    }

    escaped_size_0 = escaped_size_0.wrapping_add(1);

    return escaped_size_0;
}

unsafe fn escapeString(mut str: *const c_char, mut buf: *mut *mut c_char) {
    let mut out = null_mut::<c_char>();
    let mut out_size: usize = 0;
    let mut pos: usize = 0;
    out_size = escaped_size(str);
    out = stgMallocBytes(out_size, c"writeCCSReportJson".as_ptr()) as *mut c_char;

    while *str as i32 != '\0' as i32 {
        let c = *str as u8;

        match c as i32 {
            34 => {
                *out.offset(pos as isize) = '\\' as i32 as c_char;
                *out.offset(pos.wrapping_add(1 as usize) as isize) = '"' as i32 as c_char;
                pos = pos.wrapping_add(2 as usize);
            }
            92 => {
                *out.offset(pos as isize) = '\\' as i32 as c_char;
                *out.offset(pos.wrapping_add(1 as usize) as isize) = '\\' as i32 as c_char;
                pos = pos.wrapping_add(2 as usize);
            }
            8 => {
                *out.offset(pos as isize) = '\\' as i32 as c_char;
                *out.offset(pos.wrapping_add(1 as usize) as isize) = 'b' as i32 as c_char;
                pos = pos.wrapping_add(2 as usize);
            }
            12 => {
                *out.offset(pos as isize) = '\\' as i32 as c_char;
                *out.offset(pos.wrapping_add(1 as usize) as isize) = 'f' as i32 as c_char;
                pos = pos.wrapping_add(2 as usize);
            }
            10 => {
                *out.offset(pos as isize) = '\\' as i32 as c_char;
                *out.offset(pos.wrapping_add(1 as usize) as isize) = 'n' as i32 as c_char;
                pos = pos.wrapping_add(2 as usize);
            }
            13 => {
                *out.offset(pos as isize) = '\\' as i32 as c_char;
                *out.offset(pos.wrapping_add(1 as usize) as isize) = 'r' as i32 as c_char;
                pos = pos.wrapping_add(2 as usize);
            }
            9 => {
                *out.offset(pos as isize) = '\\' as i32 as c_char;
                *out.offset(pos.wrapping_add(1 as usize) as isize) = 't' as i32 as c_char;
                pos = pos.wrapping_add(2 as usize);
            }
            _ => {
                if c as i32 <= 0x1f {
                    *out.offset(pos as isize) = '\\' as i32 as c_char;

                    sprintf(
                        out.offset(pos.wrapping_add(1 as usize) as isize) as *mut c_char,
                        c"u%04x".as_ptr(),
                        c as i32,
                    );

                    pos = pos.wrapping_add(6 as usize);
                } else {
                    let fresh5 = pos;
                    pos = pos.wrapping_add(1);
                    *out.offset(fresh5 as isize) = c as c_char;
                }
            }
        }

        str = str.offset(1);
    }

    let fresh6 = pos;
    pos = pos.wrapping_add(1);
    *out.offset(fresh6 as isize) = '\0' as i32 as c_char;

    if !(pos == out_size) as i32 as i64 != 0 {
        __assert_rtn(
            c"escapeString".as_ptr(),
            c"rts/ProfilerReportJson.c".as_ptr(),
            190,
            c"pos == out_size".as_ptr(),
        );
    } else {
    };

    *buf = out;
}

unsafe fn logCostCentres(mut prof_file: *mut FILE) {
    let mut needs_comma = false;
    fprintf(prof_file, c"[\n".as_ptr());

    let mut cc = CC_LIST;

    while !cc.is_null() {
        let mut lbl = null_mut::<c_char>();
        let mut src_loc = null_mut::<c_char>();
        escapeString((*cc).label, &raw mut lbl);
        escapeString((*cc).srcloc, &raw mut src_loc);

        fprintf(
            prof_file,
            c"%s{\"id\": %lld, \"label\": \"%s\", \"module\": \"%s\", \"src_loc\": \"%s\", \"is_caf\": %s}"
                .as_ptr(),
            if needs_comma as i32 != 0 { c", ".as_ptr() } else { c"".as_ptr() },
            (*cc).ccID,
            lbl,
            (*cc).module,
            src_loc,
            if (*cc).is_caf != 0 { c"true".as_ptr() } else { c"false".as_ptr() },
        );

        needs_comma = true;
        stgFree(lbl as *mut c_void);
        stgFree(src_loc as *mut c_void);
        cc = (*cc).link as *mut CostCentre;
    }

    fprintf(prof_file, c"]\n".as_ptr());
}

unsafe fn logCostCentreStack(mut prof_file: *mut FILE, mut ccs: *const CostCentreStack) {
    fprintf(
        prof_file,
        c"{\"id\": %lld, \"entries\": %llu, \"alloc\": %llu, \"ticks\": %llu, ".as_ptr(),
        (*(*ccs).cc).ccID,
        (*ccs).scc_count,
        (*ccs).mem_alloc.wrapping_mul(size_of::<W_>() as StgWord64),
        (*ccs).time_ticks,
    );

    let mut need_comma = false;
    fprintf(prof_file, c"\"children\": [".as_ptr());

    let mut i = (*ccs).indexTable as *mut IndexTable;

    while !i.is_null() {
        if !(*i).back_edge {
            if need_comma {
                fprintf(prof_file, c",".as_ptr());
            }

            logCostCentreStack(prof_file, (*i).ccs);
            need_comma = true;
        }

        i = (*i).next as *mut IndexTable;
    }

    fprintf(prof_file, c"]}\n".as_ptr());
}

unsafe fn writeCCSReportJson(
    mut prof_file: *mut FILE,
    mut stack: *const CostCentreStack,
    mut totals: ProfilerTotals,
) {
    fprintf(prof_file, c"{\n\"program\": \"%s\",\n".as_ptr(), prog_name);
    fprintf(prof_file, c"\"arguments\": [".as_ptr());

    let mut count = 0;

    while !(*prog_argv.offset(count as isize)).is_null() {
        let mut arg = null_mut::<c_char>();
        escapeString(*prog_argv.offset(count as isize), &raw mut arg);

        fprintf(
            prof_file,
            c"%s\"%s\"".as_ptr(),
            if count == 0 {
                c"".as_ptr()
            } else {
                c", ".as_ptr()
            },
            arg,
        );

        stgFree(arg as *mut c_void);
        count += 1;
    }

    fprintf(prof_file, c"],\n\"rts_arguments\": [".as_ptr());

    let mut count_0 = 0;

    while !(*rts_argv.offset(count_0 as isize)).is_null() {
        let mut arg_0 = null_mut::<c_char>();
        escapeString(*rts_argv.offset(count_0 as isize), &raw mut arg_0);

        fprintf(
            prof_file,
            c"%s\"%s\"".as_ptr(),
            if count_0 == 0 {
                c"".as_ptr()
            } else {
                c", ".as_ptr()
            },
            arg_0,
        );

        stgFree(arg_0 as *mut c_void);
        count_0 += 1;
    }

    fprintf(prof_file, c"],\n".as_ptr());
    fprintf(prof_file, c"\"end_time\": \"%s\",\n".as_ptr(), time_str());

    fprintf(
        prof_file,
        c"\"initial_capabilities\": %d,\n".as_ptr(),
        RtsFlags.ParFlags.nCapabilities,
    );

    fprintf(
        prof_file,
        c"\"total_time\": %11.2f,\n".as_ptr(),
        totals.total_prof_ticks as f64 * RtsFlags.MiscFlags.tickInterval as f64
            / (TIME_RESOLUTION as u32).wrapping_mul(getNumCapabilities()) as f64,
    );

    fprintf(
        prof_file,
        c"\"total_ticks\": %lu,\n".as_ptr(),
        totals.total_prof_ticks as u64,
    );

    fprintf(
        prof_file,
        c"\"tick_interval\": %d,\n".as_ptr(),
        (RtsFlags.MiscFlags.tickInterval / 1000) as i32,
    );

    fprintf(
        prof_file,
        c"\"total_alloc\":%llu,\n".as_ptr(),
        totals.total_alloc.wrapping_mul(size_of::<W_>() as u64),
    );

    fprintf(prof_file, c"\"cost_centres\": ".as_ptr());
    logCostCentres(prof_file);
    fprintf(prof_file, c",\n\"profile\": ".as_ptr());
    logCostCentreStack(prof_file, stack);
    fprintf(prof_file, c"}\n".as_ptr());
}
