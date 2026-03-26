use crate::check_unload::{insertOCSectionIndices, loaded_objects};
use crate::ffi::hs_ffi::HsInt;
use crate::ffi::rts::_assertFail;
use crate::ffi::rts::linker::pathchar;
use crate::ffi::rts::messages::errorBelch;
use crate::linker::m_map::{mmapAnonForLinker, munmapForLinker};
use crate::linker::mach_o::ocInit_MachO;
use crate::linker_internals::{_ObjectCode, STATIC_OBJECT, isAlreadyLoaded, loadOc, mkOc};
use crate::path_utils::{mkPath, pathdir, pathdup, pathsize};
use crate::prelude::*;
use crate::rts_utils::{stgFree, stgMallocBytes, stgReallocBytes};

#[cfg(test)]
mod tests;

const ThinArchive: ArchiveFormat = 1;

type ArchiveFormat = c_uint;

const FatArchive: ArchiveFormat = 2;

const StandardArchive: ArchiveFormat = 0;

const MachO64: ObjectFileFormat = 6;

type ObjectFileFormat = c_uint;

const MachO32: ObjectFileFormat = 5;

const ELF: ObjectFileFormat = 4;

const COFFAArch64: ObjectFileFormat = 3;

const COFFI386: ObjectFileFormat = 2;

const COFFAmd64: ObjectFileFormat = 1;

const NotObject: ObjectFileFormat = 0;

unsafe fn read4Bytes(mut buf: *const c_char) -> uint32_t {
    return if 0 != 0 {
        (*(buf as *mut uint32_t) & 0xff000000 as __uint32_t) >> 24 as c_int
            | (*(buf as *mut uint32_t) & 0xff0000 as __uint32_t) >> 8 as c_int
            | (*(buf as *mut uint32_t) & 0xff00 as __uint32_t) << 8 as c_int
            | (*(buf as *mut uint32_t) & 0xff as __uint32_t) << 24 as c_int
    } else {
        _OSSwapInt32(*(buf as *mut uint32_t))
    };
}

unsafe fn loadFatArchive(
    mut input: *mut c_char,
    mut f: *mut FILE,
    mut path: *mut pathchar,
) -> bool {
    let mut nfat_arch: uint32_t = 0;
    let mut nfat_offset: uint32_t = 0;
    let mut cputype: uint32_t = 0;
    let mut cpusubtype: uint32_t = 0;
    let mycputype: uint32_t = CPU_TYPE_ARM64 as uint32_t;
    let mycpusubtype: uint32_t = CPU_SUBTYPE_ARM64_ALL as uint32_t;
    nfat_arch = read4Bytes(input.offset(4 as c_int as isize) as *const c_char);

    let mut tmp: [c_char; 20] = [0; 20];
    nfat_offset = 0 as uint32_t;

    let mut i: uint32_t = 0 as uint32_t;

    while i < nfat_arch {
        let mut n = fread(
            &raw mut tmp as *mut c_char as *mut c_void,
            1 as size_t,
            12 as size_t,
            f,
        ) as c_int;

        if n != 12 as c_int {
            errorBelch(
                b"Failed reading arch from `%s'\0" as *const u8 as *const c_char,
                path,
            );

            return r#false != 0;
        }

        cputype = read4Bytes(&raw mut tmp as *mut c_char as *const c_char);
        cpusubtype =
            read4Bytes((&raw mut tmp as *mut c_char).offset(4 as c_int as isize) as *const c_char);

        if cputype == mycputype && cpusubtype == mycpusubtype {
            nfat_offset = read4Bytes(
                (&raw mut tmp as *mut c_char).offset(8 as c_int as isize) as *const c_char
            );

            break;
        } else {
            i = i.wrapping_add(1);
        }
    }

    if nfat_offset == 0 as uint32_t {
        errorBelch(
            b"Fat archive contains %d architectures, but none of them are compatible with the host\0"
                as *const u8 as *const c_char,
            nfat_arch as c_int,
        );

        return r#false != 0;
    } else {
        let mut n_0 = fseek(f, nfat_offset as c_long, SEEK_SET);

        if n_0 != 0 as c_int {
            errorBelch(
                b"Failed to seek to arch in `%s'\0" as *const u8 as *const c_char,
                path,
            );

            return r#false != 0;
        }

        let mut tmp_0: [c_char; 20] = [0; 20];

        n_0 = fread(
            &raw mut tmp_0 as *mut c_char as *mut c_void,
            1 as size_t,
            8 as size_t,
            f,
        ) as c_int;

        if n_0 != 8 as c_int {
            errorBelch(
                b"Failed reading header from `%s'\0" as *const u8 as *const c_char,
                path,
            );

            return r#false != 0;
        }

        if strncmp(
            &raw mut tmp_0 as *mut c_char,
            b"!<arch>\n\0" as *const u8 as *const c_char,
            8 as size_t,
        ) != 0 as c_int
        {
            errorBelch(
                b"couldn't find archive in `%s'at offset %d\0" as *const u8 as *const c_char,
                path,
                nfat_offset,
            );

            return r#false != 0;
        }
    }

    return r#true != 0;
}

