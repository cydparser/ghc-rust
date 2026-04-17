use crate::check_unload::{insertOCSectionIndices, loaded_objects};
use crate::ffi::rts::_assertFail;
use crate::ffi::rts::linker::pathchar;
use crate::ffi::rts::messages::{barf, debugBelch, errorBelch};
use crate::hs_ffi::HsInt;
use crate::linker::m_map::{mmapAnonForLinker, munmapForLinker};
use crate::linker::mach_o::ocInit_MachO;
use crate::linker_internals::{
    _ObjectCode, STATIC_OBJECT, isAlreadyLoaded, linker_mutex, loadOc, mkOc,
};
use crate::path_utils::{mkPath, pathdir, pathdup, pathsize};
use crate::prelude::*;
use crate::rts_flags::RtsFlags;
use crate::rts_utils::{stgFree, stgMallocBytes, stgReallocBytes};

#[cfg(test)]
mod tests;

const ThinArchive: ArchiveFormat = 1;

type ArchiveFormat = u32;

const FatArchive: ArchiveFormat = 2;

const StandardArchive: ArchiveFormat = 0;

const MachO64: ObjectFileFormat = 6;

type ObjectFileFormat = u32;

const MachO32: ObjectFileFormat = 5;

const ELF: ObjectFileFormat = 4;

const COFFAArch64: ObjectFileFormat = 3;

const COFFI386: ObjectFileFormat = 2;

const COFFAmd64: ObjectFileFormat = 1;

const NotObject: ObjectFileFormat = 0;

unsafe fn read4Bytes(mut buf: *const c_char) -> u32 {
    return if 0 != 0 {
        (*(buf as *mut u32) & 0xff000000) >> 24
            | (*(buf as *mut u32) & 0xff0000) >> 8
            | (*(buf as *mut u32) & 0xff00) << 8
            | (*(buf as *mut u32) & 0xff) << 24
    } else {
        _OSSwapInt32(*(buf as *mut u32))
    };
}

unsafe fn loadFatArchive(
    mut input: *mut c_char,
    mut f: *mut FILE,
    mut path: *mut pathchar,
) -> bool {
    let mut nfat_arch: u32 = 0;
    let mut nfat_offset: u32 = 0;
    let mut cputype: u32 = 0;
    let mut cpusubtype: u32 = 0;
    let mycputype: u32 = CPU_TYPE_ARM64 as u32;
    let mycpusubtype: u32 = CPU_SUBTYPE_ARM64_ALL as u32;
    nfat_arch = read4Bytes(input.offset(4) as *const c_char);

    if RtsFlags.DebugFlags.linker {
        debugBelch(
            c"loadArchive: found a fat archive containing %d architectures\n".as_ptr(),
            nfat_arch,
        );
    }

    let mut tmp: [c_char; 20] = [0; 20];
    nfat_offset = 0;

    let mut i: u32 = 0;

    while i < nfat_arch {
        let mut n = fread(&raw mut tmp as *mut c_char as *mut c_void, 1, 12, f) as i32;

        if n != 12 {
            errorBelch(c"Failed reading arch from `%s'".as_ptr(), path);

            return false;
        }

        cputype = read4Bytes(&raw mut tmp as *mut c_char as *const c_char);

        cpusubtype = read4Bytes((&raw mut tmp as *mut c_char).offset(4) as *const c_char);

        if cputype == mycputype && cpusubtype == mycpusubtype {
            if RtsFlags.DebugFlags.linker {
                debugBelch(c"loadArchive: found my archive in a fat archive\n".as_ptr());
            }

            nfat_offset = read4Bytes((&raw mut tmp as *mut c_char).offset(8) as *const c_char);

            break;
        } else {
            i = i.wrapping_add(1);
        }
    }

    if nfat_offset == 0 {
        errorBelch(
            c"Fat archive contains %d architectures, but none of them are compatible with the host"
                .as_ptr(),
            nfat_arch as i32,
        );

        return false;
    } else {
        let mut n_0 = fseek(f, nfat_offset as i64, SEEK_SET);

        if n_0 != 0 {
            errorBelch(c"Failed to seek to arch in `%s'".as_ptr(), path);

            return false;
        }

        let mut tmp_0: [c_char; 20] = [0; 20];
        n_0 = fread(&raw mut tmp_0 as *mut c_char as *mut c_void, 1, 8, f) as i32;

        if n_0 != 8 {
            errorBelch(c"Failed reading header from `%s'".as_ptr(), path);

            return false;
        }

        if strncmp(&raw mut tmp_0 as *mut c_char, c"!<arch>\n".as_ptr(), 8) != 0 {
            errorBelch(
                c"couldn't find archive in `%s'at offset %d".as_ptr(),
                path,
                nfat_offset,
            );

            return false;
        }
    }

    return true;
}

