use crate::ffi::rts::flags::{HPC_YES_EXPLICIT, HPC_YES_IMPLICIT, RtsFlags};
use crate::ffi::rts::hpc::{_HpcModuleInfo, HpcModuleInfo};
use crate::ffi::rts::{prog_name, stg_exit};
use crate::ffi::stg::types::{StgWord32, StgWord64};
use crate::ffi::stg::types::{StgWord32, StgWord64};
use crate::fs::__rts_fopen;
use crate::hash::{
    StrHashTable, allocStrHashTable, freeStrHashTable, insertStrHashTable, lookupStrHashTable,
};
use crate::prelude::*;
use crate::rts_utils::{stgCallocBytes, stgFree, stgMallocBytes};
use crate::trace::{DEBUG_RTS, trace_};

#[cfg(test)]
mod tests;

/// cbindgen:no-export
#[repr(C)]
pub struct _HpcModuleInfo {
    pub(crate) modName: *mut c_char,
    pub(crate) tickCount: StgWord32,
    pub(crate) hashNo: StgWord32,
    pub(crate) tixArr: *mut StgWord64,
    pub(crate) from_file: bool,
    pub(crate) next: *mut _HpcModuleInfo,
}

#[ffi(libraries)]
pub type HpcModuleInfo = _HpcModuleInfo;

static mut hpc_inited: c_int = 0 as c_int;

static mut hpc_pid: pid_t = 0 as pid_t;

static mut tixFile: *mut FILE = null::<FILE>() as *mut FILE;

static mut tix_ch: c_int = 0;

static mut moduleHash: *mut StrHashTable = null::<StrHashTable>() as *mut StrHashTable;

static mut modules: *mut HpcModuleInfo = null::<HpcModuleInfo>() as *mut HpcModuleInfo;

static mut tixFilename: *mut c_char = null::<c_char>() as *mut c_char;

unsafe fn failure(mut msg: *mut c_char) -> ! {
    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.hpc as c_long != 0 {
        trace_(
            b"hpc failure: %s\n\0" as *const u8 as *const c_char as *mut c_char,
            msg,
        );
    }

    fprintf(
        __stderrp,
        b"Hpc failure: %s\n\0" as *const u8 as *const c_char,
        msg,
    );

    if !tixFilename.is_null() {
        fprintf(
            __stderrp,
            b"(perhaps remove %s file?)\n\0" as *const u8 as *const c_char,
            tixFilename,
        );
    } else {
        fprintf(
            __stderrp,
            b"(perhaps remove .tix file?)\n\0" as *const u8 as *const c_char,
        );
    }

    stg_exit(1 as c_int);
}

unsafe fn init_open(mut file: *mut FILE) -> c_int {
    tixFile = file;

    if tixFile.is_null() {
        return 0 as c_int;
    }

    tix_ch = getc(tixFile);

    return 1 as c_int;
}

unsafe fn expect(mut c: c_char) {
    if tix_ch != c as c_int {
        fprintf(
            __stderrp,
            b"('%c' '%c')\n\0" as *const u8 as *const c_char,
            tix_ch,
            c as c_int,
        );

        failure(
            b"parse error when reading .tix file\0" as *const u8 as *const c_char as *mut c_char,
        );
    }

    tix_ch = getc(tixFile);
}

unsafe fn ws() {
    while tix_ch == ' ' as i32 {
        tix_ch = getc(tixFile);
    }
}

