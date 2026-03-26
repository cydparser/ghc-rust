use crate::ffi::rts::_assertFail;
use crate::ffi::rts::messages::barf;
use crate::ffi::rts_api::getProgArgv;
use crate::get_env::{freeProgEnvv, getProgEnvv};
use crate::linker_internals::Section;
use crate::linker_internals::{Section, fini_t, init_t};
use crate::prelude::*;
use crate::rts_utils::{stgFree, stgMallocBytes};

pub(crate) type InitFiniKind = c_uint;

pub(crate) const INITFINI_FINI_ARRAY: InitFiniKind = 5;

pub(crate) const INITFINI_INIT_ARRAY: InitFiniKind = 4;

pub(crate) const INITFINI_DTORS: InitFiniKind = 3;

pub(crate) const INITFINI_CTORS: InitFiniKind = 2;

pub(crate) const INITFINI_FINI: InitFiniKind = 1;

pub(crate) const INITFINI_INIT: InitFiniKind = 0;

/// cbindgen:no-export
pub(crate) struct InitFiniList {
    pub(crate) section: *mut Section,
    pub(crate) priority: uint32_t,
    pub(crate) kind: InitFiniKind,
    pub(crate) next: *mut InitFiniList,
}

type SortOrder = c_uint;

const DECREASING: SortOrder = 1;

const INCREASING: SortOrder = 0;

unsafe fn addInitFini(
    mut head: *mut *mut InitFiniList,
    mut section: *mut Section,
    mut kind: InitFiniKind,
    mut priority: uint32_t,
) {
    let mut slist = stgMallocBytes(
        size_of::<InitFiniList>() as size_t,
        b"addInitFini\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut InitFiniList;

    (*slist).section = section;
    (*slist).kind = kind;
    (*slist).priority = priority;
    (*slist).next = *head;
    *head = slist;
}

unsafe fn sortInitFiniList(mut slist: *mut *mut InitFiniList, mut order: SortOrder) {
    let mut done = r#false != 0;

    while !done {
        let mut last = slist;
        done = r#true != 0;

        while !(*last).is_null() && !(**last).next.is_null() {
            let mut s0 = *last;
            let mut s1 = (*s0).next;
            let mut flip: bool = false;

            match order as c_uint {
                0 => {
                    flip = (*s0).priority > (*s1).priority;
                }
                1 => {
                    flip = (*s0).priority < (*s1).priority;
                }
                _ => {}
            }

            if flip {
                (*s0).next = (*s1).next;
                (*s1).next = s0;
                *last = s1;
                done = r#false != 0;
            } else {
                last = &raw mut (*s0).next;
            }
        }
    }
}

unsafe fn freeInitFiniList(mut slist: *mut InitFiniList) {
    while !slist.is_null() {
        let mut next = (*slist).next;
        stgFree(slist as *mut c_void);
        slist = next;
    }
}

unsafe fn runInitFini(mut head: *mut *mut InitFiniList) -> bool {
    let mut argc: c_int = 0;
    let mut envc: c_int = 0;
    let mut argv = null_mut::<*mut c_char>();
    let mut envv = null_mut::<*mut c_char>();
    getProgArgv(&raw mut argc, &raw mut argv);
    getProgEnvv(&raw mut envc, &raw mut envv);

    let mut slist = *head;

    while !slist.is_null() {
        let mut section = (*slist).section;

        match (*slist).kind as c_uint {
            0 => {
                let mut init = (*section).start as *mut init_t;
                (*init).expect("non-null function pointer")(argc, argv, envv);
            }
            1 => {
                let mut fini = (*section).start as *mut fini_t;
                (*fini).expect("non-null function pointer")();
            }
            2 => {
                let mut init_startC = (*section).start as *mut uint8_t;
                let mut init_start = init_startC as *mut init_t;
                let mut init_end = init_startC.offset((*section).size as isize) as *mut init_t;
                let mut init_0 = init_end.offset(-(1 as c_int as isize));

                while init_0 >= init_start {
                    if !(transmute::<init_t, intptr_t>(*init_0) == 0 as intptr_t
                        || transmute::<init_t, intptr_t>(*init_0) == -(1 as c_int) as intptr_t)
                    {
                        (*init_0).expect("non-null function pointer")(argc, argv, envv);
                    }

                    init_0 = init_0.offset(-1);
                }
            }
            3 => {
                let mut fini_startC = (*section).start as *mut c_char;
                let mut fini_start = fini_startC as *mut fini_t;
                let mut fini_end = fini_startC.offset((*section).size as isize) as *mut fini_t;
                let mut fini_0 = fini_start;

                while fini_0 < fini_end {
                    if !(transmute::<fini_t, intptr_t>(*fini_0) == 0 as intptr_t
                        || transmute::<fini_t, intptr_t>(*fini_0) == -(1 as c_int) as intptr_t)
                    {
                        (*fini_0).expect("non-null function pointer")();
                    }

                    fini_0 = fini_0.offset(1);
                }
            }
            4 => {
                let mut init_startC_0 = (*section).start as *mut c_char;
                let mut init_start_0 = init_startC_0 as *mut init_t;
                let mut init_end_0 = init_startC_0.offset((*section).size as isize) as *mut init_t;
                let mut init_1 = init_start_0;

                while init_1 < init_end_0 {
                    if (*init_1).is_some() as c_int as c_long != 0 {
                    } else {
                        _assertFail(
                            b"rts/linker/InitFini.c\0" as *const u8 as *const c_char,
                            159 as c_uint,
                        );
                    }

                    (*init_1).expect("non-null function pointer")(argc, argv, envv);
                    init_1 = init_1.offset(1);
                }
            }
            5 => {
                let mut fini_startC_0 = (*section).start as *mut c_char;
                let mut fini_start_0 = fini_startC_0 as *mut fini_t;
                let mut fini_end_0 = fini_startC_0.offset((*section).size as isize) as *mut fini_t;
                let mut fini_1 = fini_end_0.offset(-(1 as c_int as isize));

                while fini_1 >= fini_start_0 {
                    if (*fini_1).is_some() as c_int as c_long != 0 {
                    } else {
                        _assertFail(
                            b"rts/linker/InitFini.c\0" as *const u8 as *const c_char,
                            170 as c_uint,
                        );
                    }

                    (*fini_1).expect("non-null function pointer")();
                    fini_1 = fini_1.offset(-1);
                }
            }
            _ => {
                barf(b"unknown InitFiniKind\0" as *const u8 as *const c_char);
            }
        }

        slist = (*slist).next;
    }

    freeInitFiniList(*head);
    *head = null_mut::<InitFiniList>();
    freeProgEnvv(envc, envv as *mut *mut c_char);

    return r#true != 0;
}

unsafe fn runInit(mut head: *mut *mut InitFiniList) -> bool {
    sortInitFiniList(head, INCREASING);

    return runInitFini(head);
}

unsafe fn runFini(mut head: *mut *mut InitFiniList) -> bool {
    sortInitFiniList(head, DECREASING);

    return runInitFini(head);
}