unsafe fn identifyObjectFile_(mut buf: *mut c_char, mut sz: usize) -> ObjectFileFormat {
    if sz > 2 && *(buf as *mut u16).offset(0) as i32 == 0x8664 {
        return COFFAmd64;
    }

    if sz > 2 && *(buf as *mut u16).offset(0) as i32 == 0x14c {
        return COFFI386;
    }

    if sz > 2 && *(buf as *mut u16).offset(0) as i32 == 0xaa64 {
        return COFFAArch64;
    }

    if sz > 4
        && memcmp(
            buf as *const c_void,
            c"\u{7f}ELF".as_ptr() as *const c_void,
            4,
        ) == 0
    {
        return ELF;
    }

    if sz > 4 && *(buf as *mut u32).offset(0) == 0xfeedface {
        return MachO32;
    }

    if sz > 4 && *(buf as *mut u32).offset(0) == 0xfeedfacf {
        return MachO64;
    }

    if sz > 8 && *(buf as *mut u64).offset(0) == 0x86640002ffff0000 {
        return COFFAmd64;
    }

    return NotObject;
}

unsafe fn identifyObjectFile(mut f: *mut FILE) -> ObjectFileFormat {
    let mut buf: [c_char; 32] = [0; 32];
    let mut sz: isize = fread(&raw mut buf as *mut c_char as *mut c_void, 1, 32, f) as isize;

    if (fseek(f, -(sz as i64), 1) == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/linker/LoadArchive.c".as_ptr(), 154);
    }

    return identifyObjectFile_(&raw mut buf as *mut c_char, sz as usize);
}

unsafe fn readThinArchiveMember(
    mut n: i32,
    mut memberSize: i32,
    mut path: *mut pathchar,
    mut fileName: *mut c_char,
    mut image: *mut c_char,
) -> bool {
    let mut has_succeeded = false;
    let mut member = null_mut::<FILE>();
    let mut pathCopy = null_mut::<pathchar>();
    let mut dirName = null_mut::<pathchar>();
    let mut memberPath = null_mut::<pathchar>();
    let mut objFileName = null_mut::<pathchar>();
    memberPath = null_mut::<pathchar>();
    pathCopy = pathdup(path);
    dirName = pathdir(pathCopy);

    let mut memberLen = strlen(dirName)
        .wrapping_add(1 as usize)
        .wrapping_add(strlen(fileName))
        .wrapping_add(1 as usize) as i32;

    memberPath = stgMallocBytes(
        pathsize.wrapping_mul(memberLen as usize),
        c"loadArchive(file)".as_ptr(),
    ) as *mut pathchar;

    objFileName = mkPath(fileName);

    snprintf(
        memberPath as *mut c_char,
        memberLen as usize,
        c"%s%s".as_ptr(),
        dirName,
        objFileName,
    );

    stgFree(objFileName as *mut c_void);
    stgFree(dirName as *mut c_void);
    member = fopen(memberPath, c"rb".as_ptr()) as *mut FILE;

    if member.is_null() {
        errorBelch(
            c"loadObj: can't read thin archive `%s'".as_ptr(),
            memberPath,
        );
    } else {
        n = fread(image as *mut c_void, 1, memberSize as usize, member) as i32;

        if n != memberSize {
            errorBelch(c"loadArchive: error whilst reading `%s'".as_ptr(), fileName);
        } else {
            has_succeeded = true;
        }
    }

    fclose(member);
    stgFree(memberPath as *mut c_void);
    stgFree(pathCopy as *mut c_void);

    return has_succeeded;
}