unsafe fn identifyObjectFile_(mut buf: *mut c_char, mut sz: size_t) -> ObjectFileFormat {
    if sz > 2 as size_t
        && *(buf as *mut uint16_t).offset(0 as c_int as isize) as c_int == 0x8664 as c_int
    {
        return COFFAmd64;
    }

    if sz > 2 as size_t
        && *(buf as *mut uint16_t).offset(0 as c_int as isize) as c_int == 0x14c as c_int
    {
        return COFFI386;
    }

    if sz > 2 as size_t
        && *(buf as *mut uint16_t).offset(0 as c_int as isize) as c_int == 0xaa64 as c_int
    {
        return COFFAArch64;
    }

    if sz > 4 as size_t
        && memcmp(
            buf as *const c_void,
            b"\x7FELF\0" as *const u8 as *const c_char as *const c_void,
            4 as size_t,
        ) == 0 as c_int
    {
        return ELF;
    }

    if sz > 4 as size_t
        && *(buf as *mut uint32_t).offset(0 as c_int as isize) == 0xfeedface as uint32_t
    {
        return MachO32;
    }

    if sz > 4 as size_t
        && *(buf as *mut uint32_t).offset(0 as c_int as isize) == 0xfeedfacf as uint32_t
    {
        return MachO64;
    }

    if sz > 8 as size_t
        && *(buf as *mut uint64_t).offset(0 as c_int as isize) == 0x86640002ffff0000 as uint64_t
    {
        return COFFAmd64;
    }

    return NotObject;
}

unsafe fn identifyObjectFile(mut f: *mut FILE) -> ObjectFileFormat {
    let mut buf: [c_char; 32] = [0; 32];

    let mut sz: ssize_t = fread(
        &raw mut buf as *mut c_char as *mut c_void,
        1 as size_t,
        32 as size_t,
        f,
    ) as ssize_t;

    if (fseek(f, -(sz as c_long), 1 as c_int) == 0 as c_int) as c_int as c_long != 0 {
    } else {
        _assertFail(
            b"rts/linker/LoadArchive.c\0" as *const u8 as *const c_char,
            154 as c_uint,
        );
    }

    return identifyObjectFile_(&raw mut buf as *mut c_char, sz as size_t);
}

