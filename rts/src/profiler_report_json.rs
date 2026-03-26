use crate::ffi::rts::flags::{RtsFlags, rts_argv};
use crate::ffi::rts::prof::ccs::{CC_LIST, CostCentre, CostCentreStack, IndexTable};
use crate::ffi::rts::threads::getNumCapabilities;
use crate::ffi::rts::time::{TIME_RESOLUTION, Time};
use crate::ffi::rts::{prog_argv, prog_name};
use crate::ffi::stg::types::StgWord64;
use crate::prelude::*;
use crate::profiling::ProfilerTotals;
use crate::rts_utils::{stgFree, stgMallocBytes, time_str};

unsafe fn escaped_size(mut str: *const c_char) -> size_t {
    let mut escaped_size_0: size_t = 0 as size_t;

    while *str as c_int != '\0' as i32 {
        let c = *str as c_uchar;

        match c as c_int {
            34 => {
                escaped_size_0 = escaped_size_0.wrapping_add(2 as size_t);
            }
            92 => {
                escaped_size_0 = escaped_size_0.wrapping_add(2 as size_t);
            }
            8 => {
                escaped_size_0 = escaped_size_0.wrapping_add(2 as size_t);
            }
            12 => {
                escaped_size_0 = escaped_size_0.wrapping_add(2 as size_t);
            }
            10 => {
                escaped_size_0 = escaped_size_0.wrapping_add(2 as size_t);
            }
            13 => {
                escaped_size_0 = escaped_size_0.wrapping_add(2 as size_t);
            }
            9 => {
                escaped_size_0 = escaped_size_0.wrapping_add(2 as size_t);
            }
            _ => {
                if c as c_int <= 0x1f as c_int {
                    escaped_size_0 = escaped_size_0.wrapping_add(6 as size_t);
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
    let mut out_size: size_t = 0;
    let mut pos: size_t = 0 as size_t;
    out_size = escaped_size(str);

    out = stgMallocBytes(
        out_size,
        b"writeCCSReportJson\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut c_char;

    while *str as c_int != '\0' as i32 {
        let c = *str as c_uchar;

        match c as c_int {
            34 => {
                *out.offset(pos as isize) = '\\' as i32 as c_char;
                *out.offset(pos.wrapping_add(1 as size_t) as isize) = '"' as i32 as c_char;
                pos = pos.wrapping_add(2 as size_t);
            }
            92 => {
                *out.offset(pos as isize) = '\\' as i32 as c_char;
                *out.offset(pos.wrapping_add(1 as size_t) as isize) = '\\' as i32 as c_char;
                pos = pos.wrapping_add(2 as size_t);
            }
            8 => {
                *out.offset(pos as isize) = '\\' as i32 as c_char;
                *out.offset(pos.wrapping_add(1 as size_t) as isize) = 'b' as i32 as c_char;
                pos = pos.wrapping_add(2 as size_t);
            }
            12 => {
                *out.offset(pos as isize) = '\\' as i32 as c_char;
                *out.offset(pos.wrapping_add(1 as size_t) as isize) = 'f' as i32 as c_char;
                pos = pos.wrapping_add(2 as size_t);
            }
            10 => {
                *out.offset(pos as isize) = '\\' as i32 as c_char;
                *out.offset(pos.wrapping_add(1 as size_t) as isize) = 'n' as i32 as c_char;
                pos = pos.wrapping_add(2 as size_t);
            }
            13 => {
                *out.offset(pos as isize) = '\\' as i32 as c_char;
                *out.offset(pos.wrapping_add(1 as size_t) as isize) = 'r' as i32 as c_char;
                pos = pos.wrapping_add(2 as size_t);
            }
            9 => {
                *out.offset(pos as isize) = '\\' as i32 as c_char;
                *out.offset(pos.wrapping_add(1 as size_t) as isize) = 't' as i32 as c_char;
                pos = pos.wrapping_add(2 as size_t);
            }
            _ => {
                if c as c_int <= 0x1f as c_int {
                    *out.offset(pos as isize) = '\\' as i32 as c_char;

                    sprintf(
                        out.offset(pos.wrapping_add(1 as size_t) as isize) as *mut c_char,
                        b"u%04x\0" as *const u8 as *const c_char,
                        c as c_int,
                    );

                    pos = pos.wrapping_add(6 as size_t);
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

    if !(pos == out_size) as c_int as c_long != 0 {
        __assert_rtn(
            b"escapeString\0" as *const u8 as *const c_char,
            b"rts/ProfilerReportJson.c\0" as *const u8 as *const c_char,
            190 as c_int,
            b"pos == out_size\0" as *const u8 as *const c_char,
        );
    } else {
    };

    *buf = out;
}

unsafe fn logCostCentres(mut prof_file: *mut FILE) {
    let mut needs_comma = r#false != 0;
    fprintf(prof_file, b"[\n\0" as *const u8 as *const c_char);

    let mut cc = CC_LIST;

    while !cc.is_null() {
        let mut lbl = null_mut::<c_char>();
        let mut src_loc = null_mut::<c_char>();
        escapeString((*cc).label, &raw mut lbl);
        escapeString((*cc).srcloc, &raw mut src_loc);

        fprintf(
            prof_file,
            b"%s{\"id\": %lld, \"label\": \"%s\", \"module\": \"%s\", \"src_loc\": \"%s\", \"is_caf\": %s}\0"
                as *const u8 as *const c_char,
            if needs_comma as c_int != 0 {
                b", \0" as *const u8 as *const c_char
            } else {
                b"\0" as *const u8 as *const c_char
            },
            (*cc).ccID,
            lbl,
            (*cc).module,
            src_loc,
            if (*cc).is_caf != 0 {
                b"true\0" as *const u8 as *const c_char
            } else {
                b"false\0" as *const u8 as *const c_char
            },
        );

        needs_comma = r#true != 0;
        stgFree(lbl as *mut c_void);
        stgFree(src_loc as *mut c_void);
        cc = (*cc).link as *mut CostCentre;
    }

    fprintf(prof_file, b"]\n\0" as *const u8 as *const c_char);
}

unsafe fn logCostCentreStack(mut prof_file: *mut FILE, mut ccs: *const CostCentreStack) {
    fprintf(
        prof_file,
        b"{\"id\": %lld, \"entries\": %llu, \"alloc\": %llu, \"ticks\": %llu, \0" as *const u8
            as *const c_char,
        (*(*ccs).cc).ccID,
        (*ccs).scc_count,
        (*ccs).mem_alloc.wrapping_mul(size_of::<W_>() as StgWord64),
        (*ccs).time_ticks,
    );

    let mut need_comma = r#false != 0;

    fprintf(
        prof_file,
        b"\"children\": [\0" as *const u8 as *const c_char,
    );

    let mut i = (*ccs).indexTable as *mut IndexTable;

    while !i.is_null() {
        if !(*i).back_edge {
            if need_comma {
                fprintf(prof_file, b",\0" as *const u8 as *const c_char);
            }

            logCostCentreStack(prof_file, (*i).ccs);
            need_comma = r#true != 0;
        }

        i = (*i).next as *mut IndexTable;
    }

    fprintf(prof_file, b"]}\n\0" as *const u8 as *const c_char);
}

unsafe fn writeCCSReportJson(
    mut prof_file: *mut FILE,
    mut stack: *const CostCentreStack,
    mut totals: ProfilerTotals,
) {
    fprintf(
        prof_file,
        b"{\n\"program\": \"%s\",\n\0" as *const u8 as *const c_char,
        prog_name,
    );

    fprintf(
        prof_file,
        b"\"arguments\": [\0" as *const u8 as *const c_char,
    );

    let mut count = 0 as c_int;

    while !(*prog_argv.offset(count as isize)).is_null() {
        let mut arg = null_mut::<c_char>();
        escapeString(*prog_argv.offset(count as isize), &raw mut arg);

        fprintf(
            prof_file,
            b"%s\"%s\"\0" as *const u8 as *const c_char,
            if count == 0 as c_int {
                b"\0" as *const u8 as *const c_char
            } else {
                b", \0" as *const u8 as *const c_char
            },
            arg,
        );

        stgFree(arg as *mut c_void);
        count += 1;
    }

    fprintf(
        prof_file,
        b"],\n\"rts_arguments\": [\0" as *const u8 as *const c_char,
    );

    let mut count_0 = 0 as c_int;

    while !(*rts_argv.offset(count_0 as isize)).is_null() {
        let mut arg_0 = null_mut::<c_char>();
        escapeString(*rts_argv.offset(count_0 as isize), &raw mut arg_0);

        fprintf(
            prof_file,
            b"%s\"%s\"\0" as *const u8 as *const c_char,
            if count_0 == 0 as c_int {
                b"\0" as *const u8 as *const c_char
            } else {
                b", \0" as *const u8 as *const c_char
            },
            arg_0,
        );

        stgFree(arg_0 as *mut c_void);
        count_0 += 1;
    }

    fprintf(prof_file, b"],\n\0" as *const u8 as *const c_char);

    fprintf(
        prof_file,
        b"\"end_time\": \"%s\",\n\0" as *const u8 as *const c_char,
        time_str(),
    );

    fprintf(
        prof_file,
        b"\"initial_capabilities\": %d,\n\0" as *const u8 as *const c_char,
        RtsFlags.ParFlags.nCapabilities,
    );

    fprintf(
        prof_file,
        b"\"total_time\": %11.2f,\n\0" as *const u8 as *const c_char,
        totals.total_prof_ticks as c_double * RtsFlags.MiscFlags.tickInterval as c_double
            / (TIME_RESOLUTION as c_uint).wrapping_mul(getNumCapabilities()) as c_double,
    );

    fprintf(
        prof_file,
        b"\"total_ticks\": %lu,\n\0" as *const u8 as *const c_char,
        totals.total_prof_ticks as c_ulong,
    );

    fprintf(
        prof_file,
        b"\"tick_interval\": %d,\n\0" as *const u8 as *const c_char,
        (RtsFlags.MiscFlags.tickInterval / 1000 as Time) as c_int,
    );

    fprintf(
        prof_file,
        b"\"total_alloc\":%llu,\n\0" as *const u8 as *const c_char,
        totals.total_alloc.wrapping_mul(size_of::<W_>() as uint64_t),
    );

    fprintf(
        prof_file,
        b"\"cost_centres\": \0" as *const u8 as *const c_char,
    );

    logCostCentres(prof_file);

    fprintf(
        prof_file,
        b",\n\"profile\": \0" as *const u8 as *const c_char,
    );

    logCostCentreStack(prof_file, stack);
    fprintf(prof_file, b"}\n\0" as *const u8 as *const c_char);
}