unsafe fn checkFatArchive(
    mut magic: *mut c_char,
    mut f: *mut FILE,
    mut path: *mut pathchar,
) -> bool {
    let mut success = false;

    if read4Bytes(magic as *const c_char) == FAT_MAGIC as u32 {
        success = loadFatArchive(magic as *mut c_char, f, path);
    } else {
        errorBelch(
            c"loadArchive: Neither an archive, nor a fat archive: `%s'".as_ptr(),
            path,
        );
    }

    return success;
}

unsafe fn lookupGNUArchiveIndex(
    mut gnuFileIndexSize: i32,
    mut fileName_: *mut *mut c_char,
    mut gnuFileIndex: *mut c_char,
    mut path: *mut pathchar,
    mut thisFileNameSize: *mut usize,
    mut fileNameSize: *mut usize,
) -> bool {
    let mut fileName = *fileName_;

    if isdigit(*fileName.offset(1) as i32) != 0 {
        if gnuFileIndex.is_null() {
            errorBelch(
                c"loadArchive: GNU-variant filename without an index while reading from `%s'"
                    .as_ptr(),
                path,
            );

            return false;
        }

        let mut n: i32 = 0;
        n = 2;

        while isdigit(*fileName.offset(n as isize) as i32) != 0 {
            n += 1;
        }

        let mut end = null_mut::<c_char>();
        *fileName.offset(n as isize) = '\0' as i32 as c_char;
        n = strtol(fileName.offset(1), &raw mut end, 10) as i32;

        if n < 0 || n > gnuFileIndexSize {
            errorBelch(
                c"loadArchive: GNU-variant filename offset %d out of range [0..%d] while reading filename from `%s'"
                    .as_ptr(),
                n,
                gnuFileIndexSize,
                path,
            );

            return false;
        }

        if n != 0 && !(*gnuFileIndex.offset((n - 1) as isize) as i32 == '\n' as i32) {
            errorBelch(
                c"loadArchive: GNU-variant filename offset %d invalid (range [0..%d]) while reading filename from `%s'"
                    .as_ptr(),
                n,
                gnuFileIndexSize,
                path,
            );

            return false;
        }

        let mut i: i32 = 0;
        i = n;

        while !(*gnuFileIndex.offset(i as isize) as i32 == '\n' as i32) {
            i += 1;
        }

        let mut FileNameSize: usize = (i - n) as usize;

        if FileNameSize >= *fileNameSize {
            *fileNameSize = FileNameSize.wrapping_mul(2 as usize);

            fileName = stgReallocBytes(
                fileName as *mut c_void,
                *fileNameSize,
                c"loadArchive(fileName)".as_ptr(),
            ) as *mut c_char;

            *fileName_ = fileName;
        }

        memcpy(
            fileName as *mut c_void,
            gnuFileIndex.offset(n as isize) as *const c_void,
            FileNameSize,
        );

        if *fileName.offset(FileNameSize.wrapping_sub(1 as usize) as isize) as i32 == '/' as i32 {
            FileNameSize = FileNameSize.wrapping_sub(1);
        }

        *fileName.offset(FileNameSize as isize) = '\0' as i32 as c_char;
        *thisFileNameSize = FileNameSize;
    } else if 0 == strncmp(fileName.offset(1), c"               ".as_ptr(), 15)
        || 0 == strncmp(fileName.offset(1), c"SYM64/         ".as_ptr(), 15)
    {
        *fileName.offset(0) = '\0' as i32 as c_char;
        *thisFileNameSize = 0;
    } else {
        errorBelch(
            c"loadArchive: invalid GNU-variant filename `%.16s' while reading filename from `%s'"
                .as_ptr(),
            fileName,
            path,
        );

        return false;
    }

    return true;
}

unsafe fn identifyArchiveFormat(
    mut f: *mut FILE,
    mut path: *mut pathchar,
    mut out: *mut ArchiveFormat,
) -> bool {
    let mut tmp: [c_char; 8] = [0; 8];
    let mut n = fread(&raw mut tmp as *mut c_char as *mut c_void, 1, 8, f) as usize;

    if n != 8 {
        errorBelch(
            c"loadArchive: Failed reading header from `%s'".as_ptr(),
            path,
        );

        return false;
    }

    if strncmp(&raw mut tmp as *mut c_char, c"!<arch>\n".as_ptr(), 8) == 0 {
        *out = StandardArchive;

        return true;
    } else if strncmp(&raw mut tmp as *mut c_char, c"!<thin>\n".as_ptr(), 8) == 0 {
        *out = ThinArchive;

        return true;
    } else {
        let mut success = checkFatArchive(&raw mut tmp as *mut c_char, f, path);

        if !success {
            return false;
        }

        *out = FatArchive;

        return true;
    };
}