unsafe fn readThinArchiveMember(
    mut n: c_int,
    mut memberSize: c_int,
    mut path: *mut pathchar,
    mut fileName: *mut c_char,
    mut image: *mut c_char,
) -> bool {
    let mut has_succeeded = r#false != 0;
    let mut member = null_mut::<FILE>();
    let mut pathCopy = null_mut::<pathchar>();
    let mut dirName = null_mut::<pathchar>();
    let mut memberPath = null_mut::<pathchar>();
    let mut objFileName = null_mut::<pathchar>();
    memberPath = null_mut::<pathchar>();
    pathCopy = pathdup(path);
    dirName = pathdir(pathCopy);

    let mut memberLen = strlen(dirName)
        .wrapping_add(1 as size_t)
        .wrapping_add(strlen(fileName))
        .wrapping_add(1 as size_t) as c_int;

    memberPath = stgMallocBytes(
        pathsize.wrapping_mul(memberLen as size_t),
        b"loadArchive(file)\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut pathchar;

    objFileName = mkPath(fileName);

    snprintf(
        memberPath as *mut c_char,
        memberLen as size_t,
        b"%s%s\0" as *const u8 as *const c_char,
        dirName,
        objFileName,
    );

    stgFree(objFileName as *mut c_void);
    stgFree(dirName as *mut c_void);
    member = fopen(memberPath, b"rb\0" as *const u8 as *const c_char) as *mut FILE;

    if member.is_null() {
        errorBelch(
            b"loadObj: can't read thin archive `%s'\0" as *const u8 as *const c_char,
            memberPath,
        );
    } else {
        n = fread(
            image as *mut c_void,
            1 as size_t,
            memberSize as size_t,
            member,
        ) as c_int;

        if n != memberSize {
            errorBelch(
                b"loadArchive: error whilst reading `%s'\0" as *const u8 as *const c_char,
                fileName,
            );
        } else {
            has_succeeded = r#true != 0;
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
    let mut success = r#false != 0;

    if read4Bytes(magic as *const c_char) == FAT_MAGIC as uint32_t {
        success = loadFatArchive(magic as *mut c_char, f, path);
    } else {
        errorBelch(
            b"loadArchive: Neither an archive, nor a fat archive: `%s'\0" as *const u8
                as *const c_char,
            path,
        );
    }

    return success;
}

unsafe fn lookupGNUArchiveIndex(
    mut gnuFileIndexSize: c_int,
    mut fileName_: *mut *mut c_char,
    mut gnuFileIndex: *mut c_char,
    mut path: *mut pathchar,
    mut thisFileNameSize: *mut size_t,
    mut fileNameSize: *mut size_t,
) -> bool {
    let mut fileName = *fileName_;

    if isdigit(*fileName.offset(1 as c_int as isize) as c_int) != 0 {
        if gnuFileIndex.is_null() {
            errorBelch(
                b"loadArchive: GNU-variant filename without an index while reading from `%s'\0"
                    as *const u8 as *const c_char,
                path,
            );

            return r#false != 0;
        }

        let mut n: c_int = 0;
        n = 2 as c_int;

        while isdigit(*fileName.offset(n as isize) as c_int) != 0 {
            n += 1;
        }

        let mut end = null_mut::<c_char>();
        *fileName.offset(n as isize) = '\0' as i32 as c_char;

        n = strtol(
            fileName.offset(1 as c_int as isize),
            &raw mut end,
            10 as c_int,
        ) as c_int;

        if n < 0 as c_int || n > gnuFileIndexSize {
            errorBelch(
                b"loadArchive: GNU-variant filename offset %d out of range [0..%d] while reading filename from `%s'\0"
                    as *const u8 as *const c_char,
                n,
                gnuFileIndexSize,
                path,
            );

            return r#false != 0;
        }

        if n != 0 as c_int
            && !(*gnuFileIndex.offset((n - 1 as c_int) as isize) as c_int == '\n' as i32)
        {
            errorBelch(
                b"loadArchive: GNU-variant filename offset %d invalid (range [0..%d]) while reading filename from `%s'\0"
                    as *const u8 as *const c_char,
                n,
                gnuFileIndexSize,
                path,
            );

            return r#false != 0;
        }

        let mut i: c_int = 0;
        i = n;

        while !(*gnuFileIndex.offset(i as isize) as c_int == '\n' as i32) {
            i += 1;
        }

        let mut FileNameSize: size_t = (i - n) as size_t;

        if FileNameSize >= *fileNameSize {
            *fileNameSize = FileNameSize.wrapping_mul(2 as size_t);

            fileName = stgReallocBytes(
                fileName as *mut c_void,
                *fileNameSize,
                b"loadArchive(fileName)\0" as *const u8 as *const c_char as *mut c_char,
            ) as *mut c_char;

            *fileName_ = fileName;
        }

        memcpy(
            fileName as *mut c_void,
            gnuFileIndex.offset(n as isize) as *const c_void,
            FileNameSize,
        );

        if *fileName.offset(FileNameSize.wrapping_sub(1 as size_t) as isize) as c_int == '/' as i32
        {
            FileNameSize = FileNameSize.wrapping_sub(1);
        }

        *fileName.offset(FileNameSize as isize) = '\0' as i32 as c_char;
        *thisFileNameSize = FileNameSize;
    } else if 0 as c_int
        == strncmp(
            fileName.offset(1 as c_int as isize),
            b"               \0" as *const u8 as *const c_char,
            15 as size_t,
        )
        || 0 as c_int
            == strncmp(
                fileName.offset(1 as c_int as isize),
                b"SYM64/         \0" as *const u8 as *const c_char,
                15 as size_t,
            )
    {
        *fileName.offset(0 as c_int as isize) = '\0' as i32 as c_char;
        *thisFileNameSize = 0 as size_t;
    } else {
        errorBelch(
            b"loadArchive: invalid GNU-variant filename `%.16s' while reading filename from `%s'\0"
                as *const u8 as *const c_char,
            fileName,
            path,
        );

        return r#false != 0;
    }

    return r#true != 0;
}

unsafe fn identifyArchiveFormat(
    mut f: *mut FILE,
    mut path: *mut pathchar,
    mut out: *mut ArchiveFormat,
) -> bool {
    let mut tmp: [c_char; 8] = [0; 8];

    let mut n = fread(
        &raw mut tmp as *mut c_char as *mut c_void,
        1 as size_t,
        8 as size_t,
        f,
    ) as size_t;

    if n != 8 as size_t {
        errorBelch(
            b"loadArchive: Failed reading header from `%s'\0" as *const u8 as *const c_char,
            path,
        );

        return r#false != 0;
    }

    if strncmp(
        &raw mut tmp as *mut c_char,
        b"!<arch>\n\0" as *const u8 as *const c_char,
        8 as size_t,
    ) == 0 as c_int
    {
        *out = StandardArchive;

        return r#true != 0;
    } else if strncmp(
        &raw mut tmp as *mut c_char,
        b"!<thin>\n\0" as *const u8 as *const c_char,
        8 as size_t,
    ) == 0 as c_int
    {
        *out = ThinArchive;

        return r#true != 0;
    } else {
        let mut success = checkFatArchive(&raw mut tmp as *mut c_char, f, path);

        if !success {
            return r#false != 0;
        }

        *out = FatArchive;

        return r#true != 0;
    };
}

unsafe fn loadArchive_(mut path: *mut pathchar) -> HsInt {
    let mut archive_fmt: ArchiveFormat = StandardArchive;
    let mut isThin: bool = false;
    let mut current_block: u64;
    let mut image = null_mut::<c_char>();
    let mut retcode: HsInt = 0 as HsInt;
    let mut memberIdx = 0 as c_int;
    let mut f = null_mut::<FILE>();
    let mut thisFileNameSize: size_t = -(1 as c_int) as size_t;
    let mut misalignment = 0 as c_int;

    if isAlreadyLoaded(path) != 0 {
        return 1 as HsInt;
    }

    let mut gnuFileIndex = null_mut::<c_char>();
    let mut gnuFileIndexSize = 0 as c_int;
    let mut fileNameSize: size_t = 32 as size_t;

    let mut fileName = stgMallocBytes(
        fileNameSize,
        b"loadArchive(fileName)\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut c_char;

    f = fopen(path, b"rb\0" as *const u8 as *const c_char) as *mut FILE;

    if f.is_null() {
        errorBelch(
            b"loadArchive: loadObj: can't read `%s'\0" as *const u8 as *const c_char,
            path,
        );
    } else {
        archive_fmt = StandardArchive;

        if !identifyArchiveFormat(f, path, &raw mut archive_fmt) {
            errorBelch(
                b"loadArchive: failed to identify archive format of %s.\0" as *const u8
                    as *const c_char,
                path,
            );
        } else {
            isThin = archive_fmt as c_uint == ThinArchive as c_int as c_uint;

            loop {
                let mut n: size_t = 0;
                n = fread(fileName as *mut c_void, 1 as size_t, 16 as size_t, f) as size_t;

                if n != 16 as size_t {
                    if feof(f) != 0 {
                        current_block = 1691841359054504885;
                        break;
                    }

                    errorBelch(
                        b"loadArchive: Failed reading file name from `%s'\0" as *const u8
                            as *const c_char,
                        path,
                    );

                    current_block = 15851283293817637459;
                    break;
                } else {
                    if strncmp(
                        fileName,
                        b"!<arch>\n\0" as *const u8 as *const c_char,
                        8 as size_t,
                    ) == 0 as c_int
                    {
                        current_block = 1691841359054504885;
                        break;
                    }

                    let mut tmp: [c_char; 32] = [0; 32];

                    n = fread(
                        &raw mut tmp as *mut c_char as *mut c_void,
                        1 as size_t,
                        12 as size_t,
                        f,
                    ) as size_t;

                    if n != 12 as size_t {
                        errorBelch(
                            b"loadArchive: Failed reading mod time from `%s'\0" as *const u8
                                as *const c_char,
                            path,
                        );

                        current_block = 15851283293817637459;
                        break;
                    } else {
                        n = fread(
                            &raw mut tmp as *mut c_char as *mut c_void,
                            1 as size_t,
                            6 as size_t,
                            f,
                        ) as size_t;

                        if n != 6 as size_t {
                            errorBelch(
                                b"loadArchive: Failed reading owner from `%s'\0" as *const u8
                                    as *const c_char,
                                path,
                            );

                            current_block = 15851283293817637459;
                            break;
                        } else {
                            n = fread(
                                &raw mut tmp as *mut c_char as *mut c_void,
                                1 as size_t,
                                6 as size_t,
                                f,
                            ) as size_t;

                            if n != 6 as size_t {
                                errorBelch(
                                    b"loadArchive: Failed reading group from `%s'\0" as *const u8
                                        as *const c_char,
                                    path,
                                );

                                current_block = 15851283293817637459;
                                break;
                            } else {
                                n = fread(
                                    &raw mut tmp as *mut c_char as *mut c_void,
                                    1 as size_t,
                                    8 as size_t,
                                    f,
                                ) as size_t;

                                if n != 8 as size_t {
                                    errorBelch(
                                        b"loadArchive: Failed reading mode from `%s'\0" as *const u8
                                            as *const c_char,
                                        path,
                                    );

                                    current_block = 15851283293817637459;
                                    break;
                                } else {
                                    n = fread(
                                        &raw mut tmp as *mut c_char as *mut c_void,
                                        1 as size_t,
                                        10 as size_t,
                                        f,
                                    ) as size_t;

                                    if n != 10 as size_t {
                                        errorBelch(
                                            b"loadArchive: Failed reading size from `%s'\0"
                                                as *const u8
                                                as *const c_char,
                                            path,
                                        );

                                        current_block = 15851283293817637459;
                                        break;
                                    } else {
                                        tmp[10 as c_int as usize] = '\0' as i32 as c_char;
                                        n = 0 as size_t;

                                        while isdigit(tmp[n as usize] as c_int) != 0 {
                                            n = n.wrapping_add(1);
                                        }

                                        tmp[n as usize] = '\0' as i32 as c_char;

                                        let mut memberSize: size_t = 0;
                                        let mut end = null_mut::<c_char>();

                                        memberSize = strtol(
                                            &raw mut tmp as *mut c_char,
                                            &raw mut end,
                                            10 as c_int,
                                        )
                                            as size_t;

                                        if &raw mut tmp as *mut c_char == end {
                                            errorBelch(
                                                b"loadArchive: Failed to decode member size\0"
                                                    as *const u8
                                                    as *const c_char,
                                            );

                                            current_block = 15851283293817637459;
                                            break;
                                        } else {
                                            n = fread(
                                                &raw mut tmp as *mut c_char as *mut c_void,
                                                1 as size_t,
                                                2 as size_t,
                                                f,
                                            )
                                                as size_t;

                                            if n != 2 as size_t {
                                                errorBelch(
                                                    b"loadArchive: Failed reading magic from `%s'\0"
                                                        as *const u8
                                                        as *const c_char,
                                                    path,
                                                );

                                                current_block = 15851283293817637459;
                                                break;
                                            } else if strncmp(
                                                &raw mut tmp as *mut c_char,
                                                b"`\n\0" as *const u8 as *const c_char,
                                                2 as size_t,
                                            ) != 0 as c_int
                                            {
                                                errorBelch(
                                                    b"loadArchive: Failed reading magic from `%s' at %ld. Got %c%c\0"
                                                        as *const u8 as *const c_char,
                                                    path,
                                                    ftell(f),
                                                    tmp[0 as c_int as usize] as c_int,
                                                    tmp[1 as c_int as usize] as c_int,
                                                );

                                                current_block = 15851283293817637459;
                                                break;
                                            } else {
                                                let mut isGnuIndex = r#false != 0;

                                                if 0 as c_int
                                                    == strncmp(
                                                        fileName,
                                                        b"#1/\0" as *const u8 as *const c_char,
                                                        3 as size_t,
                                                    )
                                                {
                                                    let mut n_0: size_t = 0 as size_t;
                                                    *fileName.offset(16 as c_int as isize) =
                                                        '\0' as i32 as c_char;

                                                    if isdigit(
                                                        *fileName.offset(3 as c_int as isize)
                                                            as c_int,
                                                    ) != 0
                                                    {
                                                        n_0 = 4 as size_t;

                                                        while isdigit(
                                                            *fileName.offset(n_0 as isize) as c_int,
                                                        ) != 0
                                                        {
                                                            n_0 = n_0.wrapping_add(1);
                                                        }

                                                        *fileName.offset(n_0 as isize) =
                                                            '\0' as i32 as c_char;

                                                        thisFileNameSize = atoi(
                                                            fileName.offset(3 as c_int as isize),
                                                        )
                                                            as size_t;
                                                        memberSize = memberSize
                                                            .wrapping_sub(thisFileNameSize);

                                                        if thisFileNameSize >= fileNameSize {
                                                            fileNameSize = thisFileNameSize
                                                                .wrapping_mul(2 as size_t);

                                                            fileName = stgReallocBytes(
                                                                fileName as *mut c_void,
                                                                fileNameSize,
                                                                b"loadArchive(fileName)\0"
                                                                    as *const u8
                                                                    as *const c_char
                                                                    as *mut c_char,
                                                            )
                                                                as *mut c_char;
                                                        }

                                                        n_0 = fread(
                                                            fileName as *mut c_void,
                                                            1 as size_t,
                                                            thisFileNameSize,
                                                            f,
                                                        )
                                                            as size_t;

                                                        if n_0 != thisFileNameSize {
                                                            errorBelch(
                                                                b"Failed reading filename from `%s'\0" as *const u8
                                                                    as *const c_char,
                                                                path,
                                                            );

                                                            current_block = 15851283293817637459;
                                                            break;
                                                        } else {
                                                            *fileName.offset(
                                                                thisFileNameSize as isize,
                                                            ) = 0 as c_char;

                                                            thisFileNameSize = strlen(fileName);
                                                        }
                                                    } else {
                                                        errorBelch(
                                                            b"BSD-variant filename size not found while reading filename from `%s'\0"
                                                                as *const u8 as *const c_char,
                                                            path,
                                                        );

                                                        current_block = 15851283293817637459;
                                                        break;
                                                    }
                                                } else if 0 as c_int
                                                    == strncmp(
                                                        fileName,
                                                        b"//\0" as *const u8 as *const c_char,
                                                        2 as size_t,
                                                    )
                                                {
                                                    *fileName.offset(0 as c_int as isize) =
                                                        '\0' as i32 as c_char;
                                                    thisFileNameSize = 0 as size_t;
                                                    isGnuIndex = r#true != 0;
                                                } else if *fileName.offset(0 as c_int as isize)
                                                    as c_int
                                                    == '/' as i32
                                                {
                                                    if !lookupGNUArchiveIndex(
                                                        gnuFileIndexSize,
                                                        &raw mut fileName,
                                                        gnuFileIndex,
                                                        path,
                                                        &raw mut thisFileNameSize,
                                                        &raw mut fileNameSize,
                                                    ) {
                                                        current_block = 15851283293817637459;
                                                        break;
                                                    }
                                                } else {
                                                    thisFileNameSize = 0 as size_t;

                                                    while thisFileNameSize < 16 as size_t {
                                                        if *fileName
                                                            .offset(thisFileNameSize as isize)
                                                            as c_int
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

                                                    if thisFileNameSize == 16 as size_t {
                                                        thisFileNameSize = 0 as size_t;

                                                        while thisFileNameSize < 16 as size_t {
                                                            if *fileName
                                                                .offset(thisFileNameSize as isize)
                                                                as c_int
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

                                                let mut is_symbol_table = strcmp(
                                                    b"\0" as *const u8 as *const c_char,
                                                    fileName,
                                                ) == 0 as c_int;

                                                let mut object_fmt =
                                                    (if is_symbol_table as c_int != 0 {
                                                        NotObject as c_int as c_uint
                                                    } else {
                                                        identifyObjectFile(f) as c_uint
                                                    })
                                                        as ObjectFileFormat;

                                                let mut isImportLib = r#false != 0;

                                                if !is_symbol_table && isThin as c_int != 0
                                                    || object_fmt as c_uint
                                                        != NotObject as c_int as c_uint
                                                {
                                                    image = mmapAnonForLinker(memberSize)
                                                        as *mut c_char;

                                                    if isThin {
                                                        if !readThinArchiveMember(
                                                            n as c_int,
                                                            memberSize as c_int,
                                                            path,
                                                            fileName,
                                                            image,
                                                        ) {
                                                            current_block = 15851283293817637459;
                                                            break;
                                                        }
                                                    } else {
                                                        let mut n_1 = fread(
                                                            image as *mut c_void,
                                                            1 as size_t,
                                                            memberSize,
                                                            f,
                                                        )
                                                            as size_t;

                                                        if n_1 != memberSize {
                                                            errorBelch(
                                                                b"loadArchive: error whilst reading `%s'\0" as *const u8
                                                                    as *const c_char,
                                                                path,
                                                            );

                                                            current_block = 15851283293817637459;
                                                            break;
                                                        }
                                                    }

                                                    let mut size = snprintf(
                                                        null_mut::<c_char>(),
                                                        0 as size_t,
                                                        b"%s(#%d:%.*s)\0" as *const u8
                                                            as *const c_char,
                                                        path,
                                                        memberIdx,
                                                        thisFileNameSize as c_int,
                                                        fileName,
                                                    );

                                                    let mut archiveMemberName = stgMallocBytes(
                                                        ((size + 1 as c_int + 1 as c_int)
                                                            as size_t)
                                                            .wrapping_mul(
                                                                size_of::<pathchar>() as size_t
                                                            ),
                                                        b"loadArchive(file)\0" as *const u8
                                                            as *const c_char
                                                            as *mut c_char,
                                                    )
                                                        as *mut pathchar;

                                                    snprintf(
                                                        archiveMemberName as *mut c_char,
                                                        (size + 1 as c_int) as size_t,
                                                        b"%s(#%d:%.*s)\0" as *const u8
                                                            as *const c_char,
                                                        path,
                                                        memberIdx,
                                                        thisFileNameSize as c_int,
                                                        fileName,
                                                    );

                                                    let mut oc = mkOc(
                                                        STATIC_OBJECT,
                                                        path,
                                                        image,
                                                        memberSize as c_int,
                                                        r#false != 0,
                                                        archiveMemberName,
                                                        misalignment,
                                                    );

                                                    ocInit_MachO(oc);
                                                    stgFree(archiveMemberName as *mut c_void);

                                                    if 0 as HsInt == loadOc(oc) {
                                                        stgFree(fileName as *mut c_void);
                                                        fclose(f);

                                                        return 0 as HsInt;
                                                    } else {
                                                        insertOCSectionIndices(oc);
                                                        (*oc).next_loaded_object =
                                                            loaded_objects as *mut _ObjectCode;
                                                        loaded_objects = oc;
                                                    }
                                                } else if isGnuIndex {
                                                    if !gnuFileIndex.is_null() {
                                                        errorBelch(
                                                            b"loadArchive: GNU-variant index found, but already have an index, while reading filename from `%s'\0"
                                                                as *const u8 as *const c_char,
                                                            path,
                                                        );

                                                        current_block = 15851283293817637459;
                                                        break;
                                                    } else {
                                                        gnuFileIndex = mmapAnonForLinker(
                                                            memberSize.wrapping_add(1 as size_t),
                                                        )
                                                            as *mut c_char;

                                                        n = fread(
                                                            gnuFileIndex as *mut c_void,
                                                            1 as size_t,
                                                            memberSize,
                                                            f,
                                                        )
                                                            as size_t;

                                                        if n != memberSize {
                                                            errorBelch(
                                                                b"loadArchive: error whilst reading `%s'\0" as *const u8
                                                                    as *const c_char,
                                                                path,
                                                            );

                                                            current_block = 15851283293817637459;
                                                            break;
                                                        } else {
                                                            *gnuFileIndex
                                                                .offset(memberSize as isize) =
                                                                '/' as i32 as c_char;
                                                            gnuFileIndexSize = memberSize as c_int;
                                                        }
                                                    }
                                                } else if !isImportLib {
                                                    if !isThin || thisFileNameSize == 0 as size_t {
                                                        n = fseek(f, memberSize as c_long, SEEK_CUR)
                                                            as size_t;

                                                        if n != 0 as size_t {
                                                            errorBelch(
                                                                b"loadArchive: error whilst seeking by %zd in `%s'\0"
                                                                    as *const u8 as *const c_char,
                                                                memberSize,
                                                                path,
                                                            );

                                                            current_block = 15851283293817637459;
                                                            break;
                                                        }
                                                    }
                                                }

                                                if !(isThin as c_int != 0
                                                    && thisFileNameSize > 0 as size_t)
                                                    && memberSize.wrapping_rem(2 as size_t) != 0
                                                {
                                                    n = fread(
                                                        &raw mut tmp as *mut c_char as *mut c_void,
                                                        1 as size_t,
                                                        1 as size_t,
                                                        f,
                                                    )
                                                        as size_t;

                                                    if n != 1 as size_t {
                                                        if feof(f) != 0 {
                                                            current_block = 1691841359054504885;
                                                            break;
                                                        }

                                                        errorBelch(
                                                            b"loadArchive: Failed reading padding from `%s'\0"
                                                                as *const u8 as *const c_char,
                                                            path,
                                                        );

                                                        current_block = 15851283293817637459;
                                                        break;
                                                    }
                                                }

                                                memberIdx += 1;
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
                15851283293817637459 => {}
                _ => {
                    retcode = 1 as HsInt;
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
            (gnuFileIndexSize + 1 as c_int) as size_t,
            b"loadArchive_\0" as *const u8 as *const c_char,
        );
    }

    return retcode;
}

#[ffi(compiler, ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn loadArchive(mut path: *mut pathchar) -> HsInt {
    let mut r = loadArchive_(path);

    return r;
}

unsafe fn isArchive(mut path: *mut pathchar) -> bool {
    static mut ARCHIVE_HEADER: [c_char; 9] =
        unsafe { transmute::<[u8; 9], [c_char; 9]>(*b"!<arch>\n\0") };

    let mut buffer: [c_char; 10] = [0; 10];
    let mut f = fopen(path, b"rb\0" as *const u8 as *const c_char) as *mut FILE;

    if f.is_null() {
        return r#false != 0;
    }

    let mut ret = fread(
        &raw mut buffer as *mut c_char as *mut c_void,
        1 as size_t,
        size_of::<[c_char; 10]>() as size_t,
        f,
    ) as size_t;

    fclose(f);

    if ret < size_of::<[c_char; 10]>() as usize {
        return r#false != 0;
    }

    return strncmp(
        &raw const ARCHIVE_HEADER as *const c_char,
        &raw mut buffer as *mut c_char,
        (size_of::<[c_char; 9]>() as size_t).wrapping_sub(1 as size_t),
    ) == 0 as c_int;
}
