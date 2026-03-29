use crate::ffi::rts::_assertFail;
use crate::linker::mach_o_types::{_Stub, MachORelocationInfo, MachOSection, Stub};
use crate::linker::macho::plt_aarch64::{makeStubAarch64, needStubForRelAarch64, stubSizeAarch64};
use crate::linker_internals::{ObjectCode, Section};
use crate::prelude::*;
use crate::rts_utils::{stgCallocBytes, stgFree};

unsafe fn numberOfStubsForSection(mut oc: *mut ObjectCode, mut sectionIndex: u32) -> u32 {
    let mut n = 0;
    let mut section: *mut MachOSection =
        (*(*oc).info).macho_sections.offset(sectionIndex as isize) as *mut MachOSection;

    let mut relocation_info =
        (*oc).image.offset((*section).reloff as isize) as *mut MachORelocationInfo;

    if (*section).size > 0 {
        let mut i: usize = 0;

        while i < (*section).nreloc as usize {
            if needStubForRelAarch64(relocation_info.offset(i as isize) as *mut MachORelocationInfo)
            {
                n = n.wrapping_add(1 as u32);
            }

            i = i.wrapping_add(1);
        }
    }

    return n;
}

unsafe fn findStub(mut section: *mut Section, mut addr: *mut *mut c_void, mut flags: u8) -> bool {
    let mut s = (*(*section).info).stubs;

    while !s.is_null() {
        if (*s).target == *addr && (*s).flags as i32 == flags as i32 {
            *addr = (*s).addr;

            return EXIT_SUCCESS != 0;
        }

        s = (*s).next as *mut Stub;
    }

    return EXIT_FAILURE != 0;
}

unsafe fn makeStub(mut section: *mut Section, mut addr: *mut *mut c_void, mut flags: u8) -> bool {
    let mut s = stgCallocBytes(size_of::<Stub>() as usize, 1, c"makeStub".as_ptr()) as *mut Stub;

    if !s.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/linker/macho/plt.c".as_ptr(), 55);
    }

    (*s).target = *addr;
    (*s).flags = flags;
    (*s).next = null_mut::<_Stub>();
    (*s).addr = ((*(*section).info).stub_offset as *mut u8)
        .offset(stubSizeAarch64.wrapping_mul((*(*section).info).nstubs) as isize)
        as *mut c_void;

    if Some(
        Some(makeStubAarch64 as unsafe extern "C" fn(*mut Stub) -> bool)
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(s)
    {
        return EXIT_FAILURE != 0;
    }

    if (*(*section).info).stubs.is_null() {
        if ((*(*section).info).nstubs == 0) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/linker/macho/plt.c".as_ptr(), 65);
        }

        (*(*section).info).stubs = s;
    } else {
        let mut tail = (*(*section).info).stubs;

        while !(*tail).next.is_null() {
            tail = (*tail).next as *mut Stub;
        }

        (*tail).next = s as *mut _Stub;
    }

    (*(*section).info).nstubs = (*(*section).info).nstubs.wrapping_add(1 as usize);
    *addr = (*s).addr;

    return EXIT_SUCCESS != 0;
}

unsafe fn freeStubs(mut section: *mut Section) {
    if section.is_null() || (*(*section).info).nstubs == 0 {
        return;
    }

    let mut last = (*(*section).info).stubs;

    while !(*last).next.is_null() {
        let mut t = last;
        last = (*last).next as *mut Stub;
        stgFree(t as *mut c_void);
    }

    (*(*section).info).stubs = null_mut::<Stub>();
    (*(*section).info).nstubs = 0;
}