unsafe fn expectString() -> *mut c_char {
    let mut tmp: [c_char; 256] = [0; 256];
    let mut res = null_mut::<c_char>();
    let mut tmp_ix = 0 as c_int;
    expect('"' as i32 as c_char);

    while tix_ch != '"' as i32 {
        let fresh5 = tmp_ix;
        tmp_ix = tmp_ix + 1;
        tmp[fresh5 as usize] = tix_ch as c_char;
        tix_ch = getc(tixFile);
    }

    let fresh6 = tmp_ix;
    tmp_ix = tmp_ix + 1;
    tmp[fresh6 as usize] = 0 as c_char;
    expect('"' as i32 as c_char);

    res = stgMallocBytes(
        tmp_ix as size_t,
        b"Hpc.expectString\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut c_char;

    strcpy(res, &raw mut tmp as *mut c_char);

    return res;
}

unsafe fn expectWord64() -> StgWord64 {
    let mut tmp: StgWord64 = 0 as StgWord64;

    while isdigit(tix_ch) != 0 {
        tmp = tmp
            .wrapping_mul(10 as StgWord64)
            .wrapping_add((tix_ch - '0' as i32) as StgWord64);
        tix_ch = getc(tixFile);
    }

    return tmp;
}

unsafe fn readTix() {
    let mut i: c_uint = 0;
    let mut tmpModule = null_mut::<HpcModuleInfo>();
    let mut lookup = null::<HpcModuleInfo>();
    ws();
    expect('T' as i32 as c_char);
    expect('i' as i32 as c_char);
    expect('x' as i32 as c_char);
    ws();
    expect('[' as i32 as c_char);
    ws();

    while tix_ch != ']' as i32 {
        tmpModule = stgMallocBytes(
            size_of::<HpcModuleInfo>() as size_t,
            b"Hpc.readTix\0" as *const u8 as *const c_char as *mut c_char,
        ) as *mut HpcModuleInfo;

        (*tmpModule).from_file = r#true != 0;
        expect('T' as i32 as c_char);
        expect('i' as i32 as c_char);
        expect('x' as i32 as c_char);
        expect('M' as i32 as c_char);
        expect('o' as i32 as c_char);
        expect('d' as i32 as c_char);
        expect('u' as i32 as c_char);
        expect('l' as i32 as c_char);
        expect('e' as i32 as c_char);
        ws();
        (*tmpModule).modName = expectString();
        ws();
        (*tmpModule).hashNo = expectWord64() as c_uint as StgWord32;
        ws();
        (*tmpModule).tickCount = expectWord64() as c_int as StgWord32;

        (*tmpModule).tixArr = stgCallocBytes(
            (*tmpModule).tickCount as size_t,
            size_of::<StgWord64>() as size_t,
            b"readTix\0" as *const u8 as *const c_char as *mut c_char,
        ) as *mut StgWord64;

        ws();
        expect('[' as i32 as c_char);
        ws();
        i = 0 as c_uint;

        while (i as StgWord32) < (*tmpModule).tickCount {
            *(*tmpModule).tixArr.offset(i as isize) = expectWord64();
            ws();

            if tix_ch == ',' as i32 {
                expect(',' as i32 as c_char);
                ws();
            }

            i = i.wrapping_add(1);
        }

        expect(']' as i32 as c_char);
        ws();
        lookup = lookupStrHashTable(moduleHash, (*tmpModule).modName) as *const HpcModuleInfo;

        if lookup.is_null() {
            if DEBUG_RTS != 0 && RtsFlags.DebugFlags.hpc as c_long != 0 {
                trace_(
                    b"readTix: new HpcModuleInfo for %s\0" as *const u8 as *const c_char
                        as *mut c_char,
                    (*tmpModule).modName,
                );
            }

            insertStrHashTable(moduleHash, (*tmpModule).modName, tmpModule as *const c_void);
        } else {
            if DEBUG_RTS != 0 && RtsFlags.DebugFlags.hpc as c_long != 0 {
                trace_(
                    b"readTix: existing HpcModuleInfo for %s\0" as *const u8 as *const c_char
                        as *mut c_char,
                    (*tmpModule).modName,
                );
            }

            if (*tmpModule).hashNo != (*lookup).hashNo {
                fprintf(
                    __stderrp,
                    b"in module '%s'\n\0" as *const u8 as *const c_char,
                    (*tmpModule).modName,
                );

                failure(
                    b"module mismatch with .tix/.mix file hash number\0" as *const u8
                        as *const c_char as *mut c_char,
                );
            }

            i = 0 as c_uint;

            while (i as StgWord32) < (*tmpModule).tickCount {
                *(*lookup).tixArr.offset(i as isize) = *(*tmpModule).tixArr.offset(i as isize);
                i = i.wrapping_add(1);
            }

            stgFree((*tmpModule).tixArr as *mut c_void);
            stgFree((*tmpModule).modName as *mut c_void);
            stgFree(tmpModule as *mut c_void);
        }

        if tix_ch == ',' as i32 {
            expect(',' as i32 as c_char);
            ws();
        }
    }

    expect(']' as i32 as c_char);
    fclose(tixFile);
}

unsafe fn startupHpc() {
    let mut hpc_tixdir = null_mut::<c_char>();
    let mut hpc_tixfile = null_mut::<c_char>();

    if moduleHash.is_null() {
        return;
    }

    if hpc_inited != 0 as c_int {
        return;
    }

    hpc_inited = 1 as c_int;
    hpc_pid = getpid();
    hpc_tixdir = getenv(b"HPCTIXDIR\0" as *const u8 as *const c_char);
    hpc_tixfile = getenv(b"HPCTIXFILE\0" as *const u8 as *const c_char);

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.hpc as c_long != 0 {
        trace_(b"startupHpc\0" as *const u8 as *const c_char as *mut c_char);
    }

    if !hpc_tixfile.is_null() {
        tixFilename = strdup(hpc_tixfile);
    } else if !hpc_tixdir.is_null() {
        mkdir(hpc_tixdir, 0o777 as mode_t);

        tixFilename = stgMallocBytes(
            strlen(hpc_tixdir)
                .wrapping_add(strlen(prog_name))
                .wrapping_add(12 as size_t),
            b"Hpc.startupHpc\0" as *const u8 as *const c_char as *mut c_char,
        ) as *mut c_char;

        sprintf(
            tixFilename,
            b"%s/%s-%d.tix\0" as *const u8 as *const c_char,
            hpc_tixdir,
            prog_name,
            hpc_pid as c_int,
        );
    } else {
        tixFilename = stgMallocBytes(
            strlen(prog_name).wrapping_add(6 as size_t),
            b"Hpc.startupHpc\0" as *const u8 as *const c_char as *mut c_char,
        ) as *mut c_char;

        sprintf(
            tixFilename,
            b"%s.tix\0" as *const u8 as *const c_char,
            prog_name,
        );
    }

    if RtsFlags.HpcFlags.readTixFile as c_uint == HPC_YES_IMPLICIT as c_int as c_uint
        && init_open(__rts_fopen(
            tixFilename,
            b"r\0" as *const u8 as *const c_char,
        )) != 0
    {
        fprintf(
            __stderrp,
            b"Deprecation warning:\nI am reading in the existing tix file, and will add hpc info from this run to the existing data in that file.\nGHC 9.14 will cease looking for an existing tix file by default.\nIf you positively want to add hpc info to the current tix file, use the RTS option --read-tix-file=yes.\nMore information can be found in the accepted GHC proposal 612.\n\0"
                as *const u8 as *const c_char,
        );

        readTix();
    } else if RtsFlags.HpcFlags.readTixFile as c_uint == HPC_YES_EXPLICIT as c_int as c_uint
        && init_open(__rts_fopen(
            tixFilename,
            b"r\0" as *const u8 as *const c_char,
        )) != 0
    {
        readTix();
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_hpc_module(
    mut modName: *mut c_char,
    mut modCount: StgWord32,
    mut modHashNo: StgWord32,
    mut tixArr: *mut StgWord64,
) {
    let mut tmpModule = null_mut::<HpcModuleInfo>();
    let mut i: uint32_t = 0;

    if moduleHash.is_null() {
        moduleHash = allocStrHashTable();
    }

    tmpModule = lookupStrHashTable(moduleHash, modName) as *mut HpcModuleInfo;

    if tmpModule.is_null() {
        tmpModule = stgMallocBytes(
            size_of::<HpcModuleInfo>() as size_t,
            b"Hpc.hs_hpc_module\0" as *const u8 as *const c_char as *mut c_char,
        ) as *mut HpcModuleInfo;

        (*tmpModule).modName = modName;
        (*tmpModule).tickCount = modCount;
        (*tmpModule).hashNo = modHashNo;
        (*tmpModule).tixArr = tixArr;
        i = 0 as uint32_t;

        while i < modCount {
            *tixArr.offset(i as isize) = 0 as StgWord64;
            i = i.wrapping_add(1);
        }

        (*tmpModule).next = modules as *mut _HpcModuleInfo;
        (*tmpModule).from_file = r#false != 0;
        modules = tmpModule;
        insertStrHashTable(moduleHash, modName, tmpModule as *const c_void);
    } else {
        if (*tmpModule).tickCount != modCount {
            failure(
                b"inconsistent number of tick boxes\0" as *const u8 as *const c_char as *mut c_char,
            );
        }

        if (*tmpModule).hashNo != modHashNo {
            fprintf(
                __stderrp,
                b"in module '%s'\n\0" as *const u8 as *const c_char,
                (*tmpModule).modName,
            );

            failure(
                b"module mismatch with .tix/.mix file hash number\0" as *const u8 as *const c_char
                    as *mut c_char,
            );
        }

        i = 0 as uint32_t;

        while i < modCount {
            *tixArr.offset(i as isize) = *(*tmpModule).tixArr.offset(i as isize);
            i = i.wrapping_add(1);
        }

        if (*tmpModule).from_file {
            stgFree((*tmpModule).modName as *mut c_void);
            stgFree((*tmpModule).tixArr as *mut c_void);
        }

        (*tmpModule).from_file = r#false != 0;
    };
}

unsafe fn writeTix(mut f: *mut FILE) {
    let mut tmpModule = null_mut::<HpcModuleInfo>();
    let mut i: c_uint = 0;
    let mut inner_comma: c_uint = 0;
    let mut outer_comma: c_uint = 0;
    outer_comma = 0 as c_uint;

    if f.is_null() {
        return;
    }

    fprintf(f, b"Tix [\0" as *const u8 as *const c_char);
    tmpModule = modules;

    while !tmpModule.is_null() {
        if outer_comma != 0 {
            fprintf(f, b",\0" as *const u8 as *const c_char);
        } else {
            outer_comma = 1 as c_uint;
        }

        fprintf(
            f,
            b" TixModule \"%s\" %u %u [\0" as *const u8 as *const c_char,
            (*tmpModule).modName,
            (*tmpModule).hashNo,
            (*tmpModule).tickCount,
        );

        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.hpc as c_long != 0 {
            trace_(
                b"%s: %u (hash=%u)\n\0" as *const u8 as *const c_char as *mut c_char,
                (*tmpModule).modName,
                (*tmpModule).tickCount,
                (*tmpModule).hashNo,
            );
        }

        inner_comma = 0 as c_uint;
        i = 0 as c_uint;

        while (i as StgWord32) < (*tmpModule).tickCount {
            if inner_comma != 0 {
                fprintf(f, b",\0" as *const u8 as *const c_char);
            } else {
                inner_comma = 1 as c_uint;
            }

            if !(*tmpModule).tixArr.is_null() {
                fprintf(
                    f,
                    b"%llu\0" as *const u8 as *const c_char,
                    *(*tmpModule).tixArr.offset(i as isize),
                );
            } else {
                fprintf(f, b"0\0" as *const u8 as *const c_char);
            }

            i = i.wrapping_add(1);
        }

        fprintf(f, b"]\0" as *const u8 as *const c_char);
        tmpModule = (*tmpModule).next as *mut HpcModuleInfo;
    }

    fprintf(f, b"]\n\0" as *const u8 as *const c_char);
    fclose(f);
}

unsafe fn freeHpcModuleInfo(mut r#mod: *mut HpcModuleInfo) {
    if (*r#mod).from_file {
        stgFree((*r#mod).modName as *mut c_void);
        stgFree((*r#mod).tixArr as *mut c_void);
    }

    stgFree(r#mod as *mut c_void);
}

unsafe fn exitHpc() {
    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.hpc as c_long != 0 {
        trace_(b"exitHpc\0" as *const u8 as *const c_char as *mut c_char);
    }

    if hpc_inited == 0 as c_int {
        return;
    }

    let mut is_subprocess = hpc_pid != getpid();

    if !is_subprocess && RtsFlags.HpcFlags.writeTixFile as c_int != 0 {
        let mut f = __rts_fopen(tixFilename, b"w+\0" as *const u8 as *const c_char);
        writeTix(f);
    }

    freeStrHashTable(
        moduleHash,
        transmute::<
            Option<unsafe extern "C" fn(*mut HpcModuleInfo) -> ()>,
            Option<unsafe extern "C" fn(*mut c_void) -> ()>,
        >(Some(
            freeHpcModuleInfo as unsafe extern "C" fn(*mut HpcModuleInfo) -> (),
        )),
    );

    moduleHash = null_mut::<StrHashTable>();
    stgFree(tixFilename as *mut c_void);
    tixFilename = null_mut::<c_char>();
}

#[ffi(libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_hpc_rootModule() -> *mut HpcModuleInfo {
    return modules;
}
