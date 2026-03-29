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

static mut hpc_inited: i32 = 0;

static mut hpc_pid: pid_t = 0;

static mut tixFile: *mut FILE = null_mut::<FILE>();

static mut tix_ch: i32 = 0;

static mut moduleHash: *mut StrHashTable = null_mut::<StrHashTable>();

static mut modules: *mut HpcModuleInfo = null_mut::<HpcModuleInfo>();

static mut tixFilename: *mut c_char = null_mut::<c_char>();

unsafe fn failure(mut msg: *mut c_char) -> ! {
    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.hpc as i64 != 0 {
        trace_(c"hpc failure: %s\n".as_ptr(), msg);
    }

    fprintf(__stderrp, c"Hpc failure: %s\n".as_ptr(), msg);

    if !tixFilename.is_null() {
        fprintf(
            __stderrp,
            c"(perhaps remove %s file?)\n".as_ptr(),
            tixFilename,
        );
    } else {
        fprintf(__stderrp, c"(perhaps remove .tix file?)\n".as_ptr());
    }

    stg_exit(1);
}

unsafe fn init_open(mut file: *mut FILE) -> i32 {
    tixFile = file;

    if tixFile.is_null() {
        return 0;
    }

    tix_ch = getc(tixFile);

    return 1;
}

unsafe fn expect(mut c: c_char) {
    if tix_ch != c as i32 {
        fprintf(__stderrp, c"('%c' '%c')\n".as_ptr(), tix_ch, c as i32);
        failure(c"parse error when reading .tix file".as_ptr());
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
    let mut tmp_ix = 0;
    expect('"' as i32 as c_char);

    while tix_ch != '"' as i32 {
        let fresh5 = tmp_ix;
        tmp_ix = tmp_ix + 1;
        tmp[fresh5 as usize] = tix_ch as c_char;
        tix_ch = getc(tixFile);
    }

    let fresh6 = tmp_ix;
    tmp_ix = tmp_ix + 1;
    tmp[fresh6 as usize] = 0;
    expect('"' as i32 as c_char);
    res = stgMallocBytes(tmp_ix as usize, c"Hpc.expectString".as_ptr()) as *mut c_char;
    strcpy(res, &raw mut tmp as *mut c_char);

    return res;
}

unsafe fn expectWord64() -> StgWord64 {
    let mut tmp: StgWord64 = 0;

    while isdigit(tix_ch) != 0 {
        tmp = tmp
            .wrapping_mul(10 as StgWord64)
            .wrapping_add((tix_ch - '0' as i32) as StgWord64);
        tix_ch = getc(tixFile);
    }

    return tmp;
}

unsafe fn readTix() {
    let mut i: u32 = 0;
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
        tmpModule = stgMallocBytes(size_of::<HpcModuleInfo>() as usize, c"Hpc.readTix".as_ptr())
            as *mut HpcModuleInfo;

        (*tmpModule).from_file = true;
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
        (*tmpModule).hashNo = expectWord64() as u32 as StgWord32;
        ws();
        (*tmpModule).tickCount = expectWord64() as i32 as StgWord32;

        (*tmpModule).tixArr = stgCallocBytes(
            (*tmpModule).tickCount as usize,
            size_of::<StgWord64>() as usize,
            c"readTix".as_ptr(),
        ) as *mut StgWord64;

        ws();
        expect('[' as i32 as c_char);
        ws();
        i = 0;

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
            if DEBUG_RTS != 0 && RtsFlags.DebugFlags.hpc as i64 != 0 {
                trace_(
                    c"readTix: new HpcModuleInfo for %s".as_ptr(),
                    (*tmpModule).modName,
                );
            }

            insertStrHashTable(moduleHash, (*tmpModule).modName, tmpModule as *const c_void);
        } else {
            if DEBUG_RTS != 0 && RtsFlags.DebugFlags.hpc as i64 != 0 {
                trace_(
                    c"readTix: existing HpcModuleInfo for %s".as_ptr(),
                    (*tmpModule).modName,
                );
            }

            if (*tmpModule).hashNo != (*lookup).hashNo {
                fprintf(
                    __stderrp,
                    c"in module '%s'\n".as_ptr(),
                    (*tmpModule).modName,
                );
                failure(c"module mismatch with .tix/.mix file hash number".as_ptr());
            }

            i = 0;

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

    if hpc_inited != 0 {
        return;
    }

    hpc_inited = 1;
    hpc_pid = getpid();
    hpc_tixdir = getenv(c"HPCTIXDIR".as_ptr());
    hpc_tixfile = getenv(c"HPCTIXFILE".as_ptr());

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.hpc as i64 != 0 {
        trace_(c"startupHpc".as_ptr());
    }

    if !hpc_tixfile.is_null() {
        tixFilename = strdup(hpc_tixfile);
    } else if !hpc_tixdir.is_null() {
        mkdir(hpc_tixdir, 0o777);

        tixFilename = stgMallocBytes(
            strlen(hpc_tixdir)
                .wrapping_add(strlen(prog_name))
                .wrapping_add(12 as usize),
            c"Hpc.startupHpc".as_ptr(),
        ) as *mut c_char;

        sprintf(
            tixFilename,
            c"%s/%s-%d.tix".as_ptr(),
            hpc_tixdir,
            prog_name,
            hpc_pid as i32,
        );
    } else {
        tixFilename = stgMallocBytes(
            strlen(prog_name).wrapping_add(6 as usize),
            c"Hpc.startupHpc".as_ptr(),
        ) as *mut c_char;

        sprintf(tixFilename, c"%s.tix".as_ptr(), prog_name);
    }

    if RtsFlags.HpcFlags.readTixFile as u32 == HPC_YES_IMPLICIT as i32 as u32
        && init_open(__rts_fopen(tixFilename, c"r".as_ptr())) != 0
    {
        fprintf(
            __stderrp,
            c"Deprecation warning:\nI am reading in the existing tix file, and will add hpc info from this run to the existing data in that file.\nGHC 9.14 will cease looking for an existing tix file by default.\nIf you positively want to add hpc info to the current tix file, use the RTS option --read-tix-file=yes.\nMore information can be found in the accepted GHC proposal 612.\n"
                .as_ptr(),
        );

        readTix();
    } else if RtsFlags.HpcFlags.readTixFile as u32 == HPC_YES_EXPLICIT as i32 as u32
        && init_open(__rts_fopen(tixFilename, c"r".as_ptr())) != 0
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
    let mut i: u32 = 0;

    if moduleHash.is_null() {
        moduleHash = allocStrHashTable();
    }

    tmpModule = lookupStrHashTable(moduleHash, modName) as *mut HpcModuleInfo;

    if tmpModule.is_null() {
        tmpModule = stgMallocBytes(
            size_of::<HpcModuleInfo>() as usize,
            c"Hpc.hs_hpc_module".as_ptr(),
        ) as *mut HpcModuleInfo;

        (*tmpModule).modName = modName;
        (*tmpModule).tickCount = modCount;
        (*tmpModule).hashNo = modHashNo;
        (*tmpModule).tixArr = tixArr;
        i = 0;

        while i < modCount {
            *tixArr.offset(i as isize) = 0;
            i = i.wrapping_add(1);
        }

        (*tmpModule).next = modules as *mut _HpcModuleInfo;
        (*tmpModule).from_file = false;
        modules = tmpModule;
        insertStrHashTable(moduleHash, modName, tmpModule as *const c_void);
    } else {
        if (*tmpModule).tickCount != modCount {
            failure(c"inconsistent number of tick boxes".as_ptr());
        }

        if (*tmpModule).hashNo != modHashNo {
            fprintf(
                __stderrp,
                c"in module '%s'\n".as_ptr(),
                (*tmpModule).modName,
            );
            failure(c"module mismatch with .tix/.mix file hash number".as_ptr());
        }

        i = 0;

        while i < modCount {
            *tixArr.offset(i as isize) = *(*tmpModule).tixArr.offset(i as isize);
            i = i.wrapping_add(1);
        }

        if (*tmpModule).from_file {
            stgFree((*tmpModule).modName as *mut c_void);
            stgFree((*tmpModule).tixArr as *mut c_void);
        }

        (*tmpModule).from_file = false;
    };
}

unsafe fn writeTix(mut f: *mut FILE) {
    let mut tmpModule = null_mut::<HpcModuleInfo>();
    let mut i: u32 = 0;
    let mut inner_comma: u32 = 0;
    let mut outer_comma: u32 = 0;
    outer_comma = 0;

    if f.is_null() {
        return;
    }

    fprintf(f, c"Tix [".as_ptr());
    tmpModule = modules;

    while !tmpModule.is_null() {
        if outer_comma != 0 {
            fprintf(f, c",".as_ptr());
        } else {
            outer_comma = 1;
        }

        fprintf(
            f,
            c" TixModule \"%s\" %u %u [".as_ptr(),
            (*tmpModule).modName,
            (*tmpModule).hashNo,
            (*tmpModule).tickCount,
        );

        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.hpc as i64 != 0 {
            trace_(
                c"%s: %u (hash=%u)\n".as_ptr(),
                (*tmpModule).modName,
                (*tmpModule).tickCount,
                (*tmpModule).hashNo,
            );
        }

        inner_comma = 0;
        i = 0;

        while (i as StgWord32) < (*tmpModule).tickCount {
            if inner_comma != 0 {
                fprintf(f, c",".as_ptr());
            } else {
                inner_comma = 1;
            }

            if !(*tmpModule).tixArr.is_null() {
                fprintf(f, c"%llu".as_ptr(), *(*tmpModule).tixArr.offset(i as isize));
            } else {
                fprintf(f, c"0".as_ptr());
            }

            i = i.wrapping_add(1);
        }

        fprintf(f, c"]".as_ptr());
        tmpModule = (*tmpModule).next as *mut HpcModuleInfo;
    }

    fprintf(f, c"]\n".as_ptr());
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
    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.hpc as i64 != 0 {
        trace_(c"exitHpc".as_ptr());
    }

    if hpc_inited == 0 {
        return;
    }

    let mut is_subprocess = hpc_pid != getpid();

    if !is_subprocess && RtsFlags.HpcFlags.writeTixFile as i32 != 0 {
        let mut f = __rts_fopen(tixFilename, c"w+".as_ptr());
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