unsafe fn loadArchive_(mut path: *mut pathchar) -> HsInt {
    let mut archive_fmt: ArchiveFormat = StandardArchive;
    let mut isThin: bool = false;
    let mut current_block: u64;
    let mut image = null_mut::<c_char>();
    let mut retcode: HsInt = 0;
    let mut memberIdx = 0;
    let mut f = null_mut::<FILE>();
    let mut thisFileNameSize: usize = -1 as usize;
    let mut misalignment = 0;

    if RtsFlags.DebugFlags.linker {
        debugBelch(c"loadArchive: start\n".as_ptr());
    }

    if RtsFlags.DebugFlags.linker {
        debugBelch(c"loadArchive: Loading archive `%s'\n".as_ptr(), path);
    }

    if isAlreadyLoaded(path) != 0 {
        if RtsFlags.DebugFlags.linker {
            debugBelch(c"ignoring repeated load of %s\n".as_ptr(), path);
        }

        return 1;
    }

    let mut gnuFileIndex = null_mut::<c_char>();
    let mut gnuFileIndexSize = 0;
    let mut fileNameSize: usize = 32;
    let mut fileName =
        stgMallocBytes(fileNameSize, c"loadArchive(fileName)".as_ptr()) as *mut c_char;
    f = fopen(path, c"rb".as_ptr()) as *mut FILE;

    if f.is_null() {
        errorBelch(c"loadArchive: loadObj: can't read `%s'".as_ptr(), path);
    } else {
        archive_fmt = StandardArchive;

        if !identifyArchiveFormat(f, path, &raw mut archive_fmt) {
            errorBelch(
                c"loadArchive: failed to identify archive format of %s.".as_ptr(),
                path,
            );
        } else {
            isThin = archive_fmt as u32 == ThinArchive as i32 as u32;

            if RtsFlags.DebugFlags.linker {
                debugBelch(c"loadArchive: loading archive contents\n".as_ptr());
            }

            loop {
                let mut n: usize = 0;

                if RtsFlags.DebugFlags.linker {
                    debugBelch(c"loadArchive: reading at %ld\n".as_ptr(), ftell(f));
                }

                n = fread(fileName as *mut c_void, 1, 16, f) as usize;

                if n != 16 {
                    if feof(f) != 0 {
                        if RtsFlags.DebugFlags.linker {
                            debugBelch(
                                c"loadArchive: EOF while reading from '%s'\n".as_ptr(),
                                path,
                            );
                        }

                        current_block = 14723615986260991866;
                        break;
                    } else {
                        errorBelch(
                            c"loadArchive: Failed reading file name from `%s'".as_ptr(),
                            path,
                        );

                        current_block = 8840649376158258189;
                        break;
                    }
                } else if strncmp(fileName, c"!<arch>\n".as_ptr(), 8) == 0 {
                    if RtsFlags.DebugFlags.linker {
                        debugBelch(
                            c"loadArchive: found the start of another archive, breaking\n".as_ptr(),
                        );
                    }

                    current_block = 14723615986260991866;
                    break;
                } else {
                    let mut tmp: [c_char; 32] = [0; 32];
                    n = fread(&raw mut tmp as *mut c_char as *mut c_void, 1, 12, f) as usize;

                    if n != 12 {
                        errorBelch(
                            c"loadArchive: Failed reading mod time from `%s'".as_ptr(),
                            path,
                        );

                        current_block = 8840649376158258189;
                        break;
                    } else {
                        n = fread(&raw mut tmp as *mut c_char as *mut c_void, 1, 6, f) as usize;

                        if n != 6 {
                            errorBelch(
                                c"loadArchive: Failed reading owner from `%s'".as_ptr(),
                                path,
                            );

                            current_block = 8840649376158258189;
                            break;
                        } else {
                            n = fread(&raw mut tmp as *mut c_char as *mut c_void, 1, 6, f) as usize;

                            if n != 6 {
                                errorBelch(
                                    c"loadArchive: Failed reading group from `%s'".as_ptr(),
                                    path,
                                );

                                current_block = 8840649376158258189;
                                break;
                            } else {
                                n = fread(&raw mut tmp as *mut c_char as *mut c_void, 1, 8, f)
                                    as usize;

                                if n != 8 {
                                    errorBelch(
                                        c"loadArchive: Failed reading mode from `%s'".as_ptr(),
                                        path,
                                    );

                                    current_block = 8840649376158258189;
                                    break;
                                } else {
                                    n = fread(&raw mut tmp as *mut c_char as *mut c_void, 1, 10, f)
                                        as usize;

                                    if n != 10 {
                                        errorBelch(
                                            c"loadArchive: Failed reading size from `%s'".as_ptr(),
                                            path,
                                        );

                                        current_block = 8840649376158258189;
                                        break;
                                    } else {
                                        tmp[10] = '\0' as i32 as c_char;
                                        n = 0;

                                        while isdigit(tmp[n as usize] as i32) != 0 {
                                            n = n.wrapping_add(1);
                                        }

                                        tmp[n as usize] = '\0' as i32 as c_char;

                                        let mut memberSize: usize = 0;
                                        let mut end = null_mut::<c_char>();

                                        memberSize =
                                            strtol(&raw mut tmp as *mut c_char, &raw mut end, 10)
                                                as usize;

                                        if &raw mut tmp as *mut c_char == end {
                                            errorBelch(
                                                c"loadArchive: Failed to decode member size"
                                                    .as_ptr(),
                                            );

                                            current_block = 8840649376158258189;
                                            break;
                                        } else {
                                            if RtsFlags.DebugFlags.linker {
                                                debugBelch(
                                                    c"loadArchive: size of this archive member is %zd\n"
                                                        .as_ptr(),
                                                    memberSize,
                                                );
                                            }

                                            n = fread(
                                                &raw mut tmp as *mut c_char as *mut c_void,
                                                1,
                                                2,
                                                f,
                                            )
                                                as usize;

                                            if n != 2 {
                                                errorBelch(
                                                    c"loadArchive: Failed reading magic from `%s'"
                                                        .as_ptr(),
                                                    path,
                                                );

                                                current_block = 8840649376158258189;
                                                break;
                                            } else if strncmp(
                                                &raw mut tmp as *mut c_char,
                                                c"`\n".as_ptr(),
                                                2,
                                            ) != 0
                                            {
                                                errorBelch(
                                                    c"loadArchive: Failed reading magic from `%s' at %ld. Got %c%c"
                                                        .as_ptr(),
                                                    path,
                                                    ftell(f),
                                                    tmp[0] as i32,
                                                    tmp[1] as i32,
                                                );

                                                current_block = 8840649376158258189;
                                                break;
                                            } else {
                                                let mut isGnuIndex = false;

                                                if 0 == strncmp(fileName, c"#1/".as_ptr(), 3) {
                                                    let mut n_0: usize = 0;
                                                    *fileName.offset(16) = '\0' as i32 as c_char;

                                                    if isdigit(*fileName.offset(3) as i32) != 0 {
                                                        n_0 = 4;

                                                        while isdigit(
                                                            *fileName.offset(n_0 as isize) as i32,
                                                        ) != 0
                                                        {
                                                            n_0 = n_0.wrapping_add(1);
                                                        }

                                                        *fileName.offset(n_0 as isize) =
                                                            '\0' as i32 as c_char;
                                                        thisFileNameSize =
                                                            atoi(fileName.offset(3)) as usize;
                                                        memberSize = memberSize
                                                            .wrapping_sub(thisFileNameSize);

                                                        if thisFileNameSize >= fileNameSize {
                                                            fileNameSize = thisFileNameSize
                                                                .wrapping_mul(2 as usize);

                                                            fileName = stgReallocBytes(
                                                                fileName as *mut c_void,
                                                                fileNameSize,
                                                                c"loadArchive(fileName)".as_ptr(),
                                                            )
                                                                as *mut c_char;
                                                        }

                                                        n_0 = fread(
                                                            fileName as *mut c_void,
                                                            1,
                                                            thisFileNameSize,
                                                            f,
                                                        )
                                                            as usize;

                                                        if n_0 != thisFileNameSize {
                                                            errorBelch(
                                                                c"Failed reading filename from `%s'".as_ptr(),
                                                                path,
                                                            );

                                                            current_block = 8840649376158258189;
                                                            break;
                                                        } else {
                                                            *fileName.offset(
                                                                thisFileNameSize as isize,
                                                            ) = 0;
                                                            thisFileNameSize = strlen(fileName);
                                                        }
                                                    } else {
                                                        errorBelch(
                                                            c"BSD-variant filename size not found while reading filename from `%s'"
                                                                .as_ptr(),
                                                            path,
                                                        );

                                                        current_block = 8840649376158258189;
                                                        break;
                                                    }
                                                } else if 0 == strncmp(fileName, c"//".as_ptr(), 2)
                                                {
                                                    *fileName.offset(0) = '\0' as i32 as c_char;
                                                    thisFileNameSize = 0;
                                                    isGnuIndex = true;
                                                } else if *fileName.offset(0) as i32 == '/' as i32 {
                                                    if !lookupGNUArchiveIndex(
                                                        gnuFileIndexSize,
                                                        &raw mut fileName,
                                                        gnuFileIndex,
                                                        path,
                                                        &raw mut thisFileNameSize,
                                                        &raw mut fileNameSize,
                                                    ) {
                                                        current_block = 8840649376158258189;
                                                        break;
                                                    }
                                                } else {
                                                    thisFileNameSize = 0;

                                                    while thisFileNameSize < 16 {
                                                        if *fileName
                                                            .offset(thisFileNameSize as isize)
                                                            as i32
                                                            == '/' as i32
                                                        {
                                                            *fileName.offset(
                                                                thisFileNameSize as isize,
                                                            ) = '\0' as i32 as c_char;
                                                            break;
                                                        } else {
                                                            thisFileNameSize =
                                                                thisFileNameSize.wrapping_add(1);
                                                        }
                                                    }

                                                    if thisFileNameSize == 16 {
                                                        thisFileNameSize = 0;

                                                        while thisFileNameSize < 16 {
                                                            if *fileName
                                                                .offset(thisFileNameSize as isize)
                                                                as i32
                                                                == ' ' as i32
                                                            {
                                                                *fileName.offset(
                                                                    thisFileNameSize as isize,
                                                                ) = '\0' as i32 as c_char;
                                                                break;
                                                            } else {
                                                                thisFileNameSize = thisFileNameSize
                                                                    .wrapping_add(1);
                                                            }
                                                        }
                                                    }
                                                }

                                                if RtsFlags.DebugFlags.linker {
                                                    debugBelch(
                                                        c"loadArchive: Found member file `%s'\n"
                                                            .as_ptr(),
                                                        fileName,
                                                    );
                                                }

                                                let mut is_symbol_table =
                                                    strcmp(c"".as_ptr(), fileName) == 0;

                                                let mut object_fmt = (if is_symbol_table as i32 != 0
                                                {
                                                    NotObject as i32 as u32
                                                } else {
                                                    identifyObjectFile(f) as u32
                                                })
                                                    as ObjectFileFormat;

                                                let mut isImportLib = false;

                                                if RtsFlags.DebugFlags.linker {
                                                    debugBelch(
                                                        c"loadArchive: \tthisFileNameSize = %d\n"
                                                            .as_ptr(),
                                                        thisFileNameSize as i32,
                                                    );
                                                }

                                                if RtsFlags.DebugFlags.linker {
                                                    debugBelch(
                                                        c"loadArchive: \tisObject = %d\n".as_ptr(),
                                                        object_fmt as u32,
                                                    );
                                                }

                                                if !is_symbol_table && isThin as i32 != 0
                                                    || object_fmt as u32 != NotObject as i32 as u32
                                                {
                                                    if RtsFlags.DebugFlags.linker {
                                                        debugBelch(
                                                            c"loadArchive: Member is an object file...loading...\n"
                                                                .as_ptr(),
                                                        );
                                                    }

                                                    image = mmapAnonForLinker(memberSize)
                                                        as *mut c_char;

                                                    if isThin {
                                                        if !readThinArchiveMember(
                                                            n as i32,
                                                            memberSize as i32,
                                                            path,
                                                            fileName,
                                                            image,
                                                        ) {
                                                            current_block = 8840649376158258189;
                                                            break;
                                                        }
                                                    } else {
                                                        let mut n_1 = fread(
                                                            image as *mut c_void,
                                                            1,
                                                            memberSize,
                                                            f,
                                                        )
                                                            as usize;

                                                        if n_1 != memberSize {
                                                            errorBelch(
                                                                c"loadArchive: error whilst reading `%s'".as_ptr(),
                                                                path,
                                                            );

                                                            current_block = 8840649376158258189;
                                                            break;
                                                        }
                                                    }

                                                    let mut size = snprintf(
                                                        null_mut::<c_char>(),
                                                        0,
                                                        c"%s(#%d:%.*s)".as_ptr(),
                                                        path,
                                                        memberIdx,
                                                        thisFileNameSize as i32,
                                                        fileName,
                                                    );

                                                    let mut archiveMemberName = stgMallocBytes(
                                                        ((size + 1 as i32 + 1 as i32) as usize)
                                                            .wrapping_mul(
                                                                size_of::<pathchar>() as usize
                                                            ),
                                                        c"loadArchive(file)".as_ptr(),
                                                    )
                                                        as *mut pathchar;

                                                    snprintf(
                                                        archiveMemberName as *mut c_char,
                                                        (size + 1) as usize,
                                                        c"%s(#%d:%.*s)".as_ptr(),
                                                        path,
                                                        memberIdx,
                                                        thisFileNameSize as i32,
                                                        fileName,
                                                    );

                                                    let mut oc = mkOc(
                                                        STATIC_OBJECT,
                                                        path,
                                                        image,
                                                        memberSize as i32,
                                                        false,
                                                        archiveMemberName,
                                                        misalignment,
                                                    );

                                                    if (object_fmt as u32 == MachO32 as i32 as u32
                                                        || object_fmt as u32
                                                            == MachO64 as i32 as u32)
                                                        as i32
                                                        as i64
                                                        != 0
                                                    {
                                                    } else {
                                                        _assertFail(
                                                            c"rts/linker/LoadArchive.c".as_ptr(),
                                                            616,
                                                        );
                                                    }

                                                    ocInit_MachO(oc);
                                                    stgFree(archiveMemberName as *mut c_void);

                                                    if 0 == loadOc(oc) {
                                                        stgFree(fileName as *mut c_void);
                                                        fclose(f);

                                                        return 0;
                                                    } else {
                                                        insertOCSectionIndices(oc);
                                                        (*oc).next_loaded_object =
                                                            loaded_objects as *mut _ObjectCode;
                                                        loaded_objects = oc;
                                                    }
                                                } else if isGnuIndex {
                                                    if !gnuFileIndex.is_null() {
                                                        errorBelch(
                                                            c"loadArchive: GNU-variant index found, but already have an index, while reading filename from `%s'"
                                                                .as_ptr(),
                                                            path,
                                                        );

                                                        current_block = 8840649376158258189;
                                                        break;
                                                    } else {
                                                        if RtsFlags.DebugFlags.linker {
                                                            debugBelch(
                                                                c"loadArchive: Found GNU-variant file index\n".as_ptr(),
                                                            );
                                                        }

                                                        gnuFileIndex = mmapAnonForLinker(
                                                            memberSize.wrapping_add(1 as usize),
                                                        )
                                                            as *mut c_char;

                                                        n = fread(
                                                            gnuFileIndex as *mut c_void,
                                                            1,
                                                            memberSize,
                                                            f,
                                                        )
                                                            as usize;

                                                        if n != memberSize {
                                                            errorBelch(
                                                                c"loadArchive: error whilst reading `%s'".as_ptr(),
                                                                path,
                                                            );

                                                            current_block = 8840649376158258189;
                                                            break;
                                                        } else {
                                                            *gnuFileIndex
                                                                .offset(memberSize as isize) =
                                                                '/' as i32 as c_char;
                                                            gnuFileIndexSize = memberSize as i32;
                                                        }
                                                    }
                                                } else if !isImportLib {
                                                    if RtsFlags.DebugFlags.linker {
                                                        debugBelch(
                                                            c"loadArchive: `%s' does not appear to be an object file\n"
                                                                .as_ptr(),
                                                            fileName,
                                                        );
                                                    }

                                                    if !isThin || thisFileNameSize == 0 {
                                                        n = fseek(f, memberSize as i64, SEEK_CUR)
                                                            as usize;

                                                        if n != 0 {
                                                            errorBelch(
                                                                c"loadArchive: error whilst seeking by %zd in `%s'"
                                                                    .as_ptr(),
                                                                memberSize,
                                                                path,
                                                            );

                                                            current_block = 8840649376158258189;
                                                            break;
                                                        }
                                                    }
                                                }

                                                if !(isThin as i32 != 0 && thisFileNameSize > 0)
                                                    && memberSize.wrapping_rem(2 as usize) != 0
                                                {
                                                    if RtsFlags.DebugFlags.linker {
                                                        debugBelch(
                                                            c"loadArchive: trying to read one pad byte\n".as_ptr(),
                                                        );
                                                    }

                                                    n = fread(
                                                        &raw mut tmp as *mut c_char as *mut c_void,
                                                        1,
                                                        1,
                                                        f,
                                                    )
                                                        as usize;

                                                    if n != 1 {
                                                        if feof(f) != 0 {
                                                            if RtsFlags.DebugFlags.linker {
                                                                debugBelch(
                                                                    c"loadArchive: found EOF while reading one pad byte\n"
                                                                        .as_ptr(),
                                                                );
                                                            }

                                                            current_block = 14723615986260991866;
                                                            break;
                                                        } else {
                                                            errorBelch(
                                                                c"loadArchive: Failed reading padding from `%s'".as_ptr(),
                                                                path,
                                                            );

                                                            current_block = 8840649376158258189;
                                                            break;
                                                        }
                                                    } else if RtsFlags.DebugFlags.linker {
                                                        debugBelch(
                                                            c"loadArchive: successfully read one pad byte\n".as_ptr(),
                                                        );
                                                    }
                                                }

                                                memberIdx += 1;

                                                if RtsFlags.DebugFlags.linker {
                                                    debugBelch(
                                                        c"loadArchive: reached end of archive loading while loop\n"
                                                            .as_ptr(),
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            match current_block {
                8840649376158258189 => {}
                _ => {
                    retcode = 1;
                }
            }
        }
    }

    if !f.is_null() {
        fclose(f);
    }

    if !fileName.is_null() {
        stgFree(fileName as *mut c_void);
    }

    if !gnuFileIndex.is_null() {
        munmapForLinker(
            gnuFileIndex as *mut c_void,
            (gnuFileIndexSize + 1) as usize,
            c"loadArchive_".as_ptr(),
        );
    }

    if RtsFlags.DebugFlags.linker {
        debugBelch(c"loadArchive: done\n".as_ptr());
    }

    return retcode;
}

#[ffi(compiler, ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn loadArchive(mut path: *mut pathchar) -> HsInt {
    let mut __r = pthread_mutex_lock(&raw mut linker_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/linker/LoadArchive.c".as_ptr(),
            720,
            __r,
        );
    }

    let mut r = loadArchive_(path);

    if pthread_mutex_unlock(&raw mut linker_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/linker/LoadArchive.c".as_ptr(),
            722,
        );
    }

    return r;
}

unsafe fn isArchive(mut path: *mut pathchar) -> bool {
    static mut ARCHIVE_HEADER: [c_char; 9] =
        unsafe { transmute::<[u8; 9], [c_char; 9]>(*b"!<arch>\n\0") };

    let mut buffer: [c_char; 10] = [0; 10];
    let mut f = fopen(path, c"rb".as_ptr()) as *mut FILE;

    if f.is_null() {
        return false;
    }

    let mut ret = fread(
        &raw mut buffer as *mut c_char as *mut c_void,
        1,
        size_of::<[c_char; 10]>() as usize,
        f,
    ) as usize;

    fclose(f);

    if ret < size_of::<[c_char; 10]>() as usize {
        return false;
    }

    return strncmp(
        &raw const ARCHIVE_HEADER as *const c_char,
        &raw mut buffer as *mut c_char,
        (size_of::<[c_char; 9]>() as usize).wrapping_sub(1 as usize),
    ) == 0;
}
